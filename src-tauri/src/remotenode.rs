
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::State;
use std::sync::Mutex;

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct RemoteNode {
    pub id: String,
    pub address: String, // e.g., user@host:port
    pub status: String, // e.g., "online", "offline"
}

#[derive(Default)]
pub struct RemoteNodeManager {
    pub nodes: Mutex<HashMap<String, RemoteNode>>,
}

#[tauri::command]
pub fn add_remote_node(state: State<RemoteNodeManager>, node: RemoteNode) {
    let mut nodes = state.nodes.lock().unwrap();
    nodes.insert(node.id.clone(), node);
}

#[tauri::command]
pub fn list_remote_nodes(state: State<RemoteNodeManager>) -> Vec<RemoteNode> {
    let nodes = state.nodes.lock().unwrap();
    nodes.values().cloned().collect()
}

// TODO: Implement health checks and build execution over SSH/RPC
