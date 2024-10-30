use panduza_platform_core::{
    spawn_on_command, AttOnlyMsgAtt, BidirMsgAtt, BooleanCodec, Device, DeviceLogger, Error,
};

///
///
///
pub async fn mount_open_attribute(mut device: Device) -> Result<(), Error> {
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
        on_open_change(logger_for_cmd_event.clone(), attribute.clone())
    );

    Ok(())
}

///
///
///
async fn on_open_change(
    logger: DeviceLogger,
    mut attribute: BidirMsgAtt<BooleanCodec>,
) -> Result<(), Error> {
    while let Some(command) = attribute.pop_cmd().await {}
    Ok(())
}
