use async_trait::async_trait;
use panduza_platform_core::DeviceLogger;
use panduza_platform_core::{Device, DeviceOperations, Error};
use std::time::Duration;
use tokio::time::sleep;

pub mod data;
pub mod open;
pub mod settings;

///
/// Device to control PicoHA SSB Board
///
pub struct StdSerialPortDevice {
    ///
    /// Device logger
    logger: Option<DeviceLogger>,
}

impl StdSerialPortDevice {
    ///
    /// Constructor
    ///
    pub fn new() -> Self {
        StdSerialPortDevice { logger: None }
    }
}

#[async_trait]
impl DeviceOperations for StdSerialPortDevice {
    ///
    ///
    ///
    async fn mount(&mut self, device: Device) -> Result<(), Error> {
        //
        // Init logger
        self.logger = Some(device.logger.clone());

        //
        //
        let logger = device.logger.clone();

        //
        //
        logger.debug("Mount attributes");
        // //
        // // Mount bus selector (to choice the bus to use on the pico)
        // mount_bus_selector(device.clone()).await?;
        // //
        // // Mount memory maps
        // mount_memory_map("C1", 0, device.clone()).await?;
        // mount_memory_map("C2", 1, device.clone()).await?;
        // mount_memory_map("C3", 2, device.clone()).await?;

        open::mount_open_attribute(device.clone()).await?;
        data::mount_data_attribute(device.clone()).await?;

        // open => boolean
        // settings
        //      - port_name
        //      - baudrate
        //      -
        // data -> attribut stream string
        //

        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, mut _device: Device) {
        sleep(Duration::from_secs(5)).await;
    }
}
