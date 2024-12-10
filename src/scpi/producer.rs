use super::device::StdScpiDevice;
use panduza_platform_core::{DriverOperations, Producer};

pub struct StdScpi {}

impl StdScpi {
    pub fn new() -> Box<StdScpi> {
        Box::new(StdScpi {})
    }
}

impl Producer for StdScpi {
    fn manufacturer(&self) -> String {
        "std".to_string()
    }

    fn model(&self) -> String {
        "scpi".to_string()
    }

    fn description(&self) -> String {
        "".to_string()
    }

    fn props(&self) -> panduza_platform_core::Props {
        panduza_platform_core::Props::default()
    }

    fn produce(&self) -> Result<Box<dyn DriverOperations>, panduza_platform_core::Error> {
        return Ok(Box::new(StdScpiDevice::new()));
    }
}
