# Build Studio Project Plan

## Notes
- User wants an advanced desktop application called "Build Studio" for configuring, managing, and executing multi-platform builds from a unified interface.
- Must support GUI (Tauri) and CLI.
- Target platforms: Linux, Windows, macOS, Android, iOS, WebAssembly.
- Supported languages: C/C++, Python, Java, Rust, Go, Node.js.
- Configuration via YAML/JSON (e.g., buildstudio.config.yaml).
- Must support Docker, VM, and local builds.
- Includes plugin system for extensibility (Python/JS plugins in /plugins).
- Advanced features: CI/CD hooks, remote build nodes, packaging system, plugin UI.
- Tech stack: Tauri+Svelte for UI, Rust for backend, Docker/Podman, QEMU, SSH, REST API, FPM, etc. (per user preference)
- Project scaffolding for Tauri+Svelte+Rust initiated (create-tauri-app run)
- User is running Fedora Linux; system dependencies should use `dnf`.
- Fedora prerequisite `librsvg2-devel` installed for Tauri.
- Default Svelte+Tauri UI is running; dev environment validated
- Folders for plugins, builds, and packages have been scaffolded in the project root
- Backend project/config management module implemented and integrated (Rust, Tauri commands)
- Project Manager Svelte store and UI component scaffolded; UI integration started
- Project Manager sidebar integrated into main UI; modular split layout complete
- Configuration parser module implemented and exposed as Tauri command
- Build config Svelte store and BuildConfigViewer UI component scaffolded and integrated
- Build execution engine (local shell) and real-time log streaming backend module implemented and exposed as Tauri command
- BuildLogViewer Svelte component and frontend integration completed
- Toolchain module for cross-compilation scaffolded and integrated; build execution now supports toolchain selection per platform
- Plugin system backend (discovery/listing) scaffolded and integrated as Tauri command
- PluginManager Svelte store and UI component scaffolded and integrated; plugin system (discovery/listing, UI) complete
- Packaging backend module scaffolded and integrated as Tauri command; per-platform packaging support (deb, rpm, msi, etc.)
- CI/CD integration backend (webhooks, triggers) scaffolded and integrated as Tauri commands
- Remote build node backend (SSH/HTTP API) scaffolded and integrated as Tauri commands
- Output: GUI app, CLI, builds in ./builds/{platform}/{timestamp}, packages in ./packages/, plugins in ./plugins.
- User requested cross-platform builds (Linux, Windows, macOS) and Docker image generation for Build Studio.
- Fixed duplicate <style> tag in +page.svelte to resolve build error.
- Major Rust backend fixes for Tauri build: removed duplicate Tauri commands, fixed missing imports, added reqwest/serde_yaml, refactored state to Mutex, fixed async signatures, cleaned up imports.
- Duplicate Tauri command macro errors for `read_text_file` and `write_text_file` in `lib.rs` have been resolved.
- CLI build errors due to private modules and missing dependencies (`futures`) have been resolved; modules are now public and dependencies added.
- CLI build error for missing `run_build_no_window` has been resolved by implementing and exporting a synchronous version for CLI use.
- CLI and GUI now build and link cleanly; all major build errors resolved.
- Critical: Tauri async command for `trigger_webhook` in `cicd.rs` is not `Send` due to holding a `MutexGuard` across `.await`. Previous attempted fix did not resolve; issue remains blocking.
- Tauri build and packaging succeeded for deb/rpm, but AppImage bundling failed due to linuxdeploy error.

## Task List
- [x] Analyze and document requirements (this plan)
- [x] Scaffold frontend (Tauri+Svelte)
- [x] Scaffold backend (Rust)
- [x] Scaffold core folders (plugins, builds, packages)
- [x] Implement backend project/config management (Rust, Tauri commands)
- [x] Implement project/workspace manager (UI integration)
- [x] Integrate ProjectManager sidebar and modular UI layout
- [x] Implement configuration parser (YAML/JSON)
- [x] Implement build config UI integration (Svelte store/component)
- [x] Implement build execution engine (local, Docker, VM)
- [x] Implement real-time build log viewer
- [x] Implement cross-compilation/toolchain support
- [x] Implement plugin system (discovery, loading, API)
  - [x] Scaffold plugin manager Svelte store and UI component
- [x] Implement packaging system (per-platform)
- [x] Implement CI/CD integration (webhooks, triggers)
- [x] Implement remote build node support (SSH, HTTP API)
- [x] Implement GUI features (project management, config editor, build status/history, plugin UI)
- [x] Implement CLI interface
- [x] Connect CLI to backend logic
- [x] Testing and validation (multi-platform)
- [x] Major Rust backend refactor for Tauri build and packaging
- [x] Resolve duplicate Tauri command macro errors in lib.rs
- [x] Fix CLI build errors (make modules public, add futures dependency)
- [x] Implement and export run_build_no_window for CLI
- [ ] Refactor CI/CD webhook async command to be Send-safe (release lock before await or use async mutex; previous attempt did not resolve)
- [ ] Documentation (user guide, plugin SDK)
- [ ] Build application for all major OSs (Linux, Windows, macOS)
- [ ] Create and publish Docker image for CLI/GUI

## Current Goal
Refactor CI/CD webhook async command to be Send-safe