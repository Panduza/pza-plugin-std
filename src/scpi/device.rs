use async_trait::async_trait;
use panduza_platform_core::drivers::usb::tmc::Driver as UsbTmcDriver;
use panduza_platform_core::drivers::usb::Settings as UsbSettings;
use panduza_platform_core::{DriverOperations, Error, Instance};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

// pub mod data;
// pub mod open;
// pub mod settings;

#[derive(Default)]
///
/// Device to control PicoHA SSB Board
///
pub struct Device {}

impl Device {
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
impl DriverOperations for Device {
    ///
    ///
    ///
    async fn mount(&mut self, instance: Instance) -> Result<(), Error> {
        //
        //
        let logger = instance.logger.clone();

        // // Usb settings
        // let usb_settings = UsbSettings::new()
        //     .set_vendor(DEVICE_VENDOR_ID)
        //     .set_model(DEVICE_PRODUCT_ID);
        // // .optional_set_serial_from_json_settings(&json_settings);
        // logger.info(format!("USB settings: {}", usb_settings));

        // // let dev = usb_settings.find_usb_device();
        // // logger.info(format!("dev: {:?}", dev));

        // // endpoint_in: 0x81,
        // // endpoint_out: 0x01,
        // // max_packet_size: 512,
        // let mut driver = UsbTmcDriver::open(&usb_settings, 0x81, 0x01, 512)
        //     .unwrap()
        //     .into_arc_mutex();

        // panduza_platform_core::std::class::repl::mount("scpi", instance.clone(), driver).await?;

        // => attribute => send command STRING => return response string

        // open::mount_open_attribute(device.clone(), self.model.clone()).await?;
        // data::mount_data_attribute(device.clone(), self.model.clone()).await?;

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
    async fn wait_reboot_event(&mut self, mut _device: Instance) {
        sleep(Duration::from_secs(5)).await;
    }
}
