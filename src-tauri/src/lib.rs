use tauri::Manager;
use tauri::State;

mod project;
pub mod config;
pub mod build;
pub use crate::build::run_build_no_window;
pub mod plugin;
pub mod packaging;
mod cicd;
mod remotenode;
mod toolchain;
mod fsutils;
mod security;
mod local_nodes;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(cicd::WebhookState::default())
        .manage(remotenode::RemoteNodeManager::default())
        .manage(std::sync::Mutex::new(project::ProjectManager::default()))
        .manage(local_nodes::LocalNodeManager::new())
        .setup(|app| {
            let handle = app.handle();
            let project_manager_state = handle.state::<std::sync::Mutex<project::ProjectManager>>();
            let mut project_manager = project_manager_state.lock().unwrap();
            
            let config_dir = app.path()
                .app_config_dir()
                .expect("Failed to get app config dir");

            if !config_dir.exists() {
                std::fs::create_dir_all(&config_dir).expect("Failed to create app config dir");
            }
            
            project_manager.load_projects(&config_dir);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_app_version,
            project::list_projects,
            project::add_project,
            project::remove_project,
            config::read_config,
            build::run_build,
            // plugin::list_plugins, // This will be called with plugin directory from frontend
            packaging::create_package,
            cicd::add_webhook,
            cicd::list_webhooks,
            cicd::trigger_webhook,
            remotenode::add_remote_node,
            remotenode::list_remote_nodes,
            fsutils::read_text_file,
            fsutils::write_text_file,
            local_nodes::detect_virtualization_support,
            local_nodes::get_system_info,
            local_nodes::create_local_node,
            local_nodes::list_build_nodes,
            local_nodes::start_node,
            local_nodes::stop_node,
            local_nodes::remove_node,
            local_nodes::scan_local_nodes
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
