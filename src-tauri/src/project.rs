use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use tauri::State;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub config_path: PathBuf,
    pub last_build: Option<String>,
    pub last_status: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct ProjectManager {
    #[serde(skip)]
    config_path: PathBuf,
    pub projects: Vec<Project>,
}

impl ProjectManager {
    pub fn load_projects(&mut self, app_config_dir: &PathBuf) {
        self.config_path = app_config_dir.join("projects.json");
        if self.config_path.exists() {
            let content = fs::read_to_string(&self.config_path).unwrap_or_default();
            if let Ok(manager) = serde_json::from_str::<ProjectManager>(&content) {
                self.projects = manager.projects;
            }
        } else {
            self.save_projects(); // Create the file if it doesn't exist
        }
    }

    fn save_projects(&self) {
        let content = serde_json::to_string_pretty(self).unwrap();
        fs::write(&self.config_path, content).expect("Failed to save projects");
    }

    pub fn add_project(&mut self, name: String, path: PathBuf) -> bool {
        // Previously required a buildstudio.config.yaml file. Now we allow any folder.
        let config_path = path.join("buildstudio.config.yaml");
        let project = Project {
            name,
            path: path.clone(),
            config_path,
            last_build: None,
            last_status: None,
        };
        self.projects.push(project);
        self.save_projects();
        true
    }

    pub fn remove_project(&mut self, name: &str) -> bool {
        let len = self.projects.len();
        self.projects.retain(|p| p.name != name);
        let changed = len != self.projects.len();
        if changed {
            self.save_projects();
        }
        changed
    }
}

#[tauri::command]
pub fn list_projects(state: State<'_, Mutex<ProjectManager>>) -> Vec<Project> {
    let manager = state.lock().unwrap();
    manager.projects.clone()
}

#[tauri::command]
pub fn add_project(state: State<'_, Mutex<ProjectManager>>, name: String, path: String) -> bool {
    let path = PathBuf::from(path);
    let mut manager = state.lock().unwrap();
    manager.add_project(name, path)
}

#[tauri::command]
pub fn remove_project(state: State<'_, Mutex<ProjectManager>>, name: String) -> bool {
    let mut manager = state.lock().unwrap();
    manager.remove_project(&name)
}