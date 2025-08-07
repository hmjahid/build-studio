use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;
use tauri::State;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualizationSupport {
    pub docker: bool,
    pub wsl: bool,
    pub kvm: bool,
    pub qemu: bool,
    pub hyperv: bool,
    pub vmware: bool,
    pub virtualbox: bool,
    pub macos_vm: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub memory: u64,
    pub cpu_cores: u32,
    pub virtualization_support: VirtualizationSupport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalNodeConfig {
    pub name: String,
    pub platform: String,
    pub memory: u32,
    pub cpu_cores: u32,
    pub disk_size: u32,
    pub virtualization: String,
    pub capabilities: Vec<String>,
    pub languages: Vec<String>,
    pub install_build_tools: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalNode {
    pub id: String,
    pub name: String,
    pub node_type: String, // "local-docker", "local-vm"
    pub status: String,
    pub platform: String,
    pub memory: u32,
    pub cpu_cores: u32,
    pub disk_size: u32,
    pub vm_type: String,
    pub capabilities: Vec<String>,
    pub container_id: Option<String>,
    pub vm_id: Option<String>,
    pub created_at: String,
    pub last_seen: Option<String>,
}

pub struct LocalNodeManager {
    nodes: Mutex<HashMap<String, LocalNode>>,
}

impl LocalNodeManager {
    pub fn new() -> Self {
        Self {
            nodes: Mutex::new(HashMap::new()),
        }
    }
}

#[tauri::command]
pub async fn detect_virtualization_support() -> Result<Vec<String>, String> {
    let mut supported = Vec::new();
    
    // Check Docker
    if check_docker_available() {
        supported.push("docker".to_string());
    }
    
    // Check WSL (Windows only)
    #[cfg(target_os = "windows")]
    if check_wsl_available() {
        supported.push("wsl".to_string());
    }
    
    // Check KVM (Linux only)
    #[cfg(target_os = "linux")]
    if check_kvm_available() {
        supported.push("kvm".to_string());
    }
    
    // Check QEMU
    if check_qemu_available() {
        supported.push("qemu".to_string());
    }
    
    // Check Hyper-V (Windows only)
    #[cfg(target_os = "windows")]
    if check_hyperv_available() {
        supported.push("hyperv".to_string());
    }
    
    // Check VMware
    if check_vmware_available() {
        supported.push("vmware".to_string());
    }
    
    // Check VirtualBox
    if check_virtualbox_available() {
        supported.push("virtualbox".to_string());
    }
    
    // Check macOS VM support (macOS only)
    #[cfg(target_os = "macos")]
    if check_macos_vm_available() {
        supported.push("macos-vm".to_string());
    }
    
    Ok(supported)
}

#[tauri::command]
pub async fn get_system_info() -> Result<SystemInfo, String> {
    let os = std::env::consts::OS.to_string();
    let arch = std::env::consts::ARCH.to_string();
    
    // Get system memory (simplified)
    let memory = get_system_memory();
    let cpu_cores = num_cpus::get() as u32;
    
    let virtualization_support = VirtualizationSupport {
        docker: check_docker_available(),
        wsl: cfg!(target_os = "windows") && check_wsl_available(),
        kvm: cfg!(target_os = "linux") && check_kvm_available(),
        qemu: check_qemu_available(),
        hyperv: cfg!(target_os = "windows") && check_hyperv_available(),
        vmware: check_vmware_available(),
        virtualbox: check_virtualbox_available(),
        macos_vm: cfg!(target_os = "macos") && check_macos_vm_available(),
    };
    
    Ok(SystemInfo {
        os,
        arch,
        memory,
        cpu_cores,
        virtualization_support,
    })
}

#[tauri::command]
pub async fn create_local_node(
    config: LocalNodeConfig,
    state: State<'_, LocalNodeManager>,
) -> Result<String, String> {
    let node_id = uuid::Uuid::new_v4().to_string();
    
    match config.virtualization.as_str() {
        "docker" => create_docker_node(&node_id, &config).await,
        "wsl" => create_wsl_node(&node_id, &config).await,
        "kvm" => create_kvm_node(&node_id, &config).await,
        "qemu" => create_qemu_node(&node_id, &config).await,
        "hyperv" => create_hyperv_node(&node_id, &config).await,
        "vmware" => create_vmware_node(&node_id, &config).await,
        "virtualbox" => create_virtualbox_node(&node_id, &config).await,
        "macos-vm" => create_macos_vm_node(&node_id, &config).await,
        "auto" => create_auto_node(&node_id, &config).await,
        _ => Err("Unsupported virtualization technology".to_string()),
    }?;
    
    let node = LocalNode {
        id: node_id.clone(),
        name: config.name,
        node_type: if config.virtualization == "docker" { "local-docker".to_string() } else { "local-vm".to_string() },
        status: "installing".to_string(),
        platform: config.platform,
        memory: config.memory,
        cpu_cores: config.cpu_cores,
        disk_size: config.disk_size,
        vm_type: config.virtualization,
        capabilities: config.capabilities,
        container_id: None,
        vm_id: None,
        created_at: chrono::Utc::now().to_rfc3339(),
        last_seen: None,
    };
    
    state.nodes.lock().await.insert(node_id.clone(), node);
    
    Ok(node_id)
}

#[tauri::command]
pub async fn list_build_nodes(
    state: State<'_, LocalNodeManager>,
) -> Result<Vec<LocalNode>, String> {
    let nodes = state.nodes.lock().await;
    Ok(nodes.values().cloned().collect())
}

#[tauri::command]
pub async fn start_node(
    id: String,
    state: State<'_, LocalNodeManager>,
) -> Result<(), String> {
    let mut nodes = state.nodes.lock().await;
    if let Some(node) = nodes.get_mut(&id) {
        match node.vm_type.as_str() {
            "docker" => start_docker_node(node).await,
            "wsl" => start_wsl_node(node).await,
            "kvm" => start_kvm_node(node).await,
            "qemu" => start_qemu_node(node).await,
            _ => Err("Unsupported node type for start operation".to_string()),
        }
    } else {
        Err("Node not found".to_string())
    }
}

#[tauri::command]
pub async fn stop_node(
    id: String,
    state: State<'_, LocalNodeManager>,
) -> Result<(), String> {
    let mut nodes = state.nodes.lock().await;
    if let Some(node) = nodes.get_mut(&id) {
        match node.vm_type.as_str() {
            "docker" => stop_docker_node(node).await,
            "wsl" => stop_wsl_node(node).await,
            "kvm" => stop_kvm_node(node).await,
            "qemu" => stop_qemu_node(node).await,
            _ => Err("Unsupported node type for stop operation".to_string()),
        }
    } else {
        Err("Node not found".to_string())
    }
}

#[tauri::command]
pub async fn remove_node(
    id: String,
    state: State<'_, LocalNodeManager>,
) -> Result<(), String> {
    let mut nodes = state.nodes.lock().await;
    if let Some(node) = nodes.remove(&id) {
        match node.vm_type.as_str() {
            "docker" => remove_docker_node(&node).await,
            "wsl" => remove_wsl_node(&node).await,
            "kvm" => remove_kvm_node(&node).await,
            "qemu" => remove_qemu_node(&node).await,
            _ => Ok(()),
        }
    } else {
        Err("Node not found".to_string())
    }
}

#[tauri::command]
pub async fn scan_local_nodes() -> Result<Vec<LocalNode>, String> {
    let mut discovered_nodes = Vec::new();
    
    // Scan Docker containers
    if let Ok(docker_nodes) = scan_docker_containers().await {
        discovered_nodes.extend(docker_nodes);
    }
    
    // Scan VMs
    if let Ok(vm_nodes) = scan_virtual_machines().await {
        discovered_nodes.extend(vm_nodes);
    }
    
    Ok(discovered_nodes)
}

// Helper functions for checking virtualization support
fn check_docker_available() -> bool {
    Command::new("docker")
        .args(&["--version"])
        .output()
        .is_ok()
}

#[cfg(target_os = "windows")]
fn check_wsl_available() -> bool {
    Command::new("wsl")
        .args(&["--status"])
        .output()
        .is_ok()
}

#[cfg(target_os = "windows")]
fn check_wsl_available() -> bool {
    Command::new("wsl")
        .args(&["--status"])
        .output()
        .is_ok()
}

#[cfg(not(target_os = "windows"))]
fn check_wsl_available() -> bool {
    false
}

#[cfg(target_os = "linux")]
fn check_kvm_available() -> bool {
    std::path::Path::new("/dev/kvm").exists()
}

#[cfg(not(target_os = "linux"))]
fn check_kvm_available() -> bool {
    false
}

fn check_qemu_available() -> bool {
    Command::new("qemu-system-x86_64")
        .args(&["--version"])
        .output()
        .is_ok()
}

#[cfg(target_os = "windows")]
fn check_hyperv_available() -> bool {
    // Check if Hyper-V is enabled (simplified check)
    Command::new("powershell")
        .args(&["-Command", "Get-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V-All"])
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).contains("Enabled"))
        .unwrap_or(false)
}

#[cfg(not(target_os = "windows"))]
fn check_hyperv_available() -> bool {
    false
}

fn check_vmware_available() -> bool {
    Command::new("vmrun")
        .args(&["-T", "ws", "list"])
        .output()
        .is_ok()
}

fn check_virtualbox_available() -> bool {
    Command::new("VBoxManage")
        .args(&["--version"])
        .output()
        .is_ok()
}

#[cfg(target_os = "macos")]
fn check_macos_vm_available() -> bool {
    // Check for macOS virtualization framework
    std::path::Path::new("/System/Library/Frameworks/Virtualization.framework").exists()
}

#[cfg(not(target_os = "macos"))]
fn check_macos_vm_available() -> bool {
    false
}

fn get_system_memory() -> u64 {
    // Simplified memory detection
    #[cfg(target_os = "linux")]
    {
        if let Ok(meminfo) = std::fs::read_to_string("/proc/meminfo") {
            for line in meminfo.lines() {
                if line.starts_with("MemTotal:") {
                    if let Some(kb) = line.split_whitespace().nth(1) {
                        if let Ok(kb_val) = kb.parse::<u64>() {
                            return kb_val / 1024; // Convert to MB
                        }
                    }
                }
            }
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        // Use WMI or system calls for Windows
        return 8192; // Default fallback
    }
    
    #[cfg(target_os = "macos")]
    {
        // Use sysctl for macOS
        return 8192; // Default fallback
    }
    
    8192 // Default fallback
}

// Docker node management
async fn create_docker_node(node_id: &str, config: &LocalNodeConfig) -> Result<(), String> {
    let image_name = match config.platform.as_str() {
        "ubuntu-22.04" => "ubuntu:22.04",
        "ubuntu-20.04" => "ubuntu:20.04",
        "debian-11" => "debian:11",
        "alpine-3.18" => "alpine:3.18",
        "centos-8" => "centos:8",
        _ => "ubuntu:22.04",
    };
    
    let container_name = format!("build-studio-{}", node_id);
    
    // Pull the image
    let pull_output = Command::new("docker")
        .args(&["pull", image_name])
        .output()
        .map_err(|e| format!("Failed to pull Docker image: {}", e))?;
    
    if !pull_output.status.success() {
        return Err("Failed to pull Docker image".to_string());
    }
    
    // Create and start container
    let memory_arg = format!("{}m", config.memory);
    let cpu_arg = config.cpu_cores.to_string();
    let volume_arg = format!("/tmp/build-studio-{}:/workspace", node_id);
    
    let run_args = vec![
        "run", "-d", "--name", &container_name,
        "--memory", &memory_arg,
        "--cpus", &cpu_arg,
        "-v", &volume_arg,
        image_name,
        "sleep", "infinity"
    ];
    
    let run_output = Command::new("docker")
        .args(&run_args)
        .output()
        .map_err(|e| format!("Failed to create Docker container: {}", e))?;
    
    if !run_output.status.success() {
        return Err("Failed to create Docker container".to_string());
    }
    
    // Install build tools if requested
    if config.install_build_tools {
        install_docker_build_tools(&container_name, &config.languages).await?;
    }
    
    Ok(())
}

async fn install_docker_build_tools(container_name: &str, languages: &[String]) -> Result<(), String> {
    // Update package manager
    let update_cmd = vec!["exec", container_name, "apt", "update"];
    Command::new("docker")
        .args(&update_cmd)
        .output()
        .map_err(|e| format!("Failed to update packages: {}", e))?;
    
    // Install basic build tools
    let basic_tools = vec!["exec", container_name, "apt", "install", "-y", "build-essential", "git", "curl", "wget"];
    Command::new("docker")
        .args(&basic_tools)
        .output()
        .map_err(|e| format!("Failed to install basic tools: {}", e))?;
    
    // Install language-specific tools
    for language in languages {
        match language.as_str() {
            "rust" => {
                let rust_cmd = vec!["exec", container_name, "sh", "-c", 
                    "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"];
                Command::new("docker").args(&rust_cmd).output().ok();
            },
            "node" => {
                let node_cmd = vec!["exec", container_name, "apt", "install", "-y", "nodejs", "npm"];
                Command::new("docker").args(&node_cmd).output().ok();
            },
            "python" => {
                let python_cmd = vec!["exec", container_name, "apt", "install", "-y", "python3", "python3-pip"];
                Command::new("docker").args(&python_cmd).output().ok();
            },
            "java" => {
                let java_cmd = vec!["exec", container_name, "apt", "install", "-y", "openjdk-11-jdk"];
                Command::new("docker").args(&java_cmd).output().ok();
            },
            "go" => {
                let go_cmd = vec!["exec", container_name, "apt", "install", "-y", "golang-go"];
                Command::new("docker").args(&go_cmd).output().ok();
            },
            _ => {}
        }
    }
    
    Ok(())
}

// Placeholder implementations for other virtualization technologies
async fn create_wsl_node(_node_id: &str, _config: &LocalNodeConfig) -> Result<(), String> {
    // WSL implementation would go here
    Err("WSL node creation not yet implemented".to_string())
}

async fn create_kvm_node(_node_id: &str, _config: &LocalNodeConfig) -> Result<(), String> {
    // KVM implementation would go here
    Err("KVM node creation not yet implemented".to_string())
}

async fn create_qemu_node(_node_id: &str, _config: &LocalNodeConfig) -> Result<(), String> {
    // QEMU implementation would go here
    Err("QEMU node creation not yet implemented".to_string())
}

async fn create_hyperv_node(_node_id: &str, _config: &LocalNodeConfig) -> Result<(), String> {
    // Hyper-V implementation would go here
    Err("Hyper-V node creation not yet implemented".to_string())
}

async fn create_vmware_node(_node_id: &str, _config: &LocalNodeConfig) -> Result<(), String> {
    // VMware implementation would go here
    Err("VMware node creation not yet implemented".to_string())
}

async fn create_virtualbox_node(_node_id: &str, _config: &LocalNodeConfig) -> Result<(), String> {
    // VirtualBox implementation would go here
    Err("VirtualBox node creation not yet implemented".to_string())
}

async fn create_macos_vm_node(_node_id: &str, _config: &LocalNodeConfig) -> Result<(), String> {
    // macOS VM implementation would go here
    Err("macOS VM node creation not yet implemented".to_string())
}

async fn create_auto_node(node_id: &str, config: &LocalNodeConfig) -> Result<(), String> {
    // Auto-select best available virtualization
    if check_docker_available() {
        let mut docker_config = config.clone();
        docker_config.virtualization = "docker".to_string();
        create_docker_node(node_id, &docker_config).await
    } else {
        Err("No suitable virtualization technology available".to_string())
    }
}

// Node control functions (simplified implementations)
async fn start_docker_node(node: &mut LocalNode) -> Result<(), String> {
    let container_name = format!("build-studio-{}", node.id);
    let output = Command::new("docker")
        .args(&["start", &container_name])
        .output()
        .map_err(|e| format!("Failed to start Docker container: {}", e))?;
    
    if output.status.success() {
        node.status = "online".to_string();
        node.last_seen = Some(chrono::Utc::now().to_rfc3339());
        Ok(())
    } else {
        Err("Failed to start Docker container".to_string())
    }
}

async fn stop_docker_node(node: &mut LocalNode) -> Result<(), String> {
    let container_name = format!("build-studio-{}", node.id);
    let output = Command::new("docker")
        .args(&["stop", &container_name])
        .output()
        .map_err(|e| format!("Failed to stop Docker container: {}", e))?;
    
    if output.status.success() {
        node.status = "offline".to_string();
        Ok(())
    } else {
        Err("Failed to stop Docker container".to_string())
    }
}

async fn remove_docker_node(node: &LocalNode) -> Result<(), String> {
    let container_name = format!("build-studio-{}", node.id);
    Command::new("docker")
        .args(&["rm", "-f", &container_name])
        .output()
        .map_err(|e| format!("Failed to remove Docker container: {}", e))?;
    
    Ok(())
}

// Placeholder implementations for other VM types
async fn start_wsl_node(_node: &mut LocalNode) -> Result<(), String> {
    Err("WSL node start not yet implemented".to_string())
}

async fn stop_wsl_node(_node: &mut LocalNode) -> Result<(), String> {
    Err("WSL node stop not yet implemented".to_string())
}

async fn remove_wsl_node(_node: &LocalNode) -> Result<(), String> {
    Err("WSL node removal not yet implemented".to_string())
}

async fn start_kvm_node(_node: &mut LocalNode) -> Result<(), String> {
    Err("KVM node start not yet implemented".to_string())
}

async fn stop_kvm_node(_node: &mut LocalNode) -> Result<(), String> {
    Err("KVM node stop not yet implemented".to_string())
}

async fn remove_kvm_node(_node: &LocalNode) -> Result<(), String> {
    Err("KVM node removal not yet implemented".to_string())
}

async fn start_qemu_node(_node: &mut LocalNode) -> Result<(), String> {
    Err("QEMU node start not yet implemented".to_string())
}

async fn stop_qemu_node(_node: &mut LocalNode) -> Result<(), String> {
    Err("QEMU node stop not yet implemented".to_string())
}

async fn remove_qemu_node(_node: &LocalNode) -> Result<(), String> {
    Err("QEMU node removal not yet implemented".to_string())
}

// Node discovery functions
async fn scan_docker_containers() -> Result<Vec<LocalNode>, String> {
    let output = Command::new("docker")
        .args(&["ps", "-a", "--format", "{{.ID}}\t{{.Names}}\t{{.Image}}\t{{.Status}}"])
        .output()
        .map_err(|e| format!("Failed to list Docker containers: {}", e))?;
    
    if !output.status.success() {
        return Ok(Vec::new());
    }
    
    let containers_output = String::from_utf8_lossy(&output.stdout);
    let mut nodes = Vec::new();
    
    for line in containers_output.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 4 && parts[1].starts_with("build-studio-") {
            // This is one of our containers
            let node = LocalNode {
                id: parts[0].to_string(),
                name: parts[1].to_string(),
                node_type: "local-docker".to_string(),
                status: if parts[3].contains("Up") { "online".to_string() } else { "offline".to_string() },
                platform: "unknown".to_string(),
                memory: 0,
                cpu_cores: 0,
                disk_size: 0,
                vm_type: "docker".to_string(),
                capabilities: vec!["build".to_string()],
                container_id: Some(parts[0].to_string()),
                vm_id: None,
                created_at: chrono::Utc::now().to_rfc3339(),
                last_seen: None,
            };
            nodes.push(node);
        }
    }
    
    Ok(nodes)
}

async fn scan_virtual_machines() -> Result<Vec<LocalNode>, String> {
    // This would scan for VMs using various hypervisors
    // For now, return empty list
    Ok(Vec::new())
}
