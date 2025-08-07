use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;
use tauri::{Window, Emitter};
use crate::toolchain::Toolchain;
use crate::security::{self, SecurityConfig};

fn get_toolchain_for_platform(platform: &str) -> Toolchain {
    match platform {
        "windows" => Toolchain::MinGW,
        "wasm" | "webassembly" => Toolchain::WasmPack,
        "android" => Toolchain::AndroidNDK,
        _ => Toolchain::Native,
    }
}

#[tauri::command]
pub async fn run_build(window: Window, command: String, cwd: String, platform: Option<String>) -> Result<(), String> {
    // Load default security config
    let security_config = SecurityConfig::default();
    
    // Validate command against security policy
    if !security::validate_command(&command, &security_config) {
        return Err("Command blocked by security policy".to_string());
    }
    
    // Create sandboxed environment
    let sandbox_dir = security::create_sandbox(&cwd, &security_config)
        .map_err(|e| format!("Failed to create sandbox: {}", e))?;
    
    let toolchain = platform
        .as_ref()
        .map(|p| get_toolchain_for_platform(p))
        .unwrap_or(Toolchain::Native);
    let prefix = toolchain.command_prefix();
    let full_command = if prefix.is_empty() {
        command.clone()
    } else {
        format!("{}{}", prefix.join(""), command)
    };

    let mut cmd = if cfg!(target_os = "windows") {
        let mut c = Command::new("cmd");
        c.arg("/C").arg(&full_command);
        c
    } else {
        let mut c = Command::new("sh");
        c.arg("-c").arg(&full_command);
        c
    };

    // Set the working directory to sandbox
    cmd.current_dir(&sandbox_dir);

    // Set up stdout and stderr to be piped
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    // Spawn the command
    let mut child = cmd.spawn().map_err(|e| format!("Failed to execute command: {}", e))?;

    // Get stdout and stderr
    let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
    let stderr = child.stderr.take().ok_or("Failed to capture stderr")?;

    let window_clone = window.clone();

    // Spawn a thread to read stdout
    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(content) => {
                    let _ = window_clone.emit("build-output", content);
                }
                Err(e) => {
                    let _ = window_clone.emit("build-error", format!("Error reading stdout: {}", e));
                }
            }
        }
    });

    let window_clone = window.clone();

    // Spawn a thread to read stderr
    thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            match line {
                Ok(content) => {
                    let _ = window_clone.emit("build-error", content);
                }
                Err(e) => {
                    let _ = window_clone.emit("build-error", format!("Error reading stderr: {}", e));
                }
            }
        }
    });

    // Wait for the command to complete
    let status = child.wait().map_err(|e| format!("Failed to wait for command: {}", e))?;

    // Clean up sandbox
    if let Err(e) = security::cleanup_sandbox(&sandbox_dir) {
        let _ = window.emit("build-error", format!("Failed to clean up sandbox: {}", e));
    }

    if status.success() {
        window.emit("build-finished", Some("success".to_string())).ok();
        Ok(())
    } else {
        window.emit("build-finished", Some("failure".to_string())).ok();
        Err(format!("Command exited with status: {}", status))
    }
}

/// Synchronous build runner for CLI (no Tauri window)
pub fn run_build_no_window(command: String, cwd: String, platform: Option<String>) -> Result<(), String> {
    use std::process::{Command, Stdio};
    use std::io::{BufRead, BufReader};
    let toolchain = platform
        .as_ref()
        .map(|p| get_toolchain_for_platform(p))
        .unwrap_or(Toolchain::Native);
    let prefix = toolchain.command_prefix();
    let full_command = if prefix.is_empty() {
        command.clone()
    } else {
        format!("{}{}", prefix.join(""), command)
    };

    let mut cmd = if cfg!(target_os = "windows") {
        let mut c = Command::new("cmd");
        c.args(["/C", &full_command]);
        c
    } else {
        let mut c = Command::new("sh");
        c.args(["-c", &full_command]);
        c
    };
    cmd.current_dir(&cwd);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn().map_err(|e| e.to_string())?;
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    // Print stdout
    let stdout_reader = BufReader::new(stdout);
    for line in stdout_reader.lines() {
        if let Ok(line) = line {
            println!("{}", line);
        }
    }
    // Print stderr
    let stderr_reader = BufReader::new(stderr);
    for line in stderr_reader.lines() {
        if let Ok(line) = line {
            eprintln!("{}", line);
        }
    }

    let status = child.wait().map_err(|e| e.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err("Build failed".to_string())
    }
}
