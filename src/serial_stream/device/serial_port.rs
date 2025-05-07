use panduza_platform_core::{
    log_debug_mount_end, log_debug_mount_start, log_info, Container, Error, Instance,
};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_serial::SerialStream;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::watch;
///
pub async fn mount(
    mut instance: Instance,
    port :SerialStream
) -> Result<(), Error> {
    //
    //  using watch to sync the shutdown of reader and wrtier in case of reset
    let (reset_tx, reset_rx) = watch::channel(false);

    // Create interface
    let mut class = instance.create_class("serial-stream").finish().await;
    log_debug_mount_start!(class.logger());
    //
    //
    let att_serial_stream_rx = class
        .create_attribute("RX")
        .with_ro()
        .with_info(r#"read command"#)
        .start_as_bytes()
        .await?;

    //
    //
    let mut att_serial_stream_tx = class
        .create_attribute("TX")
        .with_wo()
        .with_info(r#"write command"#)
        .start_as_bytes()
        .await?;

    // Creating the shared serial port
    let port_shared = Arc::new(Mutex::new(port));
    
    //
    // Command writer TX
    let port_clone_tx = port_shared.clone();
    let mut reset_rx_clone = reset_rx.clone();
    let tx_handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = reset_rx_clone.changed() => {
                    log_info!(att_serial_stream_tx.logger(),"Reset triggered: stopping writer task");   
                    break
                },

                _ = att_serial_stream_tx.wait_for_commands() => {
                    while let Some(command) = att_serial_stream_tx.pop().await {
                        log_info!(att_serial_stream_tx.logger(), "Command received from client - {:?}", command);
                        let mut port_guard_tx = port_clone_tx.lock().await;
                        let _ = port_guard_tx.write_all(&command).await;
                        log_info!(
                            att_serial_stream_tx.logger(),
                            "Command sent via TCP to the device"
                        );
                    }
                }
            }
        }
    });

    // Response receiver RX
    let port_clone_rx = port_shared.clone();
    let rx_handle =tokio::spawn(async move {
        let mut buffer = [0u8; 1024];
        let mut port_guard_rx = port_clone_rx.lock().await;
        loop {
            match port_guard_rx.read(&mut buffer).await {
                Ok(0) => {
                    log_info!(att_serial_stream_rx.logger(), "Connection closed by the device");
                    let _ = reset_tx.send(true);
                    log_info!(
                        att_serial_stream_rx.logger(),
                        "Reset triggered: stopping reader task"
                    );
                    break;
                }
                Ok(n) => {
                    let data = bytes::Bytes::copy_from_slice(&buffer[..n]);
                    let _ = att_serial_stream_rx.set(data).await;
                    log_info!(
                        att_serial_stream_rx.logger(),
                        "Response sent to client via att_serial_stream_rx - {:?}",
                        String::from_utf8_lossy(&buffer[..n])
                    );
                }
                Err(e) => {
                    log_info!(att_serial_stream_rx.logger(), "Read error: {}", e);
                    let _ = reset_tx.send(true);
                    log_info!(
                        att_serial_stream_rx.logger(),
                        "Reset triggered: stopping reader task"
                    );
                    break;
                }
            }
        }
    });

    // Reboot 
    let mut reset_rx_clone = reset_rx.clone();
    tokio::spawn(async move {
        reset_rx_clone.changed().await.unwrap();
        log_info!(instance.logger(),"Reboot triggered — waiting for tasks to end");
        let _ = tx_handle.await;
        let _ = rx_handle.await;
        log_info!(instance.logger(),"Old tasks finished. Rebooting port...");
        drop(port_shared);
        instance.go_error().await;
    });

    //
    //

    log_debug_mount_end!(class.logger());
    Ok(())
}
