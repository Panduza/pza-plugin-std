use super::device::StdSerialPortDevice;
use panduza_platform_core::{DeviceOperations, Producer};

pub struct StdSerialPort {}

impl StdSerialPort {
    pub fn new() -> Box<StdSerialPort> {
        Box::new(StdSerialPort {})
    }
}

impl Producer for StdSerialPort {
    fn manufacturer(&self) -> String {
        "std".to_string()
    }

    fn model(&self) -> String {
        "serial-port".to_string()
    }

    fn produce(&self) -> Result<Box<dyn DeviceOperations>, panduza_platform_core::Error> {
        return Ok(Box::new(StdSerialPortDevice::new()));
    }
}
