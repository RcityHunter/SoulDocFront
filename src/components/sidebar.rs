use crate::routes::Route;
use crate::state::AuthState;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};

#[component]
pub fn Sidebar() -> Element {
    let route = use_route::<Route>();
    let mut ws_open = use_signal(|| false);
    let auth = use_context::<Signal<AuthState>>();

    let (user_letter, user_name) = {
        let a = auth.read();
        let letter = a
            .user
            .as_ref()
            .and_then(|u| u.display_name.as_deref().or(u.username.as_deref()))
            .and_then(|n| n.chars().next())
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_else(|| "A".to_string());
        let name = a
            .user
            .as_ref()
            .and_then(|u| u.display_name.clone().or(u.username.clone()))
            .unwrap_or_else(|| "Admin".to_string());
        (letter, name)
    };

    macro_rules! nav_cls {
        ($r:pat) => {
            if matches!(route, $r) {
                "nav-item active"
            } else {
                "nav-item"
            }
        };
    }

    rsx! {
        aside { class: "sidebar", onclick: move |_| ws_open.set(false),
            // ── Brand ──
            div { class: "sidebar-brand",
                div { class: "brand-logo", "SD" }
                div { class: "brand-text",
                    h1 { "SoulBook" }
                    p { "AI 原生知识工作台" }
                }
            }

            // ── 工作区切换 ──
            WorkspaceSwitcher { open: ws_open }

            // ── 主导航 ──
            div { class: "nav-section",
                div { class: "nav-section-title", "主导航" }
                Link { to: Route::Dashboard {}, class: nav_cls!(Route::Dashboard {}),
                    span { class: "nav-icon", "🏠" } "首页"
                }
                Link { to: Route::Spaces {}, class: nav_cls!(Route::Spaces {}),
                    span { class: "nav-icon", "🗂️" } "知识空间"
                }
                Link { to: Route::Search {}, class: nav_cls!(Route::Search {}),
                    span { class: "nav-icon", "🔍" } "搜索"
                }
                Link { to: Route::Notifications {}, class: nav_cls!(Route::Notifications {}),
                    span { class: "nav-icon", "🔔" } "通知中心"
                }
            }

            // ── 当前空间 ──
            div { class: "nav-section",
                div { class: "nav-section-title", "当前空间" }
                Link { to: Route::SpaceOverview {}, class: nav_cls!(Route::SpaceOverview {}),
                    span { class: "nav-icon", "📌" } "空间概览"
                }
                Link { to: Route::Docs {}, class: nav_cls!(Route::Docs {}),
                    span { class: "nav-icon", "📚" } "文档中心"
                }
                Link { to: Route::Editor {}, class: nav_cls!(Route::Editor {}),
                    span { class: "nav-icon", "✏️" } "编辑器"
                }
                Link { to: Route::ChangeRequests {}, class: nav_cls!(Route::ChangeRequests {}),
                    span { class: "nav-icon", "🧾" } "变更请求"
                }
                Link { to: Route::Members {}, class: nav_cls!(Route::Members {}),
                    span { class: "nav-icon", "👥" } "成员权限"
                }
                Link { to: Route::Tags {}, class: nav_cls!(Route::Tags {}),
                    span { class: "nav-icon", "🏷️" } "标签管理"
                }
                Link { to: Route::Files {}, class: nav_cls!(Route::Files {}),
                    span { class: "nav-icon", "📁" } "文件管理"
                }
                Link { to: Route::Seo {}, class: nav_cls!(Route::Seo {}),
                    span { class: "nav-icon", "🌐" } "发布 & SEO"
                }
            }

            // ── 智能与语言 ──
            div { class: "nav-section",
                div { class: "nav-section-title", "智能与语言" }
                Link { to: Route::Language {}, class: nav_cls!(Route::Language {}),
                    span { class: "nav-icon", "🌍" } "语言版本"
                }
                Link { to: Route::AiAgent {}, class: nav_cls!(Route::AiAgent {}),
                    span { class: "nav-icon", "🔌" } "AI Agent 接入"
                }
                Link { to: Route::AiTasks {}, class: nav_cls!(Route::AiTasks {}),
                    span { class: "nav-icon", "✨" } "AI 任务中心"
                }
                Link { to: Route::AiTools {}, class: nav_cls!(Route::AiTools {}),
                    span { class: "nav-icon", "🛠️" } "AI 工具配置"
                }
            }

            // ── 发布与平台 ──
            div { class: "nav-section",
                div { class: "nav-section-title", "发布与平台" }
                Link { to: Route::Workspace {}, class: nav_cls!(Route::Workspace {}),
                    span { class: "nav-icon", "🖥️" } "发布站点"
                }
                Link { to: Route::GitSync {}, class: nav_cls!(Route::GitSync {}),
                    span { class: "nav-icon", "🔀" } "GitHub 同步"
                }
                Link { to: Route::Developer {}, class: nav_cls!(Route::Developer {}),
                    span { class: "nav-icon", "⚡" } "开发者平台"
                }
                Link { to: Route::Templates {}, class: nav_cls!(Route::Templates {}),
                    span { class: "nav-icon", "📋" } "模板中心"
                }
                Link { to: Route::Settings {}, class: nav_cls!(Route::Settings {}),
                    span { class: "nav-icon", "⚙️" } "系统设置"
                }
            }

            // ── 底部用户 ──
            div { class: "sidebar-bottom",
                Link { to: Route::Profile {}, class: nav_cls!(Route::Profile {}),
                    div { class: "avatar avatar-sm",
                        style: "background:var(--gradient);margin-right:2px;",
                        "{user_letter}"
                    }
                    span { class: "identity-inline",
                        "{user_name}"
                        span { class: "identity-badge human", "人类" }
                    }
                }
            }
        }
    }
}

// ── 工作区切换器 ─────────────────────────────────────────

#[component]
fn WorkspaceSwitcher(open: Signal<bool>) -> Element {
    let mut open = open;
    let mut current = use_signal(|| "team_soulbook");
    let navigator = use_navigator();

    let (ws_letter, ws_name, ws_color) = match current() {
        "personal" => ("我", "个人工作区", "#64748b"),
        "team_soulbook" => ("S", "SoulBook 团队", "#4f46e5"),
        "team_dev" => ("P", "产品研发团队", "#2563eb"),
        "team_mkt" => ("M", "Marketing", "#7c3aed"),
        _ => ("S", "SoulBook 团队", "#4f46e5"),
    };
    let chevron_cls = if open() {
        "ws-chevron open"
    } else {
        "ws-chevron"
    };

    rsx! {
        div { class: "workspace-switcher",
            div {
                class: "workspace-current ws-trigger",
                onclick: move |e| {
                    e.stop_propagation();
                    open.set(!open());
                },
                div { style: "display:flex;align-items:center;gap:10px;flex:1;min-width:0;",
                    div { class: "ws-avatar", style: "background:{ws_color};", "{ws_letter}" }
                    div { style: "min-width:0;",
                        div { class: "workspace-current-label", "当前工作区" }
                        div { class: "workspace-current-title", "{ws_name}" }
                    }
                }
                span { class: "{chevron_cls}", "⌄" }
            }

            if open() {
                div { class: "ws-panel",
                    p { class: "ws-panel-header", "切换工作区" }

                    WsItem {
                        icon: "我", icon_bg: "#64748b", name: "个人工作区", badge: "个人",
                        selected: current() == "personal",
                        onclick: move |_| { current.set("personal"); open.set(false); }
                    }

                    div { class: "ws-separator" }
                    p { class: "ws-group-label", "团队" }

                    WsItem {
                        icon: "S", icon_bg: "#4f46e5", name: "SoulBook 团队", badge: "管理员",
                        selected: current() == "team_soulbook",
                        onclick: move |_| { current.set("team_soulbook"); open.set(false); }
                    }
                    WsItem {
                        icon: "P", icon_bg: "#2563eb", name: "产品研发团队", badge: "成员",
                        selected: current() == "team_dev",
                        onclick: move |_| { current.set("team_dev"); open.set(false); }
                    }
                    WsItem {
                        icon: "M", icon_bg: "#7c3aed", name: "Marketing", badge: "成员",
                        selected: current() == "team_mkt",
                        onclick: move |_| { current.set("team_mkt"); open.set(false); }
                    }

                    div { class: "ws-separator" }

                    button {
                        class: "ws-action-btn",
                        onclick: {
                            let navigator = navigator.clone();
                            move |_| {
                                let _ = LocalStorage::set("soulbook_open_create_space", "1");
                                open.set(false);
                                navigator.push(Route::Spaces {});
                            }
                        },
                        span { style: "font-size:16px;line-height:1;", "＋" }
                        "创建团队"
                    }
                    button {
                        class: "ws-action-btn",
                        onclick: {
                            let navigator = navigator.clone();
                            move |_| {
                                open.set(false);
                                navigator.push(Route::Members {});
                            }
                        },
                        span { style: "font-size:14px;", "⚙" }
                        "管理团队"
                    }
                }
            }
        }
    }
}

#[component]
fn WsItem(
    icon: &'static str,
    icon_bg: &'static str,
    name: &'static str,
    badge: &'static str,
    selected: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let cls = if selected {
        "ws-item ws-item-active"
    } else {
        "ws-item"
    };
    rsx! {
        div {
            class: "{cls}",
            onclick: move |e| onclick.call(e),
            div { class: "ws-item-icon", style: "background:{icon_bg};", "{icon}" }
            p { class: "ws-item-name", "{name}" }
            div { style: "display:flex;align-items:center;gap:6px;margin-left:auto;",
                span { class: "ws-item-badge", "{badge}" }
                if selected {
                    span { style: "color:var(--primary);font-weight:700;font-size:13px;", "✓" }
                }
            }
        }
    }
}
