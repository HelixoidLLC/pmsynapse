//! PMSynapse Desktop Application
//!
//! Tauri-based desktop application for PMSynapse.

use tauri::Manager;
use std::fs;
use std::path::PathBuf;

/// Get daemon PID file path
fn get_daemon_pid_path() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".pmsynapse").join("daemon.pid")
}

/// Read daemon port from PID file
fn read_daemon_port_from_file() -> Option<String> {
    let pid_path = get_daemon_pid_path();

    if !pid_path.exists() {
        eprintln!("Daemon PID file not found at: {:?}", pid_path);
        return None;
    }

    match fs::read_to_string(&pid_path) {
        Ok(content) => {
            let content = content.trim();
            eprintln!("Read daemon PID file: {}", content);

            // Handle "pid:port" format
            if content.contains(':') {
                let parts: Vec<&str> = content.split(':').collect();
                if parts.len() >= 2 {
                    let port = parts[1].to_string();
                    eprintln!("Extracted port from PID file: {}", port);
                    return Some(port);
                }
            }
            None
        }
        Err(e) => {
            eprintln!("Failed to read daemon PID file: {}", e);
            None
        }
    }
}

/// Get daemon URL - tries PID file first, then env var, then default
fn get_daemon_url() -> String {
    // 1. Try reading from daemon.pid file
    let port = read_daemon_port_from_file()
        // 2. Fall back to environment variable
        .or_else(|| std::env::var("PMSYNAPSE_DAEMON_PORT").ok())
        // 3. Fall back to default port
        .unwrap_or_else(|| "7878".to_string());

    let url = format!("http://127.0.0.1:{}/api/v1", port);
    eprintln!("Daemon URL: {}", url);
    url
}

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

/// Get knowledge graph from daemon
#[tauri::command]
async fn get_knowledge_graph() -> Result<String, String> {
    let url = format!("{}/nodes", get_daemon_url());
    eprintln!("Fetching nodes from: {}", url);
    let response = reqwest::get(&url)
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch nodes: {}", e);
            format!("Failed to connect to daemon: {}. Is the daemon running?", e)
        })?
        .text()
        .await
        .map_err(|e| e.to_string())?;
    eprintln!("Received response: {}", response);
    Ok(response)
}

/// Create a node via daemon
#[tauri::command]
async fn create_node(node_type: String, title: String, content: Option<String>) -> Result<String, String> {
    let url = format!("{}/nodes", get_daemon_url());
    let body = serde_json::json!({
        "node_type": node_type,
        "title": title,
        "content": content
    });

    let client = reqwest::Client::new();
    let response = client.post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;
    Ok(response)
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
            get_knowledge_graph,
            create_node,
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
