use panduza_platform_core::{
    log_debug_mount_end, log_debug_mount_start, log_info, Container, Error, Instance,
};

use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::watch;
use tokio::sync::Mutex;
use tokio_serial::SerialStream;
///
pub async fn mount(mut instance: Instance, port: SerialStream) -> Result<(), Error> {
    //
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
    let tx_handle = tokio::spawn(async move {
        loop {
            att_serial_stream_tx.wait_for_commands().await;
            while let Some(command) = att_serial_stream_tx.pop().await {
                log_info!(
                    att_serial_stream_tx.logger(),
                    "Command received from client - {:?}",
                    command
                );
                let mut port_guard_tx = port_clone_tx.lock().await;
                if let Err(e) = port_guard_tx.write_all(&command).await {
                    return Err(format!("Failed to send the command to the device: {}", e));
                }
                log_info!(
                    att_serial_stream_tx.logger(),
                    "Command sent via TCP to the device"
                );
            }
        }
    });

    instance
        .monitor_task("serial_stream/TX".to_string(), tx_handle)
        .await;

    // Response receiver RX
    let port_clone_rx = port_shared.clone();
    let rx_handle = tokio::spawn(async move {
        let mut buffer = [0u8; 1024];
        let mut port_guard_rx = port_clone_rx.lock().await;
        loop {
            match port_guard_rx.read(&mut buffer).await {
                Ok(0) => {
                    return Err("Connection closed by the device".to_string());
                }
                Ok(n) => {
                    let data = bytes::Bytes::copy_from_slice(&buffer[..n]);
                    if let Err(e) = att_serial_stream_rx.set(data).await {
                        return Err(format!("Failed to set data in att_serial_stream_rx: {}", e));
                    }
                    log_info!(
                        att_serial_stream_rx.logger(),
                        "Response sent to client via att_serial_stream_rx - {:?}",
                        String::from_utf8_lossy(&buffer[..n])
                    );
                }
                Err(e) => {
                    return Err(format!("Read error: {}", e));
                }
            }
        }
    });

    instance
        .monitor_task("serial_stream/RX".to_string(), rx_handle)
        .await;

    //
    //

    log_debug_mount_end!(class.logger());
    Ok(())
}
