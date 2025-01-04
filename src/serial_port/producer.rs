use super::device::StdSerialPortDevice;
use panduza_platform_core::{DriverOperations, Producer};

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

    fn description(&self) -> String {
        "".to_string()
    }

    fn props(&self) -> panduza_platform_core::Props {
        panduza_platform_core::Props::default()
    }

    fn produce(&self) -> Result<Box<dyn DriverOperations>, panduza_platform_core::Error> {
        return Ok(Box::new(StdSerialPortDevice::new()));
    }
}
