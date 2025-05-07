use panduza_platform_core::{
    log_debug_mount_end, log_debug_mount_start, log_info, Container, Error, Instance,
};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::watch;
///
pub async fn mount(
    mut instance: Instance,
    mut reader: OwnedReadHalf,
    mut writer: OwnedWriteHalf,
) -> Result<(), Error> {
    //

    //  using watch to sync the shutdown of reader and wrtier in case of reset
    let (reset_tx, mut reset_rx) = watch::channel(false);

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

    //
    // Command writer TX
    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = reset_rx.changed() => {
                    log_info!(att_serial_stream_tx.logger(),"Reset triggered: stopping writer task");
                    break
                },

                _ = att_serial_stream_tx.wait_for_commands() => {
                    while let Some(command) = att_serial_stream_tx.pop().await {
                        log_info!(att_serial_stream_tx.logger(), "Command received from client - {:?}", command);
                        let _ = writer.write_all(&command).await;
                        log_info!(
                            att_serial_stream_tx.logger(),
                            "Command sent via TCP to the device - {:?}",
                            command
                        );
                    }
                }
            }
        }
    });

    // Response receiver RX
    tokio::spawn(async move {
        let mut buffer = [0u8; 1024];
        loop {
            match reader.read(&mut buffer).await {
                Ok(0) => {
                    log_info!(att_serial_stream_rx.logger(), "Connection closed by the device");
                    let _ = reset_tx.send(true);
                    log_info!(
                        att_serial_stream_rx.logger(),
                        "Reset triggered: stopping reader task"
                    );
                    instance.go_error().await;
                    break;
                }
                Ok(n) => {
                    let data = bytes::Bytes::copy_from_slice(&buffer[..n]);
                    let _ = att_serial_stream_rx.set(data).await;
                    log_info!(
                        att_serial_stream_rx.logger(),
                        "Response sent to client via att_serial_stream_rx"
                    );
                }
                Err(e) => {
                    log_info!(att_serial_stream_rx.logger(), "Read error: {}", e);
                    let _ = reset_tx.send(true);
                    log_info!(
                        att_serial_stream_rx.logger(),
                        "Reset triggered: stopping reader task"
                    );
                    instance.go_error().await;
                    break;
                }
            }
        }
    });

    //
    //

    log_debug_mount_end!(class.logger());
    Ok(())
}
