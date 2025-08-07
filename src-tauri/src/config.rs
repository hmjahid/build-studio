use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BuildConfig {
    pub name: String,
    pub platform: String,
    pub language: Option<String>,
    pub command: String,
    pub container: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PackageConfig {
    pub r#type: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub dependencies: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BuildStudioConfig {
    pub builds: Vec<BuildConfig>,
    pub package: Option<PackageConfig>,
}

#[tauri::command]
pub fn read_config(path: String) -> Result<BuildStudioConfig, String> {
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_yaml::from_str(&content).map_err(|e| e.to_string())
}
