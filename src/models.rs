use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

fn deserialize_optional_record_id<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(value.and_then(|value| match value {
        Value::String(id) => Some(id),
        Value::Object(obj) => obj
            .get("id")
            .and_then(Value::as_str)
            .map(ToString::to_string)
            .or_else(|| {
                obj.get("tb")
                    .and_then(Value::as_str)
                    .zip(obj.get("id").and_then(Value::as_str))
                    .map(|(table, id)| format!("{}:{}", table, id))
            }),
        _ => None,
    }))
}

/* ── Auth ────────────────────────────────────────────── */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub display_name: Option<String>,
}

/* ── Space ───────────────────────────────────────────── */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Space {
    #[serde(default, deserialize_with = "deserialize_optional_record_id")]
    pub id: Option<String>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub owner_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    #[serde(alias = "document_count")]
    pub doc_count: Option<u32>,
    pub member_count: Option<u32>,
    pub tag_count: Option<u32>,
    pub stats: Option<SpaceStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SpaceStats {
    #[serde(alias = "doc_count")]
    pub document_count: Option<u32>,
    pub member_count: Option<u32>,
    pub tag_count: Option<u32>,
    pub public_document_count: Option<u32>,
    pub comment_count: Option<u32>,
    pub view_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SpaceList {
    pub items: Vec<Space>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Serialize)]
pub struct CreateSpaceRequest {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub is_public: bool,
}

/* ── Document ────────────────────────────────────────── */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Document {
    #[serde(default, deserialize_with = "deserialize_optional_record_id")]
    pub id: Option<String>,
    pub title: String,
    pub slug: String,
    pub space_id: Option<String>,
    pub content: Option<String>,
    pub is_public: Option<bool>,
    pub status: Option<String>,
    pub parent_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<DocumentMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DocumentMetadata {
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentTreeNode {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub children: Option<Vec<DocumentTreeNode>>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentList {
    pub items: Vec<Document>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
}

/* ── Tag ─────────────────────────────────────────────── */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    #[serde(default, deserialize_with = "deserialize_optional_record_id")]
    pub id: Option<String>,
    pub name: String,
    pub color: Option<String>,
    pub description: Option<String>,
    #[serde(alias = "usage_count")]
    pub doc_count: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_optional_record_id")]
    pub space_id: Option<String>,
}

/* ── Notification ────────────────────────────────────── */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Notification {
    #[serde(default, deserialize_with = "deserialize_optional_record_id")]
    pub id: Option<String>,
    pub title: String,
    #[serde(alias = "content")]
    pub message: Option<String>,
    #[serde(alias = "type")]
    pub notification_type: Option<String>,
    pub is_read: bool,
    pub created_at: Option<String>,
    pub link: Option<String>,
    pub invite_token: Option<String>,
    pub space_name: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NotificationList {
    pub items: Vec<Notification>,
    pub total: u64,
    pub unread_count: u32,
}

/* ── Search ──────────────────────────────────────────── */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub snippet: Option<String>,
    pub space_name: Option<String>,
    pub space_slug: Option<String>,
    pub doc_slug: Option<String>,
    pub result_type: Option<String>,
    pub tags: Option<Vec<String>>,
    pub score: Option<f64>,
}

/* ── Version ─────────────────────────────────────────── */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Version {
    pub id: Option<String>,
    pub document_id: Option<String>,
    pub version_number: Option<u32>,
    pub label: Option<String>,
    pub created_by: Option<String>,
    pub created_at: Option<String>,
    pub summary: Option<String>,
}

/* ── Stats ───────────────────────────────────────────── */

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Stats {
    pub space_count: u32,
    pub document_count: u32,
    pub member_count: u32,
    pub tag_count: u32,
}

/* ── Member ──────────────────────────────────────────── */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Member {
    pub user_id: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub role: String,
    pub joined_at: Option<String>,
}
