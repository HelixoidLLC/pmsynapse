//! PMSynapse Desktop Application
//!
//! Tauri-based desktop application for PMSynapse.

use tauri::Manager;

/// Get the version of the application
#[tauri::command]
fn get_version() -> String {
    snps_core::VERSION.to_string()
}

/// Initialize PMSynapse
#[tauri::command]
fn init_synapse() -> Result<String, String> {
    snps_core::init()
        .map(|_| "PMSynapse initialized successfully".to_string())
        .map_err(|e| e.to_string())
}

/// Get IDLC configuration
#[tauri::command]
fn get_idlc_config() -> Result<String, String> {
    let config = snps_core::idlc::IdlcConfig::default_config();
    serde_json::to_string(&config).map_err(|e| e.to_string())
}

/// Create a new IDLC item
#[tauri::command]
fn create_idlc_item(title: String) -> Result<String, String> {
    let item = snps_core::idlc::IdlcItem::new(&title);
    serde_json::to_string(&item).map_err(|e| e.to_string())
}

/// Run the Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            get_version,
            init_synapse,
            get_idlc_config,
            create_idlc_item,
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
