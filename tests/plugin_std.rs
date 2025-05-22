mod worlds;
use cucumber::World;
use worlds::PluginWorld;

#[tokio::main]
async fn main() {
    println!("Make sure before starting the test to connect the platform via the plugin serial_stream to the device using a TCP or serial connection");
    PluginWorld::run("tests/features/plugin").await;
}
