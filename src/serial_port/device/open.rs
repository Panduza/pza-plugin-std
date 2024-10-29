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

    Ok(())
}
