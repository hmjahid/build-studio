# Build Studio

**Build Studio** is an advanced, extensible desktop and CLI application for configuring, managing, and executing multi-platform software builds from a unified interface. Built with Tauri, SvelteKit, and Rust.

![Build Studio](https://placehold.co/800x400?text=Build+Studio+Interface)

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/hmjahid/build-studio)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-lightgrey)](https://github.com/hmajhid/build-studio)

---

## Features

### Core Features
- **Unified Interface**: Both GUI (Tauri/Svelte) and CLI interfaces
- **Multi-Platform Support**: Linux, Windows, macOS, Android, iOS, WebAssembly
- **Multi-Language Support**: C/C++, Python, Java, Rust, Go, Node.js
- **Configuration Management**: YAML/JSON configuration files (`buildstudio.config.yaml`)
- **Build Execution**: Local shell execution with real-time log streaming
- **Cross-Compilation**: Support for multiple toolchains (MinGW, Android NDK, wasm-pack)

### Advanced Features
- **Plugin System**: Extensible architecture with dynamic plugin discovery
- **Packaging**: Multi-platform package creation (deb, rpm, msi, exe, dmg, pkg, apk, wasm)
- **CI/CD Integration**: Webhook management for GitHub/GitLab with auto-trigger capabilities
- **Remote Build Nodes**: Node management with SSH and HTTP RPC support
- **Security**: Sandboxing for all builds with command validation and path restrictions
- **Offline Capability**: Complete functionality without internet connection
- **Modular Architecture**: Well-organized codebase with separate modules for each feature

---

## Installation

### Prerequisites

Before installing Build Studio, ensure you have the following installed:
- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/) and npm
- [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites/)

For cross-compilation support, you may also need:
- MinGW (for Windows targets on Linux/macOS)
- Android NDK (for Android targets)
- Emscripten (for WebAssembly targets)

### Building from Source

1. Clone the repository:
   ```sh
   git clone https://github.com/your-username/build-studio.git
   cd build-studio
   ```

2. Install frontend dependencies:
   ```sh
   npm install
   ```

3. Build the backend:
   ```sh
   cd src-tauri
   cargo build --release
   cd ..
   ```

4. Run the development version:
   ```sh
   npm run tauri dev
   ```

5. Build for production:
   ```sh
   npm run tauri build
   ```

---

## Usage

### GUI (Desktop App)

The Build Studio GUI provides a comprehensive interface for all features:

- **Project Manager**: Add/remove projects (folders with `buildstudio.config.yaml`)
- **Config Viewer/Editor**: View and edit build configurations
- **Build Runner**: Select and run builds with real-time log streaming
- **Packaging**: Generate platform-specific packages (deb, rpm, msi, etc.)
- **Plugins**: List and manage plugins in the `/plugins` directory
- **Remote Nodes**: Manage remote build nodes for distributed builds
- **CI/CD**: Configure and trigger webhooks for continuous integration

### CLI Interface

Build Studio also provides a powerful CLI for headless operations:

```sh
# Build a project
build-studio build /path/to/project

# Package a project
build-studio package /path/to/project

# List available plugins
build-studio plugins /path/to/project

# Manage remote nodes
build-studio nodes add <node-id> <address>
build-studio nodes list
```

### Configuration

Projects are configured using `buildstudio.config.yaml` files:

```yaml
builds:
  - name: "Linux Build"
    platform: "linux"
    toolchain: "gcc"
    command: "make"
    output: "./target/linux"
  - name: "Windows Build"
    platform: "windows"
    toolchain: "mingw"
    command: "make"
    output: "./target/windows"

package:
  name: "my-app"
  version: "1.0.0"
  description: "My application"
  package_type: "deb"
  output_dir: "./packages"
```

### Plugins

Build Studio supports a plugin system for extending functionality. Plugins are stored in the `/plugins` directory and can be written in Python or JavaScript.

Example plugin structure:
```
plugins/
└── sample-plugin/
    ├── plugin.json
    ├── build.sh
    └── src/
        └── main.py
```

See `docs/plugin-sdk.md` for detailed plugin development documentation.

### Security

Build Studio implements several security features to protect your build environment:

- **Sandboxing**: All builds run in isolated sandbox directories
- **Command Validation**: Dangerous commands are blocked by default
- **Path Restrictions**: File access is restricted to allowed paths
- **Network Isolation**: Network access is disabled during builds by default

See `docs/security.md` for detailed security documentation.
- **Remote Nodes**: Register and use remote build nodes via SSH
- **CI/CD**: Configure webhooks and triggers

## Build Nodes

Build Studio supports multiple types of build nodes for distributed and isolated builds:

### Local Node Deployment

Create isolated build environments on your local machine using various virtualization technologies:

#### Supported Virtualization Technologies

- **Docker**: Lightweight containerized build environments
- **WSL** (Windows): Windows Subsystem for Linux
- **KVM** (Linux): Kernel-based Virtual Machine
- **QEMU**: Cross-platform virtualization
- **Hyper-V** (Windows): Microsoft's hypervisor
- **VMware**: VMware Workstation/Fusion
- **VirtualBox**: Oracle's virtualization platform
- **macOS VM** (macOS): Native macOS virtualization framework

#### Creating Local Nodes

1. **Open Node Manager**:
   - Navigate to "Remote Nodes" tab
   - Click "Add Node" → "Install Local"

2. **Configure Node**:
   - **Name**: Friendly name for the node
   - **Platform**: Target OS (Ubuntu, Debian, Alpine, CentOS, etc.)
   - **Virtualization**: Choose from detected technologies
   - **Resources**: Memory, CPU cores, disk size
   - **Capabilities**: Build, test, package operations
   - **Languages**: Pre-install language toolchains
   - **Build Tools**: Automatically install common build tools

3. **Auto-Detection**:
   - Build Studio automatically detects available virtualization technologies
   - Shows system resources and recommendations
   - Validates configuration before creation

#### Docker Nodes (Recommended)

```bash
# Prerequisites
sudo apt install docker.io  # Ubuntu/Debian
sudo systemctl enable --now docker
sudo usermod -aG docker $USER
```

**Features**:
- Fast startup and lightweight
- Excellent isolation
- Pre-built images available
- Easy cleanup and management

#### VM-Based Nodes

**KVM (Linux)**:
```bash
# Prerequisites
sudo apt install qemu-kvm libvirt-daemon-system
sudo usermod -aG libvirt $USER
```

**WSL (Windows)**:
```powershell
# Enable WSL
wsl --install
wsl --set-default-version 2
```

#### Managing Local Nodes

- **Start/Stop**: Control node lifecycle
- **Resource Monitoring**: View CPU, memory usage
- **Build History**: Track builds per node
- **Auto-Cleanup**: Remove unused nodes
- **Backup/Restore**: Save node configurations

### Remote Build Nodes (SSH)

Connect to remote build servers for distributed builds:

#### Setting Up SSH Remote Nodes

1. **Prerequisites**:
   - SSH server running on the remote machine
   - SSH key-based authentication (recommended) or password authentication
   - Build tools installed on the remote machine
   - Network connectivity between your machine and the remote node

2. **Adding a Remote Node**:
   - Open Build Studio and navigate to the "Remote Nodes" tab
   - Click "Add Node" → "Add Remote"
   - Fill in the connection details:
     - **Name**: A friendly name for the node
     - **Address**: IP address or hostname
     - **Port**: SSH port (default: 22)
     - **Username**: SSH username
     - **Authentication**: Choose SSH key or password
     - **SSH Key Path**: Path to your private key file (if using key auth)
     - **Password**: SSH password (if using password auth)
   - Select node capabilities (build, test, package)
   - Click "Test Connection" to verify connectivity
   - Click "Add Node" to save

3. **SSH Key Authentication Setup**:
   ```bash
   # Generate SSH key pair (if you don't have one)
   ssh-keygen -t rsa -b 4096 -C "your_email@example.com"
   
   # Copy public key to remote server
   ssh-copy-id username@remote-server-ip
   
   # Test connection
   ssh username@remote-server-ip
   ```

4. **Remote Node Requirements**:
   - SSH server (OpenSSH recommended)
   - Build tools for your target platforms
   - Sufficient disk space for builds
   - Network access to download dependencies

#### Using Remote Nodes

- Remote nodes appear in your build configuration options
- Select a remote node when configuring builds
- Build artifacts are automatically transferred back
- Monitor build progress in real-time
- View build logs and status

#### Security Best Practices

- Use SSH key authentication instead of passwords
- Regularly rotate SSH keys
- Restrict SSH access using `authorized_keys` options
- Use non-standard SSH ports when possible
- Enable SSH connection logging
- Use SSH hardening techniques

### CLI
- Build a project:
  ```sh
  buildstudio-cli build <project_dir>
  ```
- Package a project:
  ```sh
  buildstudio-cli package <project_dir>
  ```
- List plugins:
  ```sh
  buildstudio-cli plugins
  ```

---

## Configuration File (`buildstudio.config.yaml`)
```yaml
builds:
  - name: "Linux Release"
    platform: "linux"
    command: "make release"
  - name: "Windows Cross"
    platform: "windows"
    command: "make win"
  - name: "WebAssembly"
    platform: "wasm"
    command: "wasm-pack build"
package:
  type: "deb"
  name: "myapp"
  version: "1.0.0"
```

---

## Plugin System
- Place `.py` or `.js` files in `/plugins`.
- Plugins are auto-discovered and listed in the UI/CLI.
- Plugins can extend build, packaging, or UI flows.

---

## Packaging
- Supported: `deb`, `rpm`, `msi`, `pkg`
- Output in `/packages/<platform>/<timestamp>`
- Uses [FPM](https://fpm.readthedocs.io/) and platform-specific tools

---

## CI/CD Integration
- Add webhooks via GUI (Settings > CI/CD)
- Trigger builds and packaging from external systems
- Supports GitHub Actions, GitLab CI, and custom hooks

---

## Remote Build Nodes
- Register SSH or HTTP nodes via GUI
- Offload builds to remote servers
- Manage nodes in the Remote Nodes panel

---

## Architecture
- **Frontend**: SvelteKit (src/)
- **Backend**: Rust (src-tauri/src/)
- **Tauri**: Desktop shell and bridge
- **CLI**: src-tauri/src/bin/cli.rs
- **Plugins**: `/plugins` folder
- **Artifacts**: `/builds`, `/packages`

---

## Contributing
- Fork and PRs welcome!
- See `CONTRIBUTING.md` for guidelines
- For plugin development, see the Plugin SDK section

---

## License
MIT
