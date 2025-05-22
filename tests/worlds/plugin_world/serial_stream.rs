use bytes::Bytes;
use cucumber::{given, then, when};

use super::PluginWorld;
///
///
#[given(expr = "the bytes attribute rx {string}")]
async fn the_bytes_attribute_ro(world: &mut PluginWorld, attribute_name: String) {
    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .expect("Attribute not found");
    let attribute = attribute_builder.expect_bytes().await.unwrap();

    world.serial_stream.att_bytes_rx = Some(attribute);
}

///
///
#[given(expr = "the bytes attribute tx {string}")]
async fn the_bytes_attribute_wo(world: &mut PluginWorld, attribute_name: String) {
    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .expect("Attribute not found");
    let attribute = attribute_builder.expect_bytes().await.unwrap();

    world.serial_stream.att_bytes_tx = Some(attribute);
}

///
///
#[when(expr = "I set tx bytes to {string}")]
async fn i_set_wo_bytes_to(world: &mut PluginWorld, b: String) {
    let bytes: Bytes = Bytes::from(b);
    world
        .serial_stream
        .att_bytes_tx
        .as_mut()
        .unwrap()
        .set(bytes)
        .await
        .unwrap();
}

///
///
#[then(expr = "the rx bytes value is {string}")]
async fn the_ro_bytes_value_is(world: &mut PluginWorld, b: String) {
    let timeout = std::time::Duration::from_secs(3);
    let start_time = std::time::Instant::now();
    let expected_value = Bytes::from(b);

    loop {
        let read_value = world
            .serial_stream
            .att_bytes_rx
            .as_ref()
            .unwrap()
            .get()
            .unwrap();
        if read_value == expected_value {
            break;
        }
        if start_time.elapsed() >= timeout {
            panic!(
                "Timeout reached: read '{:?}' != expected '{:?}'",
                read_value, expected_value
            );
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    let read_value = world
        .serial_stream
        .att_bytes_rx
        .as_ref()
        .unwrap()
        .get()
        .unwrap();
    assert_eq!(
        read_value, expected_value,
        "read '{:?}' != expected '{:?}'",
        read_value, expected_value
    );
}
