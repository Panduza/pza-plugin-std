use std::sync::Arc;

use tokio::sync::Mutex;

pub struct Model {
    // settings + control => shared across
}

impl Model {
    ///
    ///
    ///
    pub fn new() -> Self {
        Model {}
    }

    pub fn into_arc_mutex(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }
}
