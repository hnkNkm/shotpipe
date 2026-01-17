use crate::state::AppState;
use log::{debug, error, info};
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::mpsc;
use tokio::time;

pub struct ClipboardMonitor {
    app_handle: AppHandle,
    stop_tx: Option<mpsc::Sender<()>>,
}

impl ClipboardMonitor {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            stop_tx: None,
        }
    }

    pub async fn start(&mut self) -> Result<(), String> {
        if self.stop_tx.is_some() {
            return Err("Monitoring is already running".to_string());
        }

        let (stop_tx, mut stop_rx) = mpsc::channel(1);
        self.stop_tx = Some(stop_tx);

        let app_handle = self.app_handle.clone();
        
        tokio::spawn(async move {
            info!("Clipboard monitoring started");
            
            let mut interval = time::interval(Duration::from_millis(500));
            
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        if let Err(e) = check_clipboard(&app_handle).await {
                            error!("Error checking clipboard: {}", e);
                        }
                    }
                    _ = stop_rx.recv() => {
                        info!("Clipboard monitoring stopped");
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), String> {
        if let Some(stop_tx) = self.stop_tx.take() {
            stop_tx
                .send(())
                .await
                .map_err(|_| "Failed to send stop signal".to_string())?;
            Ok(())
        } else {
            Err("Monitoring is not running".to_string())
        }
    }

    pub fn is_running(&self) -> bool {
        self.stop_tx.is_some()
    }
}

async fn check_clipboard(app_handle: &AppHandle) -> Result<(), String> {
    let state = app_handle.state::<AppState>();
    
    // Check if monitoring is enabled
    let monitoring = *state.monitoring.lock().await;
    if !monitoring {
        return Ok(());
    }
    
    // Check for new image in clipboard
    match state.clipboard_manager.get_image().await {
        Ok(Some(image_data)) => {
            debug!("New image detected in clipboard");
            
            // Emit event to frontend with image data
            info!("About to emit event with image data length: {}", image_data.len());
            
            match app_handle.emit("clipboard-image-detected", &image_data) {
                Ok(_) => {
                    info!("Successfully emitted clipboard-image-detected event");
                }
                Err(e) => {
                    error!("Failed to emit event: {}", e);
                    return Err(format!("Failed to emit event: {}", e));
                }
            }
        }
        Ok(None) => {
            // No new image or same image
        }
        Err(e) => {
            error!("Failed to check clipboard: {}", e);
        }
    }
    
    Ok(())
}