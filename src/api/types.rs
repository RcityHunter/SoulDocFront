use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthSession {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in_seconds: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub remember_me: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CurrentUser {
    pub id: String,
    pub display_name: String,
    pub email: String,
    pub user_type: UserType,
    pub role: String,
    pub active_scope_id: String,
    pub active_scope_name: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserType {
    Human,
    Ai,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScopeSummary {
    pub id: String,
    pub name: String,
    pub scope_type: ScopeType,
    pub description: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeType {
    PersonalWorkspace,
    Organization,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CollectionSummary {
    pub id: String,
    pub name: String,
    pub scope_id: String,
    pub space_count: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SpaceSummary {
    pub id: String,
    pub name: String,
    pub scope_id: String,
    pub visibility: SpaceVisibility,
    pub owner_scope_type: ScopeType,
    pub document_count: usize,
    pub published: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SpaceVisibility {
    Public,
    Private,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SpaceOverview {
    pub id: String,
    pub name: String,
    pub description: String,
    pub member_count: usize,
    pub locale_count: usize,
    pub ai_user_count: usize,
    pub publish_ready: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DocumentTreeNode {
    pub id: String,
    pub title: String,
    pub locale: String,
    pub status: String,
    pub children: Vec<DocumentTreeNode>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DocumentSummary {
    pub id: String,
    pub doc_group_id: String,
    pub title: String,
    pub locale: String,
    pub status: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DocumentDetail {
    pub id: String,
    pub doc_group_id: String,
    pub title: String,
    pub locale: String,
    pub content: String,
    pub seo: SeoMetadata,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SeoMetadata {
    pub title: String,
    pub description: String,
    pub slug: String,
    pub keywords: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VersionSummary {
    pub id: String,
    pub document_id: String,
    pub label: String,
    pub author_name: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChangeRequestSummary {
    pub id: String,
    pub document_id: String,
    pub title: String,
    pub status: String,
    pub reviewer_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AiTask {
    pub id: String,
    pub name: String,
    pub status: String,
    pub target_doc_id: Option<String>,
    pub created_at: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AiToolFamily {
    pub key: String,
    pub title: String,
    pub summary: String,
    pub actions: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SiteSummary {
    pub id: String,
    pub organization_id: String,
    pub name: String,
    pub slug: String,
    pub domain: String,
    pub section_count: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SiteSection {
    pub id: String,
    pub site_id: String,
    pub name: String,
    pub linked_space_ids: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublishTarget {
    pub id: String,
    pub space_id: String,
    pub name: String,
    pub channel: String,
    pub latest_release_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReleaseRecord {
    pub id: String,
    pub target_id: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GitSyncOverview {
    pub space_id: String,
    pub repository: String,
    pub branch: String,
    pub last_status: String,
    pub conflict_count: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WebhookSubscription {
    pub id: String,
    pub event: String,
    pub target_url: String,
    pub enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServiceCredential {
    pub id: String,
    pub user_id: String,
    pub label: String,
    pub last_used_at: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAiUserRequest {
    pub display_name: String,
    pub organization_id: String,
    pub role: String,
    pub execution_boundary: String,
    pub callback_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateAiUserResponse {
    pub id: String,
    pub display_name: String,
    pub user_type: UserType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SkillRegistrationRequest {
    pub skill_name: String,
    pub display_name: String,
    pub organization_id: String,
    pub role: String,
    pub callback_url: String,
    pub execution_boundary: String,
    pub site_ids: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SkillRegistrationResult {
    pub ai_user: CreateAiUserResponse,
    pub credential: ServiceCredential,
    pub manifest_url: String,
    pub bound_site_ids: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SiteManagementRequest {
    pub skill_name: String,
    pub site_id: String,
    pub target_space_ids: Vec<String>,
    pub publish_target_id: Option<String>,
    pub require_approval: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SiteManagementPlan {
    pub skill_name: String,
    pub site_id: String,
    pub summary: String,
    pub steps: Vec<SkillOperationStep>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SkillOperationStep {
    pub order: usize,
    pub capability_family: String,
    pub action: String,
    pub method: String,
    pub endpoint: String,
    pub approval_required: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CapabilityManifest {
    pub name: String,
    pub version: String,
    pub base_url: String,
    pub families: Vec<CapabilityFamily>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CapabilityFamily {
    pub key: String,
    pub title: String,
    pub actions: Vec<ManifestAction>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SkillManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub registration_flow: Vec<ManifestAction>,
    pub site_management_flow: Vec<ManifestAction>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ManifestAction {
    pub key: String,
    pub title: String,
    pub method: String,
    pub endpoint: String,
    pub approval_required: bool,
    pub async_task: bool,
}
