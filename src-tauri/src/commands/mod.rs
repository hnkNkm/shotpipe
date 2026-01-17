use crate::clipboard::ClipboardManager;
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct Preset {
    pub id: String,
    pub name: String,
    pub max_width: u32,
    pub format: String,
    pub post_action: String,
    pub close_after_copy: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub monitoring: bool,
    pub presets: Vec<Preset>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            monitoring: true,
            presets: vec![
                Preset {
                    id: "preset1".to_string(),
                    name: "Original Size".to_string(),
                    max_width: 0,
                    format: "PNG".to_string(),
                    post_action: "clipboard".to_string(),
                    close_after_copy: true,
                },
                Preset {
                    id: "preset2".to_string(),
                    name: "Max 1200px".to_string(),
                    max_width: 1200,
                    format: "PNG".to_string(),
                    post_action: "clipboard".to_string(),
                    close_after_copy: true,
                },
                Preset {
                    id: "preset3".to_string(),
                    name: "Max 800px".to_string(),
                    max_width: 800,
                    format: "PNG".to_string(),
                    post_action: "clipboard".to_string(),
                    close_after_copy: true,
                },
            ],
        }
    }
}

#[tauri::command]
pub async fn get_clipboard_image(
    state: State<'_, AppState>,
) -> Result<Option<String>, String> {
    state.clipboard_manager.get_image().await
}

#[tauri::command]
pub async fn set_clipboard_image(
    state: State<'_, AppState>,
    image_data: String,
) -> Result<(), String> {
    state.clipboard_manager.set_image(&image_data).await
}

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<Settings, String> {
    let settings = state.settings.lock().await;
    Ok(settings.clone())
}

#[tauri::command]
pub async fn save_settings(
    state: State<'_, AppState>,
    settings: Settings,
) -> Result<(), String> {
    let mut current_settings = state.settings.lock().await;
    *current_settings = settings;
    Ok(())
}

#[tauri::command]
pub async fn start_monitoring(state: State<'_, AppState>) -> Result<(), String> {
    let mut monitoring = state.monitoring.lock().await;
    *monitoring = true;
    log::info!("Monitoring started");
    Ok(())
}

#[tauri::command]
pub async fn stop_monitoring(state: State<'_, AppState>) -> Result<(), String> {
    let mut monitoring = state.monitoring.lock().await;
    *monitoring = false;
    log::info!("Monitoring stopped");
    Ok(())
}

#[tauri::command]
pub async fn is_monitoring(state: State<'_, AppState>) -> Result<bool, String> {
    let monitoring = state.monitoring.lock().await;
    Ok(*monitoring)
}