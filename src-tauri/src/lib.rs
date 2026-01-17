mod clipboard;
mod commands;
mod state;
mod tray;

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
            
            // Setup system tray menu and event handlers
            // The tray icon itself is created by tauri.conf.json
            match tray::setup_tray(app.handle()) {
                Ok(_) => log::info!("System tray configured successfully"),
                Err(e) => log::error!("Failed to configure system tray: {}", e),
            }
            
            log::info!("Shotpipe application started");
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::new().build())
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