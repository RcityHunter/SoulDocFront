use super::{api_delete, api_get, api_post, api_put};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Clone)]
pub struct Member {
    pub user_id: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub role: Option<String>,
    pub status: Option<String>,
    #[serde(alias = "accepted_at", alias = "created_at")]
    pub joined_at: Option<String>,
}

#[derive(Deserialize)]
struct Wrap<T> {
    data: T,
}

pub async fn list_members(space_slug: &str) -> Result<Vec<Member>, String> {
    let path = format!("/api/docs/spaces/{}/members", space_slug);
    let resp: Wrap<Vec<Member>> = api_get(&path).await?;
    Ok(resp.data)
}

#[derive(Serialize)]
pub struct InviteRequest {
    pub email: Option<String>,
    pub role: String,
    pub message: Option<String>,
}

#[derive(Serialize)]
pub struct AcceptInvitationRequest {
    pub invite_token: String,
}

pub async fn invite_member(space_slug: &str, req: InviteRequest) -> Result<Value, String> {
    let path = format!("/api/docs/spaces/{}/invite", space_slug);
    let resp: Wrap<Value> = api_post(&path, &req).await?;
    Ok(resp.data)
}

pub async fn accept_invitation(invite_token: String) -> Result<Value, String> {
    let resp: Wrap<Value> = api_post(
        "/api/docs/spaces/invitations/accept",
        &AcceptInvitationRequest { invite_token },
    )
    .await?;
    Ok(resp.data)
}

pub async fn remove_member(space_slug: &str, user_id: &str) -> Result<(), String> {
    let path = format!("/api/docs/spaces/{}/members/{}", space_slug, user_id);
    api_delete(&path).await
}

#[derive(Serialize)]
pub struct UpdateMemberRequest {
    pub role: String,
}

pub async fn update_member(space_slug: &str, user_id: &str, role: String) -> Result<Value, String> {
    let path = format!("/api/docs/spaces/{}/members/{}", space_slug, user_id);
    let resp: Wrap<Value> = api_put(&path, &UpdateMemberRequest { role }).await?;
    Ok(resp.data)
}
