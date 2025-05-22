use panduza_platform_core::Producer;
//use panduza_platform_core::Scanner;

#[cfg(feature = "plugin")]
panduza_platform_core::plugin_interface!("std", "0.0.0");

//mod scpi;
//mod serial_port;
mod serial_stream;

// Export the producers of the plugin
//
pub fn plugin_producers() -> Vec<Box<dyn Producer>> {
    let mut producers: Vec<Box<dyn Producer>> = vec![];
    //producers.push(serial_port::producer::StdSerialPort::new());
    //producers.push(scpi::Package::default().boxed());
    producers.push(serial_stream::Package::default().boxed());
    return producers;
}

//
/*
pub fn plugin_scanners() -> Vec<Box<dyn Scanner>> {
    let mut scanners: Vec<Box<dyn Scanner>> = vec![];
    scanners.push(scpi::Package::default().boxed());
    return scanners;
}
*/
