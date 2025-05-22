use panduza_platform_core::{
    log_debug_mount_end, log_debug_mount_start, log_trace, Container, Error, Instance,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

///
pub async fn mount(
    mut instance: Instance,
    mut reader: OwnedReadHalf,
    mut writer: OwnedWriteHalf,
) -> Result<(), Error> {
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

    //
    // Command writer TX
    let tx_handle = tokio::spawn(async move {
        loop {
            att_serial_stream_tx.wait_for_commands().await;
            while let Some(command) = att_serial_stream_tx.pop().await {
                log_trace!(
                    att_serial_stream_tx.logger(),
                    "Command received from client - {:?}",
                    command
                );
                if let Err(e) = writer.write_all(&command).await {
                    return Err(format!("Failed to send the command to the device: {}", e));
                }
                log_trace!(
                    att_serial_stream_tx.logger(),
                    "Command sent via TCP to the device - {:?}",
                    command
                );
            }
        }
    });

    instance
        .monitor_task("serial_stream/TX".to_string(), tx_handle)
        .await;

    // Response receiver RX
    let rx_handle = tokio::spawn(async move {
        let mut buffer = [0u8; 1024];
        loop {
            match reader.read(&mut buffer).await {
                Ok(0) => {
                    return Err("Connection closed by the device".to_string());
                }
                Ok(n) => {
                    let data = bytes::Bytes::copy_from_slice(&buffer[..n]);
                    let data_log = data.clone();
                    if let Err(e) = att_serial_stream_rx.set(data).await {
                        return Err(format!("Failed to set data in att_serial_stream_rx: {}", e));
                    }
                    log_trace!(
                        att_serial_stream_rx.logger(),
                        "Response sent to client via att_serial_stream_rx - {:?}",
                        data_log
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
