use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::Notify;

use panduza_platform_connectors::SerialSettings;

pub enum Request {
    Open,
    Close,
    Restart,
}

pub struct Model {
    // settings + control => shared across
    settings: SerialSettings,

    ///
    ///
    request: Option<Request>,
    request_notifier: Arc<Notify>,
}

impl Model {
    ///
    ///
    ///
    pub fn new() -> Self {
        Model {
            settings: SerialSettings::new(),
            request: None,
            request_notifier: Arc::new(Notify::new()),
        }
    }

    pub fn into_arc_mutex(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

    ///
    /// Setting cloner
    ///
    pub fn settings(&self) -> SerialSettings {
        self.settings.clone()
    }

    pub fn clone_request_notifier(&self) -> Arc<Notify> {
        self.request_notifier.clone()
    }

    pub fn take_request(&mut self) -> Option<Request> {
        self.request.take()
    }

    pub fn request_open(&mut self) {
        self.request = Some(Request::Open);
        self.request_notifier.notify_waiters();
    }

    pub fn request_close(&mut self) {
        self.request = Some(Request::Close);
        self.request_notifier.notify_waiters();
    }
}
