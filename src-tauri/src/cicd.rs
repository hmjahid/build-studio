
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::State;
use std::sync::Mutex;

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Webhook {
    pub id: String,
    pub url: String,
    pub secret: String,
    pub events: Vec<String>,
}

#[derive(Default)]
pub struct WebhookState {
    pub webhooks: Mutex<HashMap<String, Webhook>>,
}

#[tauri::command]
pub fn add_webhook(state: State<WebhookState>, webhook: Webhook) {
    let mut webhooks = state.webhooks.lock().unwrap();
    webhooks.insert(webhook.id.clone(), webhook);
}

#[tauri::command]
pub fn list_webhooks(state: State<WebhookState>) -> Vec<Webhook> {
    let webhooks = state.webhooks.lock().unwrap();
    webhooks.values().cloned().collect()
}

#[tauri::command]
pub async fn trigger_webhook(
    state: State<'_, WebhookState>,
    id: String,
    payload: serde_json::Value,
) -> Result<(), String> {
    let url = {
        let webhooks = state.webhooks.lock().unwrap();
        webhooks.get(&id).map(|w| w.url.clone())
    };

    if let Some(url) = url {
        let client = reqwest::Client::new();
        let res = client
            .post(&url)
            .header("X-GitHub-Event", "push") // Example event
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Webhook not found".to_string())
    }
}
