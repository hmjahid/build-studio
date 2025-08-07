//! Security module for Build Studio
//! Provides sandboxing and security features for builds

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

/// Security configuration for builds
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SecurityConfig {
    /// Enable sandboxing for builds
    pub enable_sandbox: bool,
    /// Allowed directories for file access
    pub allowed_paths: Vec<String>,
    /// Blocked system commands
    pub blocked_commands: Vec<String>,
    /// Enable network isolation
    pub network_isolation: bool,
    /// Maximum build time in seconds
    pub max_build_time: Option<u32>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_sandbox: true,
            allowed_paths: vec!["./src".to_string(), "./builds".to_string(), "./packages".to_string()],
            blocked_commands: vec![
                "rm".to_string(),
                "rmdir".to_string(),
                "mv".to_string(),
                "cp".to_string(),
                "chmod".to_string(),
                "chown".to_string(),
            ],
            network_isolation: true,
            max_build_time: Some(3600), // 1 hour default
        }
    }
}

/// Validate that a command is not blocked
pub fn validate_command(command: &str, config: &SecurityConfig) -> bool {
    if !config.enable_sandbox {
        return true;
    }
    
    // Check if command is in blocked list
    for blocked in &config.blocked_commands {
        if command.contains(blocked) {
            return false;
        }
    }
    
    true
}

/// Create a sandboxed environment for builds
pub fn create_sandbox(project_dir: &str, config: &SecurityConfig) -> Result<String, String> {
    if !config.enable_sandbox {
        return Ok(project_dir.to_string());
    }
    
    // Create a temporary directory for the sandbox
    let sandbox_dir = format!("{}/.sandbox", project_dir);
    
    // Create the sandbox directory
    fs::create_dir_all(&sandbox_dir)
        .map_err(|e| format!("Failed to create sandbox directory: {}", e))?;
    
    // Copy allowed files to sandbox
    for allowed_path in &config.allowed_paths {
        let src_path = format!("{}/{}", project_dir, allowed_path);
        let dest_path = format!("{}/{}", sandbox_dir, allowed_path);
        
        if Path::new(&src_path).exists() {
            // Create parent directory if needed
            if let Some(parent) = Path::new(&dest_path).parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create directory: {}", e))?;
            }
            
            // Copy file or directory
            if Path::new(&src_path).is_dir() {
                fs_extra::dir::copy(&src_path, &dest_path, &fs_extra::dir::CopyOptions::new())
                    .map_err(|e| format!("Failed to copy directory: {}", e))?;
            } else {
                fs::copy(&src_path, &dest_path)
                    .map_err(|e| format!("Failed to copy file: {}", e))?;
            }
        }
    }
    
    Ok(sandbox_dir)
}

/// Clean up sandbox environment
pub fn cleanup_sandbox(sandbox_dir: &str) -> Result<(), String> {
    if Path::new(sandbox_dir).exists() {
        fs::remove_dir_all(sandbox_dir)
            .map_err(|e| format!("Failed to remove sandbox directory: {}", e))?;
    }
    Ok(())
}
