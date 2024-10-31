use crate::serial_port::model::Model;
use crate::serial_port::model::Request;
use panduza_platform_connectors::SerialSettings;
use panduza_platform_core::{
    spawn_loop, spawn_on_command, BidirMsgAtt, Device, DeviceLogger, Error, RawCodec,
};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::sync::{Mutex, Notify};
use tokio::time::sleep;
use tokio_serial::SerialStream;

struct Driver {
    serial_stream: Option<SerialStream>,
}

impl Driver {
    ///
    ///
    fn new() -> Self {
        Self {
            serial_stream: None,
        }
    }

    ///
    ///
    pub async fn open(&mut self, settings: &SerialSettings) -> Result<(), Error> {
        //
        // Internal driver already initialized by an other entity => OK
        if self.serial_stream.is_some() {
            return Ok(());
        }

        //
        // Get the port name
        let port_name = settings
            .port_name
            .as_ref()
            .ok_or_else(|| Error::BadSettings("Port name is not set in settings".to_string()))?;

        //
        // Setup builder
        let serial_builder = tokio_serial::new(port_name, settings.baudrate)
            .data_bits(settings.data_bits)
            .stop_bits(settings.stop_bits)
            .parity(settings.parity)
            .flow_control(settings.flow_control);

        //
        // Build the stream
        self.serial_stream = Some(
            SerialStream::open(&serial_builder)
                .map_err(|e| Error::BadSettings(format!("Unable to open serial stream: {}", e)))?,
        );

        Ok(())
    }
}

///
///
///
pub async fn mount_data_attribute(
    mut device: Device,
    model: Arc<Mutex<Model>>,
) -> Result<(), Error> {
    //
    // Create the attribute
    let attribute = device
        .create_attribute("data")
        .message()
        .with_bidir_access()
        .finish_with_codec::<RawCodec>()
        .await;

    //
    //
    let request_notifier = model.lock().await.clone_request_notifier();
    spawn_loop!(device, {
        //
        //
        let mut driver = Driver::new();
        //
        //
        process_requests(request_notifier.clone(), model.clone(), &mut driver).await;
    });

    // spawn_on_command!(
    //     device,
    //     attribute,
    //     on_open_change(logger_for_cmd_event.clone(), attribute.clone())
    // );

    Ok(())
}

async fn process_requests(notifier: Arc<Notify>, model: Arc<Mutex<Model>>, driver: &mut Driver) {
    tokio::select! {

        //
        // new request
        _ = notifier.notified() => {

            let mut model_lock = model.lock().await;

            match model_lock.take_request() {
                Some(request) => match request {
                    Request::Open => {}
                    Request::Close => todo!(),
                    Request::Restart => todo!(),
                },
                None => todo!(),
            }

            drop(model_lock);
        }

        // something to write
        // something to read
    }
}

// ///
// ///
// ///
// async fn on_open_change(
//     logger: DeviceLogger,
//     mut attribute: BidirMsgAtt<RawCodec>,
// ) -> Result<(), Error> {
//     while let Some(command) = attribute.pop_cmd().await {}
//     Ok(())
// }
