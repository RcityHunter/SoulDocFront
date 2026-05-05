use super::{api_delete, api_get, api_post};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Clone, Default)]
pub struct AiTaskStats {
    pub running: i64,
    pub completed: i64,
    pub pending: i64,
    pub failed: i64,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct AiTask {
    pub id: Option<Value>,
    pub task_type: Option<String>,
    pub document_id: Option<String>,
    pub document_title: Option<String>,
    pub space_id: Option<String>,
    pub model: Option<String>,
    pub status: Option<String>,
    pub progress: Option<i64>,
    pub target_language: Option<String>,
    pub result: Option<Value>,
    pub created_by: Option<String>,
    pub created_at: Option<String>,
    pub error: Option<String>,
}

#[derive(Deserialize)]
struct Wrap<T> {
    data: T,
}
#[derive(Deserialize)]
struct ListData {
    items: Vec<AiTask>,
    stats: AiTaskStats,
}

pub struct TaskList {
    pub items: Vec<AiTask>,
    pub stats: AiTaskStats,
}

pub async fn list_tasks(status: Option<&str>) -> Result<TaskList, String> {
    let mut path = "/api/docs/ai-tasks?per_page=50".to_string();
    if let Some(s) = status {
        path.push_str(&format!("&status={}", s));
    }
    let resp: Wrap<ListData> = api_get(&path).await?;
    Ok(TaskList {
        items: resp.data.items,
        stats: resp.data.stats,
    })
}

#[derive(Serialize)]
pub struct CreateTaskRequest {
    pub task_type: String,
    pub document_id: String,
    pub document_title: Option<String>,
    pub space_id: Option<String>,
    pub model: Option<String>,
    pub target_language: Option<String>,
}

pub async fn create_task(req: CreateTaskRequest) -> Result<Value, String> {
    let resp: Wrap<Value> = api_post("/api/docs/ai-tasks", &req).await?;
    Ok(resp.data)
}

pub async fn cancel_task(id: &str) -> Result<(), String> {
    let path = format!("/api/docs/ai-tasks/{}/cancel", id);
    let _: Value = api_post(&path, &serde_json::json!({})).await?;
    Ok(())
}

pub async fn retry_task(id: &str) -> Result<(), String> {
    let path = format!("/api/docs/ai-tasks/{}/retry", id);
    let _: Value = api_post(&path, &serde_json::json!({})).await?;
    Ok(())
}

pub async fn delete_task(id: &str) -> Result<(), String> {
    let path = format!("/api/docs/ai-tasks/{}", id);
    api_delete(&path).await
}
