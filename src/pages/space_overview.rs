use crate::api::members as members_api;
use crate::api::spaces as spaces_api;
use crate::models::Space;
use crate::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn SpaceOverview() -> Element {
    let spaces_res = use_resource(|| async move { spaces_api::list_spaces(1, 50).await });
    let mut selected_slug = use_signal(|| String::new());
    let mut active_tab = use_signal(|| "overview");

    use_effect(move || {
        if selected_slug.read().is_empty() {
            if let Some(Ok(data)) = &*spaces_res.read() {
                if let Some(first) = data.spaces.as_ref().or(data.items.as_ref()).and_then(|s| s.first()) {
                    selected_slug.set(first.slug.clone());
                }
            }
        }
    });

    let members_res = use_resource(move || {
        let slug = selected_slug.read().clone();
        async move {
            if slug.is_empty() {
                return Err("请选择空间".to_string());
            }
            members_api::list_members(&slug).await
        }
    });

    // Get selected space object
    let selected_space: Option<Space> = match &*spaces_res.read() {
        Some(Ok(data)) => {
            let spaces = data
                .spaces
                .as_ref()
                .or(data.items.as_ref())
                .cloned()
                .unwrap_or_default();
            let slug = selected_slug.read().clone();
            spaces.into_iter().find(|s| s.slug == slug)
        }
        _ => None,
    };

    rsx! {
        document::Title { "空间概览 — SoulBook" }
        div { class: "page-content",
            // Space selector
            div { class: "card", style: "margin-bottom:20px;",
                div { class: "card-header", h3 { "选择空间" } }
                match &*spaces_res.read() {
                    None => rsx! { p { class: "text-muted", style: "padding:12px;", "加载中…" } },
                    Some(Err(e)) => rsx! { p { style: "color:#dc2626;padding:12px;", "{e}" } },
                    Some(Ok(data)) => {
                        let spaces = data.spaces.as_ref().or(data.items.as_ref()).cloned().unwrap_or_default();
                        rsx! {
                            div { style: "display:flex;flex-wrap:wrap;gap:8px;padding:4px 0;",
                                for space in spaces.iter() {
                                    {
                                        let slug = space.slug.clone();
                                        let name = space.name.clone();
                                        let is_sel = *selected_slug.read() == slug;
                                        rsx! {
                                            button {
                                                class: if is_sel { "btn btn-primary btn-sm" } else { "btn btn-sm" },
                                                onclick: move |_| { selected_slug.set(slug.clone()); active_tab.set("overview"); },
                                                "{name}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if selected_slug.read().is_empty() {
                div { style: "text-align:center;padding:60px;color:var(--muted);",
                    div { style: "font-size:48px;margin-bottom:12px;", "📘" }
                    h3 { "请选择一个知识空间" }
                    p { style: "font-size:13px;", "选择空间后查看详细概览信息" }
                }
            } else {
                if let Some(space) = &selected_space {
                    // Header
                    div { class: "page-header",
                        div { class: "page-header-left",
                            div { class: "breadcrumb", style: "margin-bottom:8px;",
                                Link { to: Route::Spaces {}, "空间" }
                                span { class: "breadcrumb-sep", "›" }
                                strong { "{space.name}" }
                            }
                            h1 { "📘 {space.name}" }
                            p {
                                if let Some(desc) = &space.description {
                                    "{desc}"
                                } else {
                                    "知识空间"
                                }
                                if space.is_public {
                                    span { class: "badge badge-success", style: "margin-left:10px;", "公共" }
                                } else {
                                    span { class: "badge badge-gray", style: "margin-left:10px;", "私有" }
                                }
                            }
                        }
                        div { class: "page-header-actions",
                            Link { to: Route::Docs {}, class: "btn btn-sm btn-primary", "进入文档中心" }
                        }
                    }

                    // Stats
                    div { class: "grid-4", style: "margin-bottom:24px;",
                        div { class: "metric-card",
                            div { class: "metric-icon", style: "background:#eef2ff;", "📄" }
                            div { class: "metric-value", "{space.doc_count.unwrap_or(0)}" }
                            div { class: "metric-label", "文档总数" }
                        }
                        div { class: "metric-card",
                            div { class: "metric-icon", style: "background:#d1fae5;", "👥" }
                            div { class: "metric-value", "{space.member_count.unwrap_or(0)}" }
                            div { class: "metric-label", "成员数" }
                        }
                        div { class: "metric-card",
                            div { class: "metric-icon", style: "background:#ede9fe;", "🏷️" }
                            div { class: "metric-value", "{space.tag_count.unwrap_or(0)}" }
                            div { class: "metric-label", "标签数" }
                        }
                        div { class: "metric-card",
                            div { class: "metric-icon", style: "background:#fef3c7;", "🔗" }
                            div { class: "metric-value", "{space.slug}" }
                            div { class: "metric-label", "Slug" }
                        }
                    }
                } else {
                    div { class: "page-header",
                        div { class: "page-header-left",
                            h1 { "📘 空间概览" }
                        }
                    }
                }

                // Tabs
                div { class: "tabs",
                    div { class: if active_tab() == "overview" { "tab active" } else { "tab" }, onclick: move |_| active_tab.set("overview"), "概览" }
                    div { class: if active_tab() == "members" { "tab active" } else { "tab" }, onclick: move |_| active_tab.set("members"), "成员" }
                }

                if active_tab() == "overview" {
                    div { class: "grid-2", style: "gap:20px;",
                        div { class: "card",
                            div { class: "card-header", h3 { "空间描述" } }
                            if let Some(space) = &selected_space {
                                if let Some(desc) = &space.description {
                                    p { style: "color:var(--text3);line-height:1.7;font-size:14px;", "{desc}" }
                                } else {
                                    p { style: "color:var(--muted);font-size:14px;font-style:italic;", "暂无描述" }
                                }
                                div { style: "margin-top:16px;display:flex;flex-wrap:wrap;gap:8px;",
                                    if space.is_public {
                                        span { class: "tag", "🌐 公共空间" }
                                    } else {
                                        span { class: "tag", "🔒 私有空间" }
                                    }
                                    if let Some(created_at) = &space.created_at {
                                        span { class: "tag", "📅 {created_at}" }
                                    }
                                }
                            }
                        }
                        div { class: "card",
                            div { class: "card-header", h3 { "快速入口" } }
                            div { style: "display:flex;flex-direction:column;gap:8px;",
                                Link { to: Route::Docs {}, style: "display:flex;align-items:center;justify-content:space-between;padding:10px 12px;border:1px solid var(--line);border-radius:9px;background:var(--panel2);",
                                    span { style: "font-size:13.5px;", "文档中心" }
                                    span { "→" }
                                }
                                Link { to: Route::ChangeRequests {}, style: "display:flex;align-items:center;justify-content:space-between;padding:10px 12px;border:1px solid var(--line);border-radius:9px;background:var(--panel2);",
                                    span { style: "font-size:13.5px;", "变更请求" }
                                    span { "→" }
                                }
                                Link { to: Route::AiTasks {}, style: "display:flex;align-items:center;justify-content:space-between;padding:10px 12px;border:1px solid var(--line);border-radius:9px;background:var(--panel2);",
                                    span { style: "font-size:13.5px;", "AI 任务" }
                                    span { "→" }
                                }
                                Link { to: Route::Members {}, style: "display:flex;align-items:center;justify-content:space-between;padding:10px 12px;border:1px solid var(--line);border-radius:9px;background:var(--panel2);",
                                    span { style: "font-size:13.5px;", "成员管理" }
                                    span { "→" }
                                }
                            }
                        }
                    }
                }

                if active_tab() == "members" {
                    match &*members_res.read() {
                        None => rsx! { div { class: "text-muted", style: "padding:40px;text-align:center;", "加载中…" } },
                        Some(Err(e)) => rsx! { div { style: "color:#dc2626;padding:40px;text-align:center;", "加载失败：{e}" } },
                        Some(Ok(members)) if members.is_empty() => rsx! {
                            div { style: "padding:40px;text-align:center;color:var(--muted);",
                                div { style: "font-size:36px;margin-bottom:10px;", "👥" }
                                p { "该空间还没有成员" }
                                Link { to: Route::Members {}, class: "btn btn-sm btn-primary", style: "margin-top:12px;display:inline-block;", "管理成员" }
                            }
                        },
                        Some(Ok(members)) => rsx! {
                            div { class: "card",
                                table { style: "width:100%;border-collapse:collapse;",
                                    thead {
                                        tr { style: "border-bottom:1px solid var(--line);",
                                            th { style: "text-align:left;padding:10px 16px;font-size:12px;color:var(--muted);font-weight:600;", "成员" }
                                            th { style: "text-align:left;padding:10px 16px;font-size:12px;color:var(--muted);font-weight:600;", "角色" }
                                            th { style: "text-align:left;padding:10px 16px;font-size:12px;color:var(--muted);font-weight:600;", "状态" }
                                            th { style: "text-align:left;padding:10px 16px;font-size:12px;color:var(--muted);font-weight:600;", "加入时间" }
                                        }
                                    }
                                    tbody {
                                        for m in members.iter() {
                                            tr { style: "border-bottom:1px solid var(--line);",
                                                td { style: "padding:12px 16px;",
                                                    div { style: "display:flex;align-items:center;gap:8px;",
                                                        div { class: "avatar", style: "width:28px;height:28px;font-size:11px;",
                                                            "{m.username.as_deref().or(m.email.as_deref()).unwrap_or(\"?\").chars().next().unwrap_or('?').to_uppercase().to_string()}"
                                                        }
                                                        div {
                                                            p { style: "font-size:13.5px;font-weight:500;", "{m.username.as_deref().unwrap_or(\"-\")}" }
                                                            p { style: "font-size:12px;color:var(--muted);", "{m.email.as_deref().unwrap_or(\"-\")}" }
                                                        }
                                                    }
                                                }
                                                td { style: "padding:12px 16px;",
                                                    span { class: "badge badge-primary", "{m.role.as_deref().unwrap_or(\"-\")}" }
                                                }
                                                td { style: "padding:12px 16px;",
                                                    span { class: "badge badge-success", "{m.status.as_deref().unwrap_or(\"active\")}" }
                                                }
                                                td { style: "padding:12px 16px;font-size:12px;color:var(--muted);",
                                                    "{m.joined_at.as_deref().unwrap_or(\"-\")}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}
