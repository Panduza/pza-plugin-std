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
// use tokio_serial::SerialStream;
use serial2_tokio::SerialPort;

struct Driver {
    // pub serial_stream: Option<SerialStream>,
    pub serial_stream: Option<SerialPort>,
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
        // let port_name = settings
        //     .port_name
        //     .as_ref()
        //     .ok_or_else(|| Error::BadSettings("Port name is not set in settings".to_string()))?;

        //
        //
        let port = SerialPort::open("COM7", 115200).unwrap();

        // //
        // // Setup builder
        // let serial_builder = tokio_serial::new(port_name, settings.baudrate)
        //     .data_bits(settings.data_bits)
        //     .stop_bits(settings.stop_bits)
        //     .parity(settings.parity)
        //     .flow_control(settings.flow_control);

        //
        // Build the stream
        self.serial_stream = Some(port);

        Ok(())
    }

    ///
    ///
    ///
    pub async fn read(&self, buf: &mut [u8]) {
        match &self.serial_stream {
            Some(stream) => {
                stream.read(buf).await.unwrap();
            }
            None => {
                sleep(Duration::from_secs(60)).await;
            }
        }
    }

    // pub async fn read_available(&self) {
    //     match &self.serial_stream {
    //         Some(stream) => {
    //             stream.readable().await.unwrap();
    //         }
    //         None => sleep(Duration::from_secs(60)).await,
    //     }
    // }
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
    let attribute: BidirMsgAtt<RawCodec> = device
        .create_attribute("data")
        .message()
        .with_bidir_access()
        .finish_with_codec::<RawCodec>()
        .await;

    //
    //
    let request_notifier = model.lock().await.clone_request_notifier();
    device
        .spawn(async move {
            // tokio::spawn(async move {
            // loop {
            process_requests(request_notifier.clone(), model.clone(), attribute.clone()).await;
            // }

            Ok(())
        })
        .await;

    // spawn_on_command!(
    //     device,
    //     attribute,
    //     on_open_change(logger_for_cmd_event.clone(), attribute.clone())
    // );

    Ok(())
}

///
///
///
async fn process_requests(
    notifier: Arc<Notify>,
    model: Arc<Mutex<Model>>,
    // driver: Arc<Mutex<Driver>>,
    attribute: BidirMsgAtt<RawCodec>,
) {
    // //
    //
    // let driver = Arc::new(Mutex::new(Driver::new()));
    let mut driver = Driver::new();

    // let mut ppp = driver.lock().await.serial_stream.take().unwrap();

    // // ppp.readable().await;
    let mut buf: [u8; 512] = [0; 512];
    // ppp.try_read(&mut buf);

    // let mut driver_lock = driver.lock().await;
    loop {
        tokio::select! {

            //
            // new request
            _ = notifier.notified() => {

                let mut model_lock = model.lock().await;

                match model_lock.take_request() {
                    Some(request) => match request {
                        Request::Open => {
                            driver.open(&model_lock.settings()).await.unwrap();
                        }
                        Request::Close => todo!(),
                        Request::Restart => todo!(),
                    },
                    None => todo!(),
                }

                drop(model_lock);
            }
            _ = driver.read(&mut buf) => {
                println!("data : {:?}", buf);
                // attribute.set(buf).await;
            }
            // _ = driver_lock.read_available() => {
            //     println!("read");
            // }

            // something to write
            // something to read
        }
    }

    // drop(driver_lock);
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
