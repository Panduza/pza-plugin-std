use async_trait::async_trait;
use panduza_platform_core::connector::usb::tmc::Driver as UsbTmcDriver;
use panduza_platform_core::connector::usb::Settings as UsbSettings;
use panduza_platform_core::{log_debug, DriverOperations, Error, Instance};
use std::time::Duration;
use tokio::time::sleep;

// pub mod data;
// pub mod open;
// pub mod settings;

#[derive(Default)]
///
/// Device to control PicoHA SSB Board
///
pub struct Device {}

impl Device {}

#[async_trait]
impl DriverOperations for Device {
    ///
    /// Mount the device instance
    ///
    async fn mount(&mut self, mut instance: Instance) -> Result<(), Error> {
        //
        //
        let logger = instance.logger.clone();

        //
        // Usb settings
        let settings = instance.settings().await.ok_or(Error::BadSettings(
            "Usb Settings are required for this instance".to_string(),
        ))?;

        //
        // Compose USB settings
        let usb_settings = UsbSettings::from_json_settings(&settings);
        log_debug!(logger, "Try to open SCPI interface on {:?}", &usb_settings);

        //     .set_vendor(DEVICE_VENDOR_ID)
        //     .set_model(DEVICE_PRODUCT_ID);
        // // .optional_set_serial_from_json_settings(&json_settings);
        // logger.info(format!("USB settings: {}", usb_settings));

        // // let dev = usb_settings.find_usb_device();
        // // logger.info(format!("dev: {:?}", dev));

        // // endpoint_in: 0x81,
        // // endpoint_out: 0x01,
        // // max_packet_size: 512,

        // let pok = instance
        //     .create_attribute("pok")
        //     .with_rw()
        //     .finish_as_json()
        //     .await?;

        // pok.set(json!("Hello, World!")).await?;

        //
        // Mount the driver
        let mut driver = UsbTmcDriver::open(&usb_settings)?.into_arc_mutex();

        panduza_platform_core::std::class::repl::mount("scpi", instance.clone(), driver).await?;

        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, mut _device: Instance) {
        sleep(Duration::from_secs(5)).await;
    }
}
