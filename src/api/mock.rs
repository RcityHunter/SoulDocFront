use async_trait::async_trait;

use crate::api::{
    client::{ApiError, SoulDocGateway},
    types::{
        AiTask, AiToolFamily, AuthSession, CapabilityFamily, CapabilityManifest,
        ChangeRequestSummary, CreateAiUserRequest, CreateAiUserResponse, CurrentUser,
        DocumentDetail, DocumentTreeNode, GitSyncOverview, LoginRequest, ManifestAction,
        PublishTarget, ScopeSummary, ScopeType, SeoMetadata, ServiceCredential, SiteManagementPlan,
        SiteManagementRequest, SiteSection, SiteSummary, SkillManifest, SkillOperationStep,
        SkillRegistrationRequest, SkillRegistrationResult, SpaceOverview, SpaceSummary,
        SpaceVisibility, UserType, VersionSummary, WebhookSubscription,
    },
};

#[derive(Default)]
pub struct MockGateway;

#[async_trait(?Send)]
impl SoulDocGateway for MockGateway {
    async fn login(&self, _request: LoginRequest) -> Result<AuthSession, ApiError> {
        Ok(AuthSession {
            access_token: "mock-access-token".into(),
            refresh_token: "mock-refresh-token".into(),
            expires_in_seconds: 7200,
        })
    }

    async fn current_user(&self) -> Result<CurrentUser, ApiError> {
        Ok(CurrentUser {
            id: "user_admin".into(),
            display_name: "Admin".into(),
            email: "admin@souldoc.io".into(),
            user_type: UserType::Human,
            role: "Owner".into(),
            active_scope_id: "org_souldoc".into(),
            active_scope_name: "SoulDoc 团队".into(),
        })
    }

    async fn list_workspaces(&self) -> Result<Vec<ScopeSummary>, ApiError> {
        Ok(vec![
            ScopeSummary {
                id: "pw_admin".into(),
                name: "个人工作区".into(),
                scope_type: ScopeType::PersonalWorkspace,
                description: "私有草稿、偏好与个人知识沉淀".into(),
            },
            ScopeSummary {
                id: "org_souldoc".into(),
                name: "SoulDoc 团队".into(),
                scope_type: ScopeType::Organization,
                description: "成员、内容集合、Space、发布站点与审批流".into(),
            },
        ])
    }

    async fn list_spaces(&self) -> Result<Vec<SpaceSummary>, ApiError> {
        Ok(vec![
            SpaceSummary {
                id: "space_demo".into(),
                name: "SoulDoc Demo".into(),
                scope_id: "org_souldoc".into(),
                visibility: SpaceVisibility::Public,
                owner_scope_type: ScopeType::Organization,
                document_count: 248,
                published: true,
            },
            SpaceSummary {
                id: "space_api".into(),
                name: "开放平台".into(),
                scope_id: "org_souldoc".into(),
                visibility: SpaceVisibility::Public,
                owner_scope_type: ScopeType::Organization,
                document_count: 86,
                published: true,
            },
        ])
    }

    async fn get_space_overview(&self, _space_id: &str) -> Result<SpaceOverview, ApiError> {
        Ok(SpaceOverview {
            id: "space_demo".into(),
            name: "SoulDoc Demo".into(),
            description: "承载文档树、版本、CR、AI 任务与语言版本".into(),
            member_count: 12,
            locale_count: 3,
            ai_user_count: 4,
            publish_ready: true,
        })
    }

    async fn get_doc_tree(&self, _space_id: &str) -> Result<Vec<DocumentTreeNode>, ApiError> {
        Ok(vec![
            DocumentTreeNode {
                id: "doc_prd".into(),
                title: "产品需求文档 PRD".into(),
                locale: "zh-CN".into(),
                status: "draft".into(),
                children: vec![],
            },
            DocumentTreeNode {
                id: "doc_ai_tools".into(),
                title: "AI 工具配置".into(),
                locale: "zh-CN".into(),
                status: "review".into(),
                children: vec![],
            },
        ])
    }

    async fn get_doc_detail(&self, _doc_id: &str) -> Result<DocumentDetail, ApiError> {
        Ok(DocumentDetail {
            id: "doc_ai_tools".into(),
            doc_group_id: "group_ai_tools".into(),
            title: "AI 工具配置".into(),
            locale: "zh-CN".into(),
            content: "围绕 6 大能力族构建 SoulDoc 的 AI 与 Skill 接入边界。".into(),
            seo: SeoMetadata {
                title: "AI 工具配置".into(),
                description: "SoulDoc AI 工具、Skill 与站点治理接入方案".into(),
                slug: "ai-tools".into(),
                keywords: vec!["AI".into(), "Skill".into(), "Site".into()],
            },
        })
    }

    async fn list_versions(&self, _doc_id: &str) -> Result<Vec<VersionSummary>, ApiError> {
        Ok(vec![
            VersionSummary {
                id: "v24".into(),
                document_id: "doc_ai_tools".into(),
                label: "v24".into(),
                author_name: "Admin".into(),
                created_at: "2026-04-18T10:30:00+08:00".into(),
            },
            VersionSummary {
                id: "v23".into(),
                document_id: "doc_ai_tools".into(),
                label: "v23".into(),
                author_name: "Aurora".into(),
                created_at: "2026-04-17T22:15:00+08:00".into(),
            },
        ])
    }

    async fn list_change_requests(
        &self,
        _doc_id: &str,
    ) -> Result<Vec<ChangeRequestSummary>, ApiError> {
        Ok(vec![ChangeRequestSummary {
            id: "cr_15".into(),
            document_id: "doc_ai_tools".into(),
            title: "更新 Skill 接入与站点治理方案".into(),
            status: "reviewing".into(),
            reviewer_name: "Admin".into(),
        }])
    }

    async fn list_ai_tasks(&self) -> Result<Vec<AiTask>, ApiError> {
        Ok(vec![
            AiTask {
                id: "task_translate".into(),
                name: "翻译文档".into(),
                status: "running".into(),
                target_doc_id: Some("doc_ai_tools".into()),
                created_at: "2026-04-18T13:20:00+08:00".into(),
            },
            AiTask {
                id: "task_seo".into(),
                name: "发布前 SEO 检查".into(),
                status: "queued".into(),
                target_doc_id: Some("doc_ai_tools".into()),
                created_at: "2026-04-18T13:25:00+08:00".into(),
            },
        ])
    }

    async fn list_ai_tool_families(&self) -> Result<Vec<AiToolFamily>, ApiError> {
        Ok(vec![
            AiToolFamily {
                key: "context".into(),
                title: "Context".into(),
                summary: "读取上下文、导航位置、权限边界".into(),
                actions: vec!["context.get".into(), "permission.inspect".into()],
            },
            AiToolFamily {
                key: "connectors".into(),
                title: "Connectors".into(),
                summary: "GitHub 同步、Webhook、外部 Skill 与 Manifest".into(),
                actions: vec![
                    "git.sync".into(),
                    "webhook.emit".into(),
                    "knowledge.retrieve".into(),
                ],
            },
        ])
    }

    async fn list_sites(&self) -> Result<Vec<SiteSummary>, ApiError> {
        Ok(vec![SiteSummary {
            id: "site_docs".into(),
            organization_id: "org_souldoc".into(),
            name: "docs.souldoc.io".into(),
            slug: "docs".into(),
            domain: "docs.souldoc.io".into(),
            section_count: 3,
        }])
    }

    async fn list_site_sections(&self, _site_id: &str) -> Result<Vec<SiteSection>, ApiError> {
        Ok(vec![
            SiteSection {
                id: "section_product".into(),
                site_id: "site_docs".into(),
                name: "产品文档".into(),
                linked_space_ids: vec!["space_demo".into()],
            },
            SiteSection {
                id: "section_platform".into(),
                site_id: "site_docs".into(),
                name: "开放平台".into(),
                linked_space_ids: vec!["space_api".into()],
            },
        ])
    }

    async fn list_publish_targets(&self, _space_id: &str) -> Result<Vec<PublishTarget>, ApiError> {
        Ok(vec![PublishTarget {
            id: "target_docs_prod".into(),
            space_id: "space_demo".into(),
            name: "生产站点".into(),
            channel: "public-site".into(),
            latest_release_id: Some("release_1024".into()),
        }])
    }

    async fn get_git_sync(&self, _space_id: &str) -> Result<GitSyncOverview, ApiError> {
        Ok(GitSyncOverview {
            space_id: "space_demo".into(),
            repository: "github.com/souldoc/docs".into(),
            branch: "main".into(),
            last_status: "conflict".into(),
            conflict_count: 1,
        })
    }

    async fn list_webhooks(&self) -> Result<Vec<WebhookSubscription>, ApiError> {
        Ok(vec![
            WebhookSubscription {
                id: "wh_publish".into(),
                event: "document.published".into(),
                target_url: "https://example.com/webhooks/publish".into(),
                enabled: true,
            },
            WebhookSubscription {
                id: "wh_locale".into(),
                event: "locale.review_required".into(),
                target_url: "https://example.com/webhooks/locale".into(),
                enabled: true,
            },
        ])
    }

    async fn create_ai_user(
        &self,
        request: CreateAiUserRequest,
    ) -> Result<CreateAiUserResponse, ApiError> {
        Ok(CreateAiUserResponse {
            id: format!("ai_{}", request.display_name.to_lowercase()),
            display_name: request.display_name,
            user_type: UserType::Ai,
        })
    }

    async fn create_service_credential(
        &self,
        user_id: &str,
        label: &str,
    ) -> Result<ServiceCredential, ApiError> {
        Ok(ServiceCredential {
            id: format!("cred_{user_id}"),
            user_id: user_id.to_string(),
            label: label.to_string(),
            last_used_at: None,
        })
    }

    async fn capability_manifest(&self) -> Result<CapabilityManifest, ApiError> {
        serde_json::from_str(include_str!("../../public/.well-known/capabilities.json"))
            .map_err(|error| ApiError::Serialization(error.to_string()))
    }

    async fn skill_manifest(&self) -> Result<SkillManifest, ApiError> {
        serde_json::from_str(include_str!("../../public/.well-known/skill-manifest.json"))
            .map_err(|error| ApiError::Serialization(error.to_string()))
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
        Ok(SiteManagementPlan {
            skill_name: request.skill_name,
            site_id: request.site_id,
            summary: "Skill 先读取站点、分区和 Space 绑定，再按发布链生成预览与发布任务。".into(),
            steps: vec![
                SkillOperationStep {
                    order: 1,
                    capability_family: "Context".into(),
                    action: "context.get".into(),
                    method: "GET".into(),
                    endpoint: "/connectors/docs/:docId/context".into(),
                    approval_required: false,
                },
                SkillOperationStep {
                    order: 2,
                    capability_family: "Knowledge".into(),
                    action: "content.list".into(),
                    method: "GET".into(),
                    endpoint: "/sites/:siteId/sections".into(),
                    approval_required: false,
                },
                SkillOperationStep {
                    order: 3,
                    capability_family: "Publishing".into(),
                    action: "publish.preview".into(),
                    method: "POST".into(),
                    endpoint: "/publish-targets/:targetId/release".into(),
                    approval_required: request.require_approval,
                },
                SkillOperationStep {
                    order: 4,
                    capability_family: "Connectors".into(),
                    action: "webhook.emit".into(),
                    method: "POST".into(),
                    endpoint: "/webhooks/:id/test".into(),
                    approval_required: false,
                },
            ],
        })
    }
}

#[allow(dead_code)]
fn _manifest_examples() -> (CapabilityManifest, SkillManifest) {
    (
        CapabilityManifest {
            name: "SoulDoc Capabilities".into(),
            version: "1.0.0".into(),
            base_url: "/api/v1".into(),
            families: vec![CapabilityFamily {
                key: "publishing".into(),
                title: "Publishing".into(),
                actions: vec![ManifestAction {
                    key: "publish.preview".into(),
                    title: "发布预览".into(),
                    method: "POST".into(),
                    endpoint: "/publish-targets/:targetId/release".into(),
                    approval_required: true,
                    async_task: true,
                }],
            }],
        },
        SkillManifest {
            name: "souldoc-site-manager".into(),
            version: "1.0.0".into(),
            description: "用于注册 AI 账号并管理站点编排、发布和 Webhook".into(),
            registration_flow: vec![],
            site_management_flow: vec![],
        },
    )
}
