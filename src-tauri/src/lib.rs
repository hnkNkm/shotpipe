mod clipboard;
mod commands;
mod state;

use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();
    
    tauri::Builder::default()
        .setup(|app| {
            // Initialize app state
            let app_state = AppState::new().expect("Failed to initialize app state");
            app.manage(app_state);
            
            log::info!("Shotpipe application started");
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_clipboard_image,
            commands::set_clipboard_image,
            commands::get_settings,
            commands::save_settings,
            commands::start_monitoring,
            commands::stop_monitoring,
            commands::is_monitoring,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}