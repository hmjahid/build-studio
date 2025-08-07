
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Clone, Serialize, Deserialize)]
pub struct Plugin {
    pub name: String,
    pub author: String,
    pub version: String,
    pub description: String,
    pub path: String,
}

#[tauri::command]
pub fn list_plugins(plugin_dir: String) -> Vec<Plugin> {
    let mut plugins = Vec::new();
    
    // Try to read the plugins directory
    if let Ok(entries) = fs::read_dir(&plugin_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                
                // Check if it's a directory
                if path.is_dir() {
                    // Look for a plugin manifest file (plugin.json or plugin.yaml)
                    let plugin_json = path.join("plugin.json");
                    let plugin_yaml = path.join("plugin.yaml");
                    
                    let plugin_info = if plugin_json.exists() {
                        // Parse JSON manifest
                        if let Ok(content) = fs::read_to_string(&plugin_json) {
                            serde_json::from_str(&content).unwrap_or_else(|_| {
                                Plugin {
                                    name: path.file_name().unwrap().to_string_lossy().to_string(),
                                    author: "Unknown".to_string(),
                                    version: "0.1.0".to_string(),
                                    description: "Plugin description not available".to_string(),
                                    path: path.to_string_lossy().to_string(),
                                }
                            })
                        } else {
                            Plugin {
                                name: path.file_name().unwrap().to_string_lossy().to_string(),
                                author: "Unknown".to_string(),
                                version: "0.1.0".to_string(),
                                description: "Plugin description not available".to_string(),
                                path: path.to_string_lossy().to_string(),
                            }
                        }
                    } else if plugin_yaml.exists() {
                        // Parse YAML manifest
                        if let Ok(content) = fs::read_to_string(&plugin_yaml) {
                            serde_yaml::from_str(&content).unwrap_or_else(|_| {
                                Plugin {
                                    name: path.file_name().unwrap().to_string_lossy().to_string(),
                                    author: "Unknown".to_string(),
                                    version: "0.1.0".to_string(),
                                    description: "Plugin description not available".to_string(),
                                    path: path.to_string_lossy().to_string(),
                                }
                            })
                        } else {
                            Plugin {
                                name: path.file_name().unwrap().to_string_lossy().to_string(),
                                author: "Unknown".to_string(),
                                version: "0.1.0".to_string(),
                                description: "Plugin description not available".to_string(),
                                path: path.to_string_lossy().to_string(),
                            }
                        }
                    } else {
                        // No manifest file, create a default plugin entry
                        Plugin {
                            name: path.file_name().unwrap().to_string_lossy().to_string(),
                            author: "Unknown".to_string(),
                            version: "0.1.0".to_string(),
                            description: "Plugin description not available".to_string(),
                            path: path.to_string_lossy().to_string(),
                        }
                    };
                    
                    plugins.push(plugin_info);
                }
            }
        }
    }
    
    plugins
}
