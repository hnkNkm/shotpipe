use tauri::{
    AppHandle, Manager, Runtime, Emitter,
    menu::{Menu, MenuBuilder, MenuItemBuilder, PredefinedMenuItem},
};
use crate::state::AppState;

pub fn setup_tray<R: Runtime>(app: &AppHandle<R>) -> Result<(), Box<dyn std::error::Error>> {
    // The tray icon is created automatically by tauri.conf.json
    // We just need to set up the menu and event handlers
    
    let menu = create_tray_menu(app)?;
    
    // Get the tray that was created by tauri.conf.json
    if let Some(tray) = app.tray_by_id("tray") {
        // Set the menu for the existing tray
        tray.set_menu(Some(menu))?;
        
        // Set up menu event handler
        tray.on_menu_event(move |app, event| {
            log::info!("Tray menu event triggered: {:?}", event.id());
            handle_tray_event(app, event.id().as_ref());
        });
        
        log::info!("Tray menu configured successfully");
    } else {
        log::error!("Tray icon not found - check tauri.conf.json configuration");
        return Err("Tray icon not found".into());
    }
    
    Ok(())
}

fn create_tray_menu<R: Runtime>(app: &AppHandle<R>) -> Result<Menu<R>, Box<dyn std::error::Error>> {
    let state = app.state::<AppState>();
    let monitoring = futures::executor::block_on(async {
        *state.monitoring.lock().await
    });

    let monitoring_text = if monitoring {
        "Stop Monitoring"
    } else {
        "Start Monitoring"
    };

    let toggle_monitoring = MenuItemBuilder::new(monitoring_text)
        .id("toggle_monitoring")
        .build(app)?;
        
    let open_settings = MenuItemBuilder::new("Settings")
        .id("open_settings")
        .build(app)?;
    
    let separator = PredefinedMenuItem::separator(app)?;
    
    let quit = MenuItemBuilder::new("Quit")
        .id("quit")
        .build(app)?;
    
    let menu = MenuBuilder::new(app)
        .item(&toggle_monitoring)
        .item(&open_settings)
        .item(&separator)
        .item(&quit)
        .build()?;
    
    Ok(menu)
}

fn handle_tray_event<R: Runtime>(app: &AppHandle<R>, id: &str) {
    match id {
        "toggle_monitoring" => {
            let app_handle = app.clone();
            tauri::async_runtime::spawn(async move {
                let state = app_handle.state::<AppState>();
                
                // Get current status
                let current_status = *state.monitoring.lock().await;
                
                // Toggle the actual monitoring
                let result = if !current_status {
                    // Start monitoring
                    let mut monitor_guard = state.clipboard_monitor.lock().await;
                    if let Some(monitor) = monitor_guard.as_mut() {
                        if let Err(e) = monitor.start().await {
                            log::error!("Failed to start monitoring: {}", e);
                            return;
                        }
                    }
                    let mut monitoring = state.monitoring.lock().await;
                    *monitoring = true;
                    true
                } else {
                    // Stop monitoring
                    let mut monitor_guard = state.clipboard_monitor.lock().await;
                    if let Some(monitor) = monitor_guard.as_mut() {
                        if let Err(e) = monitor.stop().await {
                            log::error!("Failed to stop monitoring: {}", e);
                            return;
                        }
                    }
                    let mut monitoring = state.monitoring.lock().await;
                    *monitoring = false;
                    false
                };
                
                log::info!("Monitoring toggled: {}", result);
                
                // Update tray menu
                if let Some(tray) = app_handle.tray_by_id("tray") {
                    if let Ok(menu) = create_tray_menu(&app_handle) {
                        let _ = tray.set_menu(Some(menu));
                    }
                }
                
                // Emit event to frontend
                let _ = app_handle.emit("monitoring-changed", result);
            });
        }
        "open_settings" => {
            log::info!("Opening settings window");
            
            // Get or create the main window
            match app.get_webview_window("main") {
                Some(window) => {
                    log::debug!("Found existing main window, showing it");
                    // Make sure the window is visible and focused
                    if let Err(e) = window.show() {
                        log::error!("Failed to show window: {}", e);
                    }
                    if let Err(e) = window.unminimize() {
                        log::error!("Failed to unminimize window: {}", e);
                    }
                    if let Err(e) = window.set_focus() {
                        log::error!("Failed to focus window: {}", e);
                    }
                }
                None => {
                    log::error!("Main window not found - it should be created at startup with label 'main'");
                    // Try to create a new window as fallback
                    if let Err(e) = tauri::WebviewWindowBuilder::new(
                        app,
                        "main",
                        tauri::WebviewUrl::App("index.html".into())
                    )
                    .title("Shotpipe")
                    .inner_size(800.0, 600.0)
                    .build() {
                        log::error!("Failed to create new window: {}", e);
                    }
                }
            }
        }
        "quit" => {
            app.exit(0);
        }
        _ => {
            log::debug!("Unknown tray event: {}", id);
        }
    }
}

pub fn update_tray_tooltip<R: Runtime>(
    app: &AppHandle<R>,
    monitoring: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(tray) = app.tray_by_id("tray") {
        let tooltip = if monitoring {
            "Shotpipe - Monitoring clipboard"
        } else {
            "Shotpipe - Monitoring stopped"
        };
        tray.set_tooltip(Some(tooltip))?;
    }
    Ok(())
}