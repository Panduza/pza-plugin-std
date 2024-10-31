use crate::serial_port::model::Model;
use panduza_platform_core::{
    spawn_on_command, BidirMsgAtt, BooleanCodec, Device, DeviceLogger, Error,
};
use std::sync::Arc;
use tokio::sync::Mutex;

///
///
///
pub async fn mount_open_attribute(
    mut device: Device,
    model: Arc<Mutex<Model>>,
) -> Result<(), Error> {
    //
    // Create the attribute
    let attribute = device
        .create_attribute("open")
        .message()
        .with_bidir_access()
        .finish_with_codec::<BooleanCodec>()
        .await;

    //
    // Init to false
    attribute.set(false).await.unwrap();

    //
    // Execute action on each command received
    let logger_for_cmd_event = device.logger.clone();

    spawn_on_command!(
        device,
        attribute,
        on_open_change(
            logger_for_cmd_event.clone(),
            attribute.clone(),
            model.clone()
        )
    );

    Ok(())
}

///
///
///
async fn on_open_change(
    logger: DeviceLogger,
    mut attribute: BidirMsgAtt<BooleanCodec>,
    model: Arc<Mutex<Model>>,
) -> Result<(), Error> {
    // println!("poooooooooooooooo on_open_change");
    while let Some(command) = attribute.pop_cmd().await {
        logger.debug(format!("open {:?}", command));

        let order = command.value;
        match order {
            true => {
                model.lock().await.request_open();
            }
            false => {
                model.lock().await.request_close();
            }
        }
    }
    Ok(())
}
