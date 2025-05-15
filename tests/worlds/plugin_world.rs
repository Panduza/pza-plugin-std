mod serial_stream;

use cucumber::Parameter;
use cucumber::{given, then, World};
use panduza::{reactor::ReactorOptions, BooleanAttribute, BytesAttribute, JsonAttribute, Reactor};
use panduza::{NotificationAttribute, StatusAttribute};
use std::time::Duration;
use std::{fmt::Debug, str::FromStr};

// --- TEST PARAMETERS ---
const PLAFORM_LOCALHOST: &str = "localhost";
const PLAFORM_PORT: u16 = 1883;
// -----------------------

#[derive(Default)]
pub struct SerialStreamSubWorld {
    pub att_bytes_rx: Option<BytesAttribute>,
    pub att_bytes_tx: Option<BytesAttribute>,
    pub att_boolean_alert_wo: Option<BooleanAttribute>,
    pub att_boolean_error_wo: Option<BooleanAttribute>,
    //pub topic_bytes_rx: Option<String>,
    //pub topic_bytes_tx: Option<String>,
    pub topic_boolean_alert_wo: Option<String>,
    pub topic_boolean_error_wo: Option<String>,
}

#[derive(Default, World)]
pub struct PluginWorld {
    /// Reactor object
    ///
    pub r: Option<Reactor>,

    ///
    ///
    pub platform_status: Option<StatusAttribute>,

    ///
    ///
    pub platform_notifications: Option<NotificationAttribute>,

    ///
    ///
    pub att_instance_status: Option<JsonAttribute>,

    /// Reactor sub world data
    ///
    pub serial_stream: SerialStreamSubWorld,
}

impl Debug for PluginWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PluginWorld")
            // .field("r", &self.r)
            .finish()
    }
}

///
///
#[given(expr = "a reactor connected on a test platform")]
async fn a_client_connected_on_a_test_platform(world: &mut PluginWorld) {
    let options = ReactorOptions::new(PLAFORM_LOCALHOST, PLAFORM_PORT);
    let reactor = panduza::new_reactor(options).await.unwrap();

    world.r = Some(reactor);
    world.platform_status = Some(world.r.as_ref().unwrap().new_status_attribute().await);

    world.platform_notifications =
        Some(world.r.as_ref().unwrap().new_notification_attribute().await);

    world
        .platform_status
        .as_mut()
        .unwrap()
        .wait_for_all_instances_to_be_running(Duration::from_secs(15))
        .await
        .expect("Error while waiting for instance to be in running state");
}

///
///
#[given(expr = "the tested driver connect with the device")]
async fn driver_connected_with_device(world: &mut PluginWorld) {
    //
    // Make sure before starting the test to connect the platform via the plugin serial_stream to the device using a TCP or serial connection
    //
}
