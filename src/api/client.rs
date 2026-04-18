use std::rc::Rc;

use async_trait::async_trait;
use thiserror::Error;

use crate::api::{
    http::HttpGateway,
    mock::MockGateway,
    types::{
        AiTask, AiToolFamily, AuthSession, CapabilityManifest, ChangeRequestSummary,
        CreateAiUserRequest, CreateAiUserResponse, CurrentUser, DocumentDetail, DocumentTreeNode,
        GitSyncOverview, LoginRequest, PublishTarget, ScopeSummary, ServiceCredential,
        SiteManagementPlan, SiteManagementRequest, SiteSection, SiteSummary, SkillManifest,
        SkillRegistrationRequest, SkillRegistrationResult, SpaceOverview, SpaceSummary,
        VersionSummary, WebhookSubscription,
    },
};

#[derive(Clone, Debug)]
pub struct ApiConfig {
    pub base_url: String,
    pub token: Option<String>,
}

impl ApiConfig {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            token: None,
        }
    }

    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApiMode {
    Mock,
    Http,
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("请求失败: {0}")]
    Transport(String),
    #[error("接口返回错误 {status}: {message}")]
    Http { status: u16, message: String },
    #[error("JSON 解析失败: {0}")]
    Serialization(String),
    #[error("当前平台未启用 HTTP 接口访问")]
    UnsupportedPlatform,
}

#[async_trait(?Send)]
pub trait SoulDocGateway {
    async fn login(&self, request: LoginRequest) -> Result<AuthSession, ApiError>;
    async fn current_user(&self) -> Result<CurrentUser, ApiError>;
    async fn list_workspaces(&self) -> Result<Vec<ScopeSummary>, ApiError>;
    async fn list_spaces(&self) -> Result<Vec<SpaceSummary>, ApiError>;
    async fn get_space_overview(&self, space_id: &str) -> Result<SpaceOverview, ApiError>;
    async fn get_doc_tree(&self, space_id: &str) -> Result<Vec<DocumentTreeNode>, ApiError>;
    async fn get_doc_detail(&self, doc_id: &str) -> Result<DocumentDetail, ApiError>;
    async fn list_versions(&self, doc_id: &str) -> Result<Vec<VersionSummary>, ApiError>;
    async fn list_change_requests(
        &self,
        doc_id: &str,
    ) -> Result<Vec<ChangeRequestSummary>, ApiError>;
    async fn list_ai_tasks(&self) -> Result<Vec<AiTask>, ApiError>;
    async fn list_ai_tool_families(&self) -> Result<Vec<AiToolFamily>, ApiError>;
    async fn list_sites(&self) -> Result<Vec<SiteSummary>, ApiError>;
    async fn list_site_sections(&self, site_id: &str) -> Result<Vec<SiteSection>, ApiError>;
    async fn list_publish_targets(&self, space_id: &str) -> Result<Vec<PublishTarget>, ApiError>;
    async fn get_git_sync(&self, space_id: &str) -> Result<GitSyncOverview, ApiError>;
    async fn list_webhooks(&self) -> Result<Vec<WebhookSubscription>, ApiError>;
    async fn create_ai_user(
        &self,
        request: CreateAiUserRequest,
    ) -> Result<CreateAiUserResponse, ApiError>;
    async fn create_service_credential(
        &self,
        user_id: &str,
        label: &str,
    ) -> Result<ServiceCredential, ApiError>;
    async fn capability_manifest(&self) -> Result<CapabilityManifest, ApiError>;
    async fn skill_manifest(&self) -> Result<SkillManifest, ApiError>;
    async fn register_ai_skill_account(
        &self,
        request: SkillRegistrationRequest,
    ) -> Result<SkillRegistrationResult, ApiError>;
    async fn build_site_management_plan(
        &self,
        request: SiteManagementRequest,
    ) -> Result<SiteManagementPlan, ApiError>;
}

#[derive(Clone)]
pub struct SoulDocApi {
    gateway: Rc<dyn SoulDocGateway>,
}

impl SoulDocApi {
    pub fn new(mode: ApiMode, config: ApiConfig) -> Self {
        let gateway: Rc<dyn SoulDocGateway> = match mode {
            ApiMode::Mock => Rc::new(MockGateway::default()),
            ApiMode::Http => Rc::new(HttpGateway::new(config)),
        };

        Self { gateway }
    }

    pub fn mock() -> Self {
        Self {
            gateway: Rc::new(MockGateway::default()),
        }
    }

    pub fn http(config: ApiConfig) -> Self {
        Self {
            gateway: Rc::new(HttpGateway::new(config)),
        }
    }

    pub async fn login(&self, request: LoginRequest) -> Result<AuthSession, ApiError> {
        self.gateway.login(request).await
    }

    pub async fn current_user(&self) -> Result<CurrentUser, ApiError> {
        self.gateway.current_user().await
    }

    pub async fn list_workspaces(&self) -> Result<Vec<ScopeSummary>, ApiError> {
        self.gateway.list_workspaces().await
    }

    pub async fn list_spaces(&self) -> Result<Vec<SpaceSummary>, ApiError> {
        self.gateway.list_spaces().await
    }

    pub async fn get_space_overview(&self, space_id: &str) -> Result<SpaceOverview, ApiError> {
        self.gateway.get_space_overview(space_id).await
    }

    pub async fn get_doc_tree(&self, space_id: &str) -> Result<Vec<DocumentTreeNode>, ApiError> {
        self.gateway.get_doc_tree(space_id).await
    }

    pub async fn get_doc_detail(&self, doc_id: &str) -> Result<DocumentDetail, ApiError> {
        self.gateway.get_doc_detail(doc_id).await
    }

    pub async fn list_versions(&self, doc_id: &str) -> Result<Vec<VersionSummary>, ApiError> {
        self.gateway.list_versions(doc_id).await
    }

    pub async fn list_change_requests(
        &self,
        doc_id: &str,
    ) -> Result<Vec<ChangeRequestSummary>, ApiError> {
        self.gateway.list_change_requests(doc_id).await
    }

    pub async fn list_ai_tasks(&self) -> Result<Vec<AiTask>, ApiError> {
        self.gateway.list_ai_tasks().await
    }

    pub async fn list_ai_tool_families(&self) -> Result<Vec<AiToolFamily>, ApiError> {
        self.gateway.list_ai_tool_families().await
    }

    pub async fn list_sites(&self) -> Result<Vec<SiteSummary>, ApiError> {
        self.gateway.list_sites().await
    }

    pub async fn list_site_sections(&self, site_id: &str) -> Result<Vec<SiteSection>, ApiError> {
        self.gateway.list_site_sections(site_id).await
    }

    pub async fn list_publish_targets(
        &self,
        space_id: &str,
    ) -> Result<Vec<PublishTarget>, ApiError> {
        self.gateway.list_publish_targets(space_id).await
    }

    pub async fn get_git_sync(&self, space_id: &str) -> Result<GitSyncOverview, ApiError> {
        self.gateway.get_git_sync(space_id).await
    }

    pub async fn list_webhooks(&self) -> Result<Vec<WebhookSubscription>, ApiError> {
        self.gateway.list_webhooks().await
    }

    pub async fn create_ai_user(
        &self,
        request: CreateAiUserRequest,
    ) -> Result<CreateAiUserResponse, ApiError> {
        self.gateway.create_ai_user(request).await
    }

    pub async fn create_service_credential(
        &self,
        user_id: &str,
        label: &str,
    ) -> Result<ServiceCredential, ApiError> {
        self.gateway.create_service_credential(user_id, label).await
    }

    pub async fn capability_manifest(&self) -> Result<CapabilityManifest, ApiError> {
        self.gateway.capability_manifest().await
    }

    pub async fn skill_manifest(&self) -> Result<SkillManifest, ApiError> {
        self.gateway.skill_manifest().await
    }

    pub async fn register_ai_skill_account(
        &self,
        request: SkillRegistrationRequest,
    ) -> Result<SkillRegistrationResult, ApiError> {
        self.gateway.register_ai_skill_account(request).await
    }

    pub async fn build_site_management_plan(
        &self,
        request: SiteManagementRequest,
    ) -> Result<SiteManagementPlan, ApiError> {
        self.gateway.build_site_management_plan(request).await
    }
}
