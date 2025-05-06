mod bytes;
use async_trait::async_trait;
use panduza_platform_core::Error::DriverError;
use panduza_platform_core::{
    log_error, log_info, log_info_mount_end, log_info_mount_start, Actions, Container, Error,
    Instance, Logger,
};
//tcp
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::sleep;

#[derive(Default)]
///
/// Device to control Zybo Board
///
///
pub struct Device {}

impl Device {
    /// Constructor
    ///
    pub fn new() -> Self {
        Device {}
    }
}

#[async_trait]
impl Actions for Device {
    async fn mount(&mut self, mut instance: Instance) -> Result<(), Error> {
        // Start logging
        let logger = instance.logger().clone();
        log_info_mount_start!(logger);

        // Catching the IP and the Port in the settings
        match instance.settings().await {
            Some(settings) => {
                let ip = settings
                    .get("ip")
                    .expect("no ip found in the tree.json")
                    .as_str()
                    .unwrap();
                let port = settings
                    .get("port")
                    .expect("no port found in the tree.json")
                    .as_str()
                    .unwrap();

                let transport = settings
                    .get("transport")
                    .expect("no transport found in the tree.json")
                    .as_str()
                    .unwrap();

                let message_on_connect = settings
                    .get("message_on_connect")
                    .expect("no message_on_connect found in the tree.json")
                    .as_bool()
                    .unwrap();

                log_info!(logger, "[ SETTINGS CHOSEN ]");
                log_info!(logger, "ip : {}", ip);
                log_info!(logger, "port : {}", port);
                log_info!(logger, "transport : {}", transport);
                log_info!(logger, "message_on_connect : {}", message_on_connect);

                if transport == "tcp" {
                    log_info!(logger, "Mounting serial stream over tcp driver ...");

                    let device_addr = format!("{}:{}", ip, port);

                    // Connecting to the Zybo board
                    log_info!(
                        logger,
                        "Attempting to connect to the device at address {}",
                        device_addr
                    );

                    let stream = match TcpStream::connect(device_addr).await {
                        Ok(s) => s,
                        Err(_) => {
                            log_info!(logger, "Connection failed, rebooting device...");
                            instance.go_error().await;
                            return Err(DriverError("Connection failed".to_string()));
                        }
                    };

                    // Splitting reader and writer
                    let (mut reader, mut writer) = stream.into_split();

                    if message_on_connect == true {
                        log_info!(
                            logger,
                            "Sending an empty command to complete the device initialization"
                        );
                        loop {
                            {
                                // Sending the empty command
                                if let Err(e) = writer.write_all("ping".as_bytes()).await {
                                    log_info!(logger, "Error while sending: {}", e);
                                    instance.go_error().await;
                                    return Err(DriverError(
                                        "Write error during message on connect".to_string(),
                                    ));
                                } else {
                                    let mut buf = [0u8; 1024];

                                    match reader.read(&mut buf).await {
                                        Ok(n) if n > 0 => {
                                            log_info!(logger, "Board initialization complete");
                                            break;
                                        }
                                        Ok(_) => {
                                            log_info!(logger, "No data received, retrying...");
                                            continue;
                                        }
                                        Err(e) => {
                                            log_info!(logger, "Read error: {}, remounting...", e);
                                            instance.go_error().await;
                                            return Err(DriverError(
                                                "Read error during message on connect".to_string(),
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        log_info!(
                            logger,
                            "No need to send a message after the connection of the device"
                        );
                    }

                    log_info!(
                        logger,
                        "Driver mount finished, string attributes can now be mounted"
                    );

                    bytes::mount(instance.clone(), reader, writer).await?;

                    // Ok
                    log_info_mount_end!(logger);
                    return Ok(());
                } else {
                    log_info!(
                        logger,
                        "Mounting  serial stream over serial driver ... [ TO DO ]"
                    );
                    // Ok
                    log_info_mount_end!(logger);
                    return Ok(());
                }
            }
            None => {
                log_error!(
                    logger,
                    "Mount impossible : No settings found in the tree.json, please fill it ..."
                );
                return Err(DriverError(
                    "No settings found to mount the driver".to_string(),
                ));
            }
        }
    }

    async fn wait_reboot_event(&mut self, instance: Instance) {
        sleep(Duration::from_secs(5)).await;

        // Get the IP and the port of the device written in the tree.json
        match instance.settings().await {
            Some(settings) => {
                let ip = settings
                    .get("ip")
                    .expect("no ip found in the tree.json")
                    .as_str()
                    .unwrap();
                let port = settings
                    .get("port")
                    .expect("no port found in the tree.json")
                    .as_str()
                    .unwrap();
                let device_addr = format!("{}:{}", ip, port);

                log_info!(
                    instance.logger(),
                    "Trying to reconnect before rebooting ..."
                );

                let _stream: TcpStream = loop {
                    match TcpStream::connect(&device_addr).await {
                        Ok(s) => {
                            log_info!(
                                instance.logger(),
                                "Reconnection succeed : Trying to mount ..."
                            );
                            break s;
                        }
                        _ => {
                            log_info!(instance.logger(), "Reconnection failed : retrying ...");
                            continue;
                        }
                    }
                };
            }

            None => {
                log_error!(
                    instance.logger(),
                    "No settings founds in the tree.son : the driver didn't reconnect with the device"
                );
            }
        };
    }
}
