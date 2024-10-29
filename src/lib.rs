panduza_platform_core::plugin_interface!("pza-plugin-std");

mod serial_port;

// Export the producers of the plugin
//
pub fn plugin_producers() -> Vec<Box<dyn Producer>> {
    let mut producers: Vec<Box<dyn Producer>> = vec![];
    producers.push(serial_port::producer::StdSerialPort::new());
    return producers;
}
