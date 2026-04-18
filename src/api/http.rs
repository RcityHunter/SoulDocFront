use async_trait::async_trait;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::json;

use crate::api::{
    client::{ApiConfig, ApiError, SoulDocGateway},
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
pub struct HttpGateway {
    config: ApiConfig,
}

impl HttpGateway {
    pub fn new(config: ApiConfig) -> Self {
        Self { config }
    }

    fn build_url(&self, path: &str) -> String {
        let base = self.config.base_url.trim_end_matches('/');
        let path = path.trim_start_matches('/');
        format!("{base}/{path}")
    }

    #[cfg(target_arch = "wasm32")]
    async fn request_json<T, B>(
        &self,
        method: &str,
        path: &str,
        body: Option<&B>,
    ) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        use gloo_net::http::Request;

        let url = self.build_url(path);
        let mut request = match method {
            "GET" => Request::get(&url),
            "POST" => Request::post(&url),
            "PUT" => Request::put(&url),
            "DELETE" => Request::delete(&url),
            "PATCH" => Request::patch(&url),
            other => return Err(ApiError::Transport(format!("不支持的 HTTP 方法: {other}"))),
        };

        if let Some(token) = &self.config.token {
            request = request.header("Authorization", &format!("Bearer {token}"));
        }

        let response = match body {
            Some(payload) => request
                .json(payload)
                .map_err(|error| ApiError::Transport(error.to_string()))?
                .send()
                .await
                .map_err(|error| ApiError::Transport(error.to_string()))?,
            None => request
                .send()
                .await
                .map_err(|error| ApiError::Transport(error.to_string()))?,
        };

        let status = response.status();
        if !(200..300).contains(&status) {
            let message = response
                .text()
                .await
                .unwrap_or_else(|_| "请求失败".to_string());
            return Err(ApiError::Http { status, message });
        }

        response
            .json::<T>()
            .await
            .map_err(|error| ApiError::Serialization(error.to_string()))
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn request_json<T, B>(
        &self,
        _method: &str,
        _path: &str,
        _body: Option<&B>,
    ) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        Err(ApiError::UnsupportedPlatform)
    }

    async fn get<T>(&self, path: &str) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
    {
        self.request_json::<T, ()>("GET", path, None).await
    }

    async fn post<T, B>(&self, path: &str, body: &B) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        self.request_json("POST", path, Some(body)).await
    }
}

#[async_trait(?Send)]
impl SoulDocGateway for HttpGateway {
    async fn login(&self, request: LoginRequest) -> Result<AuthSession, ApiError> {
        self.post("/api/v1/auth/login", &request).await
    }

    async fn current_user(&self) -> Result<CurrentUser, ApiError> {
        self.get("/api/v1/auth/me").await
    }

    async fn list_workspaces(&self) -> Result<Vec<ScopeSummary>, ApiError> {
        self.get("/api/v1/me/workspaces").await
    }

    async fn list_spaces(&self) -> Result<Vec<SpaceSummary>, ApiError> {
        self.get("/api/v1/spaces").await
    }

    async fn get_space_overview(&self, space_id: &str) -> Result<SpaceOverview, ApiError> {
        self.get(&format!("/api/v1/spaces/{space_id}/overview"))
            .await
    }

    async fn get_doc_tree(&self, space_id: &str) -> Result<Vec<DocumentTreeNode>, ApiError> {
        self.get(&format!("/api/v1/spaces/{space_id}/docs/tree"))
            .await
    }

    async fn get_doc_detail(&self, doc_id: &str) -> Result<DocumentDetail, ApiError> {
        self.get(&format!("/api/v1/docs/{doc_id}")).await
    }

    async fn list_versions(&self, doc_id: &str) -> Result<Vec<VersionSummary>, ApiError> {
        self.get(&format!("/api/v1/docs/{doc_id}/versions")).await
    }

    async fn list_change_requests(
        &self,
        doc_id: &str,
    ) -> Result<Vec<ChangeRequestSummary>, ApiError> {
        self.get(&format!("/api/v1/docs/{doc_id}/change-requests"))
            .await
    }

    async fn list_ai_tasks(&self) -> Result<Vec<AiTask>, ApiError> {
        self.get("/api/v1/ai/tasks").await
    }

    async fn list_ai_tool_families(&self) -> Result<Vec<AiToolFamily>, ApiError> {
        self.get("/api/v1/ai/tool-families").await
    }

    async fn list_sites(&self) -> Result<Vec<SiteSummary>, ApiError> {
        self.get("/api/v1/sites").await
    }

    async fn list_site_sections(&self, site_id: &str) -> Result<Vec<SiteSection>, ApiError> {
        self.get(&format!("/api/v1/sites/{site_id}/sections")).await
    }

    async fn list_publish_targets(&self, space_id: &str) -> Result<Vec<PublishTarget>, ApiError> {
        self.get(&format!("/api/v1/spaces/{space_id}/publish-targets"))
            .await
    }

    async fn get_git_sync(&self, space_id: &str) -> Result<GitSyncOverview, ApiError> {
        self.get(&format!("/api/v1/spaces/{space_id}/git-sync"))
            .await
    }

    async fn list_webhooks(&self) -> Result<Vec<WebhookSubscription>, ApiError> {
        self.get("/api/v1/webhooks").await
    }

    async fn create_ai_user(
        &self,
        request: CreateAiUserRequest,
    ) -> Result<CreateAiUserResponse, ApiError> {
        self.post(
            "/api/v1/users",
            &json!({
                "display_name": request.display_name,
                "organization_id": request.organization_id,
                "role": request.role,
                "callback_url": request.callback_url,
                "execution_boundary": request.execution_boundary,
                "user_type": "ai"
            }),
        )
        .await
    }

    async fn create_service_credential(
        &self,
        user_id: &str,
        label: &str,
    ) -> Result<ServiceCredential, ApiError> {
        self.post(
            &format!("/api/v1/users/{user_id}/service-credentials"),
            &json!({ "label": label }),
        )
        .await
    }

    async fn capability_manifest(&self) -> Result<CapabilityManifest, ApiError> {
        self.get("/.well-known/capabilities.json").await
    }

    async fn skill_manifest(&self) -> Result<SkillManifest, ApiError> {
        self.get("/.well-known/skill-manifest.json").await
    }

    async fn register_ai_skill_account(
        &self,
        request: SkillRegistrationRequest,
    ) -> Result<SkillRegistrationResult, ApiError> {
        let ai_user = self
            .create_ai_user(CreateAiUserRequest {
                display_name: request.display_name,
                organization_id: request.organization_id,
                role: request.role,
                execution_boundary: request.execution_boundary,
                callback_url: request.callback_url,
            })
            .await?;

        let credential = self
            .create_service_credential(
                &ai_user.id,
                &format!("{} Skill Credential", request.skill_name),
            )
            .await?;

        Ok(SkillRegistrationResult {
            ai_user,
            credential,
            manifest_url: "/.well-known/skill-manifest.json".into(),
            bound_site_ids: request.site_ids,
        })
    }

    async fn build_site_management_plan(
        &self,
        request: SiteManagementRequest,
    ) -> Result<SiteManagementPlan, ApiError> {
        let sections: Vec<SiteSection> = self.list_site_sections(&request.site_id).await?;
        let site_id = request.site_id.clone();
        let publish_target = request
            .publish_target_id
            .unwrap_or_else(|| "preview-target".into());

        Ok(SiteManagementPlan {
            skill_name: request.skill_name,
            site_id,
            summary: format!(
                "Skill 将读取 {} 个站点分区，必要时绑定 Space，再通过发布目标 {publish_target} 进入发布链。",
                sections.len()
            ),
            steps: vec![
                crate::api::types::SkillOperationStep {
                    order: 1,
                    capability_family: "Knowledge".into(),
                    action: "content.list".into(),
                    method: "GET".into(),
                    endpoint: format!("/api/v1/sites/{}/sections", request.site_id),
                    approval_required: false,
                },
                crate::api::types::SkillOperationStep {
                    order: 2,
                    capability_family: "Publishing".into(),
                    action: "publish.preview".into(),
                    method: "POST".into(),
                    endpoint: format!("/api/v1/publish-targets/{publish_target}/release"),
                    approval_required: request.require_approval,
                },
                crate::api::types::SkillOperationStep {
                    order: 3,
                    capability_family: "Connectors".into(),
                    action: "webhook.emit".into(),
                    method: "POST".into(),
                    endpoint: "/api/v1/webhooks/:id/test".into(),
                    approval_required: false,
                },
            ],
        })
    }
}
