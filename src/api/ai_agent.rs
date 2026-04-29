use super::{api_get, api_post, BASE_URL};
use gloo_net::http::Request;
use serde_json::{json, Value};

// ── Public: Agent self-registration (no auth required) ───────────────────────

pub struct AgentRegisterRequest<'a> {
    pub agent_name: &'a str,
    pub agent_type: &'a str,
    pub contact_email: &'a str,
    pub description: &'a str,
}

/// Agent submits a registration application.
/// Returns `{ request_id, status, message }`.
pub async fn agent_register(req: AgentRegisterRequest<'_>) -> Result<Value, String> {
    let body = json!({
        "agent_name":     req.agent_name,
        "agent_type":     req.agent_type,
        "contact_email":  req.contact_email,
        "description":    req.description,
    });
    let url = format!("{}/agent/v1/register", BASE_URL);
    let resp = Request::post(&url)
        .header("Content-Type", "application/json")
        .json(&body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.ok() {
        resp.json::<Value>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("HTTP {}: {}", resp.status(), resp.text().await.unwrap_or_default()))
    }
}

/// Poll registration status by request_id.
/// Returns `{ status, api_key? (once only), message }`.
pub async fn get_register_status(request_id: &str) -> Result<Value, String> {
    let url = format!("{}/agent/v1/register/{}", BASE_URL, request_id);
    let resp = Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.ok() {
        resp.json::<Value>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("HTTP {}: {}", resp.status(), resp.text().await.unwrap_or_default()))
    }
}

// ── Public: Agent health ──────────────────────────────────────────────────────

pub async fn get_agent_health() -> Result<Value, String> {
    api_get("/agent/v1/system/health").await
}

// ── Admin: Agent request management (auth required) ───────────────────────────

/// List all agent registration requests (any status).
pub async fn list_agent_requests() -> Result<Value, String> {
    api_get("/api/docs/developer/agent-requests").await
}

/// Approve an agent registration. Returns the generated `api_key`.
pub async fn approve_agent_request(reg_id: &str) -> Result<Value, String> {
    api_post(
        &format!("/api/docs/developer/agent-requests/{}/approve", reg_id),
        &json!({}),
    )
    .await
}

/// Reject an agent registration with an optional reason.
pub async fn reject_agent_request(reg_id: &str, reason: &str) -> Result<Value, String> {
    api_post(
        &format!("/api/docs/developer/agent-requests/{}/reject", reg_id),
        &json!({ "reason": reason }),
    )
    .await
}
