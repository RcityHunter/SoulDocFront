use crate::api::{notifications as notif_api, spaces as spaces_api, stats as stats_api};
use crate::models::Space;
use crate::routes::Route;
use crate::state::AuthState;
use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    let auth = use_context::<Signal<AuthState>>();
    let display_name = {
        let state = auth.read();
        state
            .user
            .as_ref()
            .and_then(|u| u.username.as_deref())
            .unwrap_or("用户")
            .to_string()
    };

    let stats_res = use_resource(|| async move { stats_api::get_overview().await });
    let spaces_res = use_resource(|| async move { spaces_api::list_spaces(1, 6).await });
    let notif_res = use_resource(|| async move { notif_api::get_unread_count().await });

    rsx! {
        document::Title { "首页工作台 — SoulBook" }
        div { class: "page-content",
            div { class: "page-header",
                div { class: "page-header-left",
                    h1 { "👋 你好，{display_name}" }
                    p { "AI 原生知识工作台 — 管理文档、空间与 AI 任务" }
                }
                div { class: "page-header-actions",
                    Link { to: Route::Spaces {}, class: "btn btn-sm", "查看空间" }
                    Link { to: Route::AiTasks {}, class: "btn btn-sm btn-primary", "AI 任务" }
                }
            }

            // 核心指标
            div { class: "grid-4", style: "margin-bottom:24px;",
                MetricCard {
                    icon: "🗂️", icon_bg: "#eef2ff",
                    value: stats_res.read().as_ref().and_then(|r| r.as_ref().ok()).map(|s| s.space_count.to_string()).unwrap_or("…".into()),
                    label: "知识空间", change: ""
                }
                MetricCard {
                    icon: "📄", icon_bg: "#d1fae5",
                    value: stats_res.read().as_ref().and_then(|r| r.as_ref().ok()).map(|s| s.document_count.to_string()).unwrap_or("…".into()),
                    label: "文档总数", change: ""
                }
                MetricCard {
                    icon: "🧾", icon_bg: "#fef3c7",
                    value: stats_res.read().as_ref().and_then(|r| r.as_ref().ok()).map(|s| s.open_change_requests.to_string()).unwrap_or("…".into()),
                    label: "待审变更请求", change: ""
                }
                MetricCard {
                    icon: "🔔", icon_bg: "#ede9fe",
                    value: notif_res.read().as_ref().and_then(|r| r.as_ref().ok()).map(|n| n.to_string()).unwrap_or("…".into()),
                    label: "未读通知", change: ""
                }
            }

            // 快捷入口
            div { class: "card", style: "margin-bottom:20px;",
                div { class: "card-header", h3 { "快捷入口" } }
                div { class: "grid-4", style: "gap:10px;",
                    QuickLink { to: Route::Spaces {}, icon: "🗂️", label: "知识空间" }
                    QuickLink { to: Route::Docs {}, icon: "📄", label: "文档中心" }
                    QuickLink { to: Route::Tags {}, icon: "🏷️", label: "标签管理" }
                    QuickLink { to: Route::Search {}, icon: "🔍", label: "全局搜索" }
                    QuickLink { to: Route::ChangeRequests {}, icon: "🧾", label: "变更请求" }
                    QuickLink { to: Route::AiTasks {}, icon: "✨", label: "AI 任务" }
                    QuickLink { to: Route::Language {}, icon: "🌍", label: "语言版本" }
                    QuickLink { to: Route::Members {}, icon: "👥", label: "成员权限" }
                }
            }

            // 最近空间
            div { class: "card",
                div { class: "card-header",
                    h3 { "最近空间" }
                    Link { to: Route::Spaces {}, class: "btn btn-sm", "全部" }
                }
                match &*spaces_res.read() {
                    None => rsx! { p { class: "text-muted", style: "padding:16px;", "加载中…" } },
                    Some(Err(e)) => rsx! { p { style: "color:#dc2626;padding:16px;", "加载失败：{e}" } },
                    Some(Ok(data)) => {
                        let spaces: Vec<Space> = data.spaces.as_ref().or(data.items.as_ref()).cloned().unwrap_or_default();
                        if spaces.is_empty() {
                            rsx! {
                                div { style: "padding:24px;text-align:center;color:var(--muted);",
                                    p { "还没有空间，" }
                                    Link { to: Route::Spaces {}, style: "color:var(--primary);", "创建第一个空间" }
                                }
                            }
                        } else {
                            rsx! {
                                div { class: "grid-3", style: "gap:12px;padding:4px 0;",
                                    for space in spaces.iter().take(6) {
                                        Link { to: Route::SpaceOverview {},
                                            div { class: "space-card", style: "padding:16px;",
                                                div { style: "display:flex;align-items:center;gap:10px;margin-bottom:8px;",
                                                    span { style: "font-size:22px;", "🗂️" }
                                                    div {
                                                        p { style: "font-size:13.5px;font-weight:600;", "{space.name}" }
                                                        p { style: "font-size:12px;color:var(--muted);", "{space.slug}" }
                                                    }
                                                }
                                                div { style: "display:flex;gap:12px;font-size:12px;color:var(--muted);",
                                                    span { "📄 {space.doc_count.unwrap_or(0)}" }
                                                    span { "👥 {space.member_count.unwrap_or(0)}" }
                                                    if space.is_public {
                                                        span { class: "badge badge-success", style: "font-size:10px;", "公开" }
                                                    } else {
                                                        span { class: "badge badge-gray", style: "font-size:10px;", "私有" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MetricCard(
    icon: &'static str,
    icon_bg: &'static str,
    value: String,
    label: &'static str,
    change: &'static str,
) -> Element {
    rsx! {
        div { class: "metric-card",
            div { class: "metric-icon", style: "background:{icon_bg};", "{icon}" }
            div { class: "metric-value", "{value}" }
            div { class: "metric-label", "{label}" }
            if !change.is_empty() {
                div { class: "metric-change up", "{change}" }
            }
        }
    }
}

#[component]
fn QuickLink(to: Route, icon: &'static str, label: &'static str) -> Element {
    rsx! {
        Link { to,
            div { style: "display:flex;flex-direction:column;align-items:center;gap:6px;padding:14px 8px;border:1px solid var(--line);border-radius:10px;background:var(--panel2);cursor:pointer;transition:border-color .15s;",
                span { style: "font-size:22px;", "{icon}" }
                span { style: "font-size:12px;color:var(--text3);", "{label}" }
            }
        }
    }
}
