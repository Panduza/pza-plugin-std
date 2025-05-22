mod serial_port;
mod tcp;
use async_trait::async_trait;
use panduza_platform_core::Error::DriverError;
use panduza_platform_core::{
    log_error, log_info_mount_end, log_info_mount_start, log_trace, Actions, Container, Error,
    Instance, Logger,
};
//tcp
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::sleep;
//serial port
use tokio_serial::{SerialPortBuilderExt, SerialStream};

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
    async fn mount(&mut self, instance: Instance) -> Result<(), Error> {
        // Start logging
        let logger = instance.logger().clone();
        log_info_mount_start!(logger);

        // Catching the IP and the Port in the settings
        match instance.settings().await {
            Some(settings) => {
                let transport = settings
                    .get("transport")
                    .expect("no transport found in the tree.json")
                    .as_str()
                    .unwrap();

                let tcp_ip = settings
                    .get("tcp_ip")
                    .expect("no tcp_ip found in the tree.json")
                    .as_str()
                    .unwrap();

                let tcp_port = settings
                    .get("tcp_port")
                    .expect("no tcp_port found in the tree.json")
                    .as_str()
                    .unwrap();

                let serial_port_name: &str = settings
                    .get("serial_port_name")
                    .expect("no serial_port_name found in the tree.json")
                    .as_str()
                    .unwrap();

                let serial_baud_rate_str: &str = settings
                    .get("serial_baud_rate")
                    .expect("no serial_port_name found in the tree.json")
                    .as_str()
                    .unwrap();
                let serial_baud_rate: u32 = serial_baud_rate_str
                    .parse()
                    .expect("serial_baud_rate should be a 32 bits integers");

                let message_on_connect = settings
                    .get("message_on_connect")
                    .expect("no message_on_connect found in the tree.json")
                    .as_bool()
                    .unwrap();

                if transport == "tcp" {
                    log_trace!(logger, "[ SETTINGS CHOSEN ]");
                    log_trace!(logger, "transport : {}", transport);
                    log_trace!(logger, "tcp_ip : {}", tcp_ip);
                    log_trace!(logger, "tcp_port : {}", tcp_port);
                    log_trace!(logger, "message_on_connect : {}", message_on_connect);
                    log_trace!(logger, "Mounting serial stream over tcp driver ...");

                    let device_addr = format!("{}:{}", tcp_ip, tcp_port);

                    // Connecting to the Zybo board
                    log_trace!(
                        logger,
                        "Attempting to connect to the device at address {}",
                        device_addr
                    );

                    let stream = match TcpStream::connect(device_addr).await {
                        Ok(s) => s,
                        Err(_) => {
                            log_trace!(logger, "Connection failed, rebooting device...");
                            return Err(DriverError("Connection failed".to_string()));
                        }
                    };

                    // Splitting reader and writer
                    let (mut reader, mut writer) = stream.into_split();

                    if message_on_connect == true {
                        log_trace!(
                            logger,
                            "Sending an empty command to complete the device initialization"
                        );
                        loop {
                            {
                                // Sending the empty command
                                if let Err(e) = writer.write_all("ping".as_bytes()).await {
                                    log_trace!(logger, "Error while sending: {}", e);
                                    return Err(DriverError(
                                        "Write error during message on connect".to_string(),
                                    ));
                                } else {
                                    let mut buf = [0u8; 1024];

                                    match reader.read(&mut buf).await {
                                        Ok(n) if n > 0 => {
                                            log_trace!(logger, "Board initialization complete");
                                            break;
                                        }
                                        Ok(_) => {
                                            log_trace!(logger, "No data received, retrying...");
                                            continue;
                                        }
                                        Err(e) => {
                                            log_trace!(logger, "Read error: {}, remounting...", e);
                                            return Err(DriverError(
                                                "Read error during message on connect".to_string(),
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        log_trace!(
                            logger,
                            "No need to send a message after the connection of the device"
                        );
                    }

                    log_trace!(
                        logger,
                        "Driver mount finished, string attributes can now be mounted"
                    );

                    tcp::mount(instance.clone(), reader, writer).await?;

                    // Ok
                    log_info_mount_end!(logger);
                    return Ok(());
                } else if transport == "serial-port" {
                    log_trace!(logger, "[ SETTINGS CHOSEN ]");
                    log_trace!(logger, "serial_port_name : {}", serial_port_name);
                    log_trace!(logger, "serial_baud_rate : {}", serial_baud_rate);
                    log_trace!(logger, "Mounting serial port over serial driver ...");

                    let port: SerialStream =
                        match tokio_serial::new(serial_port_name, serial_baud_rate)
                            .timeout(Duration::from_millis(1000))
                            .open_native_async()
                        {
                            Ok(p) => p,
                            Err(_) => {
                                log_trace!(logger, "Connection failed, rebooting device...");
                                return Err(DriverError("Connection failed".to_string()));
                            }
                        };

                    serial_port::mount(instance.clone(), port).await?;

                    // Ok
                    log_info_mount_end!(logger);
                    return Ok(());
                } else {
                    log_trace!(
                        logger,
                        "Mounting other transport over serial driver ... [ TO DO ]"
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
                let transport = settings
                    .get("transport")
                    .expect("no transport found in the tree.json")
                    .as_str()
                    .unwrap();
                let tcp_ip = settings
                    .get("tcp_ip")
                    .expect("no tcp_ip found in the tree.json")
                    .as_str()
                    .unwrap();
                let tcp_port = settings
                    .get("tcp_port")
                    .expect("no tcp_port found in the tree.json")
                    .as_str()
                    .unwrap();
                let serial_port_name: &str = settings
                    .get("serial_port_name")
                    .expect("no serial_port_name found in the tree.json")
                    .as_str()
                    .unwrap();
                let serial_baud_rate_str: &str = settings
                    .get("serial_baud_rate")
                    .expect("no serial_port_name found in the tree.json")
                    .as_str()
                    .unwrap();
                let serial_baud_rate: u32 = serial_baud_rate_str
                    .parse()
                    .expect("serial_baud_rate should be a 32 bits integers");

                log_trace!(
                    instance.logger(),
                    "Trying to reconnect before rebooting ..."
                );

                if transport == "tcp" {
                    let device_addr = format!("{}:{}", tcp_ip, tcp_port);

                    let _stream: TcpStream = loop {
                        match TcpStream::connect(&device_addr).await {
                            Ok(s) => {
                                log_trace!(
                                    instance.logger(),
                                    "Reconnection succeed : Trying to mount ..."
                                );
                                break s;
                            }
                            _ => {
                                log_trace!(instance.logger(), "Reconnection failed : retrying ...");
                                continue;
                            }
                        }
                    };
                } else if transport == "serial-port" {
                    let port: SerialStream = loop {
                        match tokio_serial::new(serial_port_name, serial_baud_rate)
                            .timeout(Duration::from_millis(1000))
                            .open_native_async()
                        {
                            Ok(p) => {
                                log_trace!(
                                    instance.logger(),
                                    "Reconnection succeed : Trying to mount ..."
                                );
                                break p;
                            }
                            _ => {
                                log_trace!(instance.logger(), "Reconnection failed : retrying ...");
                                tokio::time::sleep(Duration::from_millis(1000)).await;
                                continue;
                            }
                        }
                    };
                    log_trace!(instance.logger(), "Dropping port");
                    drop(port);
                    tokio::time::sleep(Duration::from_millis(300)).await;
                } else {
                    log_trace!(instance.logger(), "Reboot other transport ... [ TO DO ]");
                }
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
