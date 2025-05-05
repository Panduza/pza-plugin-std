pub mod device;

use panduza_platform_core::{Actions, Producer};

#[derive(Default)]
pub struct Package {}

impl Package {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Producer for Package {
    fn manufacturer(&self) -> String {
        "std".to_string()
    }

    fn model(&self) -> String {
        "serial_stream".to_string()
    }

    fn description(&self) -> String {
        "Serial stream interface".to_string()
    }

    fn props(&self) -> panduza_platform_core::Props {
        panduza_platform_core::Props::default()
    }

    fn produce(&self) -> Result<Box<dyn Actions>, panduza_platform_core::Error> {
        return Ok(Box::new(device::Device::new()));
    }
}
