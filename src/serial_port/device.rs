use async_trait::async_trait;
use panduza_platform_core::{Device, DeviceOperations, Error};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

use super::model::Model;

pub mod data;
pub mod open;
pub mod settings;

///
/// Device to control PicoHA SSB Board
///
pub struct StdSerialPortDevice {
    model: Arc<Mutex<Model>>, // Serial stream
                              // model: settings + control => shared across
                              // serial_stream: Option<SerialStream>, => totalement own by data
}

impl StdSerialPortDevice {
    ///
    /// Constructor
    ///
    pub fn new() -> Self {
        StdSerialPortDevice {
            model: Model::new().into_arc_mutex(),
        }
    }
}

impl StdSerialPortDevice {
    // ///
    // /// Prepare settings of the device
    // ///
    // pub async fn prepare_settings(&mut self, device: Device) -> Result<(), Error> {
    //     // Get the device logger
    //     let logger = device.logger.clone();

    //     // Get the device settings
    //     let json_settings = device
    //         .settings()
    //         .await
    //         .or(Some(serde_json::Value::Null))
    //         .unwrap();

    //     // Log debug info
    //     logger.info("Build interfaces for \"picoha.dio\" device");
    //     logger.info(format!("JSON settings: {:?}", json_settings));

    //     // // Usb settings
    //     // let usb_settings = UsbSettings::new()
    //     //     .set_vendor(PICOHA_VENDOR_ID)
    //     //     .set_model(PICOHA_PRODUCT_ID)
    //     //     .optional_set_serial_from_json_settings(&json_settings);
    //     // logger.info(format!("USB settings: {:?}", usb_settings));

    //     // Serial settings
    //     self.serial_settings = Some(
    //         SerialSettings::new()
    //             .set_port_name_from_json_or_usb_settings(&json_settings, &usb_settings)
    //             .map_err(|e| Error::Generic(e.to_string()))?
    //             .set_baudrate(PICOHA_SERIAL_BAUDRATE)
    //             .set_read_timeout(Duration::from_secs(2)),
    //     );
    //     logger.info(format!(
    //         "SERIAL settings: {:?}",
    //         self.serial_settings.as_ref().unwrap()
    //     ));

    //     Ok(())
    // }
}

#[async_trait]
impl DeviceOperations for StdSerialPortDevice {
    ///
    ///
    ///
    async fn mount(&mut self, device: Device) -> Result<(), Error> {
        //
        //
        let logger = device.logger.clone();

        //
        //
        logger.debug("Mount attributes");

        open::mount_open_attribute(device.clone(), self.model.clone()).await?;
        data::mount_data_attribute(device.clone(), self.model.clone()).await?;

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
