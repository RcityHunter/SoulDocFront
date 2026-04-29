#![allow(dead_code)]

pub mod ai_agent;
pub mod ai_tasks;
pub mod auth;
pub mod change_requests;
pub mod developer;
pub mod documents;
pub mod files;
pub mod git_sync;
pub mod language;
pub mod members;
pub mod notifications;
pub mod publications;
pub mod publish;
pub mod search;
pub mod settings;
pub mod spaces;
pub mod stats;
pub mod tags;
pub mod templates;
pub mod tool_configs;
pub mod versions;

use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use serde::{de::DeserializeOwned, Serialize};

const TOKEN_KEY: &str = "soulbook_token";
pub const BASE_URL: &str = "";

pub fn get_token() -> Option<String> {
    LocalStorage::get::<String>(TOKEN_KEY)
        .ok()
        .or_else(|| LocalStorage::get::<String>("souldoc_token").ok())
        .or_else(|| LocalStorage::get::<String>("jwt_token").ok())
        .or_else(|| LocalStorage::get::<String>("auth_token").ok())
        .or_else(|| LocalStorage::get::<String>("token").ok())
}

pub async fn api_get<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    let url = format!("{}{}", BASE_URL, path);
    let mut req = Request::get(&url);
    if let Some(token) = get_token() {
        req = req.header("Authorization", &format!("Bearer {}", token));
    }
    let resp = req.send().await.map_err(|e| e.to_string())?;
    if resp.ok() {
        resp.json::<T>().await.map_err(|e| e.to_string())
    } else {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        Err(format!("HTTP {}: {}", status, text))
    }
}

pub async fn api_post<B: Serialize, T: DeserializeOwned>(
    path: &str,
    body: &B,
) -> Result<T, String> {
    let url = format!("{}{}", BASE_URL, path);
    let mut req = Request::post(&url).header("Content-Type", "application/json");
    if let Some(token) = get_token() {
        req = req.header("Authorization", &format!("Bearer {}", token));
    }
    let resp = req
        .json(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.ok() {
        resp.json::<T>().await.map_err(|e| e.to_string())
    } else {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        Err(format!("HTTP {}: {}", status, text))
    }
}

pub async fn api_put<B: Serialize, T: DeserializeOwned>(path: &str, body: &B) -> Result<T, String> {
    let url = format!("{}{}", BASE_URL, path);
    let mut req = Request::put(&url).header("Content-Type", "application/json");
    if let Some(token) = get_token() {
        req = req.header("Authorization", &format!("Bearer {}", token));
    }
    let resp = req
        .json(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.ok() {
        resp.json::<T>().await.map_err(|e| e.to_string())
    } else {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        Err(format!("HTTP {}: {}", status, text))
    }
}

pub async fn api_delete(path: &str) -> Result<(), String> {
    let url = format!("{}{}", BASE_URL, path);
    let mut req = Request::delete(&url);
    if let Some(token) = get_token() {
        req = req.header("Authorization", &format!("Bearer {}", token));
    }
    let resp = req.send().await.map_err(|e| e.to_string())?;
    if resp.ok() {
        Ok(())
    } else {
        Err(format!("HTTP {}", resp.status()))
    }
}
