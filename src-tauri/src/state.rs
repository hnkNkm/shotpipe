use crate::clipboard::ClipboardManager;
use crate::commands::Settings;
use crate::monitor::ClipboardMonitor;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    pub clipboard_manager: ClipboardManager,
    pub settings: Arc<Mutex<Settings>>,
    pub monitoring: Arc<Mutex<bool>>,
    pub clipboard_monitor: Arc<Mutex<Option<ClipboardMonitor>>>,
}

impl AppState {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            clipboard_manager: ClipboardManager::new()?,
            settings: Arc::new(Mutex::new(Settings::default())),
            monitoring: Arc::new(Mutex::new(false)),
            clipboard_monitor: Arc::new(Mutex::new(None)),
        })
    }
}