use crate::api::developer as dev_api;
use dioxus::prelude::*;
use serde_json::Value;

#[component]
pub fn Developer() -> Element {
    let mut active_tab = use_signal(|| "api");
    let keys_res = use_resource(|| async move { dev_api::list_api_keys().await });
    let webhooks_res = use_resource(|| async move { dev_api::list_webhooks().await });
    let ai_users_res = use_resource(|| async move { dev_api::list_ai_users().await });
    let manifest_res = use_resource(|| async move { dev_api::get_manifest().await });

    let mut show_create_key = use_signal(|| false);
    let mut new_key_name = use_signal(|| String::new());
    let mut creating_key = use_signal(|| false);
    let mut created_key = use_signal(|| String::new());

    let mut show_create_hook = use_signal(|| false);
    let mut new_hook_name = use_signal(|| String::new());
    let mut new_hook_url = use_signal(|| String::new());
    let mut creating_hook = use_signal(|| false);

    let mut action_msg = use_signal(|| String::new());

    rsx! {
        document::Title { "开发者平台 — SoulBook" }
        div { class: "page-content",
            div { class: "page-header",
                div { class: "page-header-left",
                    h1 { "⚡ 开发者平台" }
                    p { "API 密钥、Webhook、能力清单与 AI 用户注册" }
                }
                div { class: "page-header-actions",
                    if active_tab() == "api" {
                        button { class: "btn btn-primary", onclick: move |_| { show_create_key.set(true); created_key.set(String::new()); }, "＋ 创建 API 密钥" }
                    }
                    if active_tab() == "webhook" {
                        button { class: "btn btn-primary", onclick: move |_| show_create_hook.set(true), "＋ 添加 Webhook" }
                    }
                }
            }

            // Create API Key modal
            if show_create_key() {
                div { style: "position:fixed;inset:0;background:rgba(0,0,0,.4);z-index:300;display:flex;align-items:center;justify-content:center;",
                    onclick: move |_| show_create_key.set(false),
                    div { class: "card", style: "width:440px;padding:24px;", onclick: move |e| e.stop_propagation(),
                        h3 { style: "font-size:15px;font-weight:700;margin-bottom:16px;", "创建 API 密钥" }
                        if !created_key().is_empty() {
                            div { style: "margin-bottom:16px;padding:12px;background:#f0fdf4;border:1px solid #86efac;border-radius:8px;",
                                p { style: "font-weight:600;color:#16a34a;margin-bottom:6px;", "✅ 密钥已创建，请立即保存（仅显示一次）" }
                                p { style: "font-size:12px;font-family:monospace;word-break:break-all;color:#166534;", "{created_key}" }
                            }
                            button { class: "btn", onclick: move |_| show_create_key.set(false), "关闭" }
                        } else {
                            div { class: "form-group",
                                label { class: "form-label", "密钥名称" }
                                input { class: "input", placeholder: "如：CI/CD Bot", value: "{new_key_name}",
                                    oninput: move |e| new_key_name.set(e.value()) }
                            }
                            div { style: "display:flex;gap:10px;justify-content:flex-end;",
                                button { class: "btn", onclick: move |_| show_create_key.set(false), "取消" }
                                button { class: "btn btn-primary", disabled: creating_key(),
                                    onclick: move |_| {
                                        let name = new_key_name.read().trim().to_string();
                                        if name.is_empty() { return; }
                                        creating_key.set(true);
                                        spawn(async move {
                                            match dev_api::create_api_key(&serde_json::json!({ "name": name })).await {
                                                Ok(data) => {
                                                    let key = data.get("key").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                                    created_key.set(key);
                                                    new_key_name.set(String::new());
                                                }
                                                Err(e) => action_msg.set(format!("创建失败：{}", e)),
                                            }
                                            creating_key.set(false);
                                        });
                                    },
                                    if creating_key() { "创建中…" } else { "创建" }
                                }
                            }
                        }
                    }
                }
            }

            // Create Webhook modal
            if show_create_hook() {
                div { style: "position:fixed;inset:0;background:rgba(0,0,0,.4);z-index:300;display:flex;align-items:center;justify-content:center;",
                    onclick: move |_| show_create_hook.set(false),
                    div { class: "card", style: "width:440px;padding:24px;", onclick: move |e| e.stop_propagation(),
                        h3 { style: "font-size:15px;font-weight:700;margin-bottom:16px;", "添加 Webhook" }
                        div { class: "form-group",
                            label { class: "form-label", "名称" }
                            input { class: "input", placeholder: "如：Slack 通知", value: "{new_hook_name}",
                                oninput: move |e| new_hook_name.set(e.value()) }
                        }
                        div { class: "form-group",
                            label { class: "form-label", "目标 URL" }
                            input { class: "input", placeholder: "https://hooks.slack.com/...", value: "{new_hook_url}",
                                oninput: move |e| new_hook_url.set(e.value()) }
                        }
                        div { style: "display:flex;gap:10px;justify-content:flex-end;",
                            button { class: "btn", onclick: move |_| show_create_hook.set(false), "取消" }
                            button { class: "btn btn-primary", disabled: creating_hook(),
                                onclick: move |_| {
                                    let name = new_hook_name.read().trim().to_string();
                                    let url = new_hook_url.read().trim().to_string();
                                    if name.is_empty() || url.is_empty() { return; }
                                    creating_hook.set(true);
                                    spawn(async move {
                                        match dev_api::create_webhook(&serde_json::json!({ "name": name, "url": url })).await {
                                            Ok(_) => {
                                                show_create_hook.set(false);
                                                new_hook_name.set(String::new());
                                                new_hook_url.set(String::new());
                                            }
                                            Err(e) => action_msg.set(format!("创建失败：{}", e)),
                                        }
                                        creating_hook.set(false);
                                    });
                                },
                                if creating_hook() { "添加中…" } else { "添加" }
                            }
                        }
                    }
                }
            }

            div { class: "tabs",
                TabBtn { id: "api", label: "API 密钥", active: active_tab(), onclick: move |_| active_tab.set("api") }
                TabBtn { id: "webhook", label: "Webhook", active: active_tab(), onclick: move |_| active_tab.set("webhook") }
                TabBtn { id: "ai-users", label: "AI 用户", active: active_tab(), onclick: move |_| active_tab.set("ai-users") }
                TabBtn { id: "manifest", label: "能力清单", active: active_tab(), onclick: move |_| active_tab.set("manifest") }
            }

            if !action_msg().is_empty() {
                div { style: "padding:10px 14px;background:var(--panel2);border:1px solid var(--line);border-radius:8px;font-size:13px;margin-bottom:16px;",
                    "{action_msg}"
                }
            }

            if active_tab() == "api" {
                match &*keys_res.read() {
                    None => rsx! { p { style: "color:var(--muted);", "加载中…" } },
                    Some(Err(e)) => rsx! { p { style: "color:#dc2626;", "加载失败：{e}" } },
                    Some(Ok(keys)) if keys.is_empty() => rsx! {
                        div { style: "display:flex;flex-direction:column;gap:16px;",
                            div { style: "text-align:center;padding:60px;color:var(--muted);",
                                div { style: "font-size:48px;margin-bottom:12px;", "🔑" }
                                h3 { "暂无 API 密钥" }
                                p { style: "font-size:13px;margin-bottom:20px;", "创建 API 密钥以通过编程方式访问 SoulBook" }
                                button { class: "btn btn-primary", onclick: move |_| show_create_key.set(true), "＋ 创建第一个 API 密钥" }
                            }
                            div { class: "card",
                                div { class: "card-header", h3 { "快速开始" } }
                                div { class: "terminal",
                                    div { class: "terminal-line cmd", "npm install @soulbook/sdk" }
                                }
                            }
                        }
                    },
                    Some(Ok(keys)) => rsx! {
                        div { style: "display:flex;flex-direction:column;gap:10px;",
                            for key in keys.iter() {
                                ApiKeyRow { key_data: key.clone(), on_delete: move |id: String| {
                                    spawn(async move {
                                        let _ = dev_api::delete_api_key(&id).await;
                                        action_msg.set("密钥已删除".to_string());
                                    });
                                }}
                            }
                        }
                    },
                }
            }

            if active_tab() == "webhook" {
                match &*webhooks_res.read() {
                    None => rsx! { p { style: "color:var(--muted);", "加载中…" } },
                    Some(Err(e)) => rsx! { p { style: "color:#dc2626;", "加载失败：{e}" } },
                    Some(Ok(hooks)) if hooks.is_empty() => rsx! {
                        div { style: "text-align:center;padding:60px;color:var(--muted);",
                            div { style: "font-size:48px;margin-bottom:12px;", "🔌" }
                            h3 { "暂无 Webhook 订阅" }
                            p { style: "font-size:13px;margin-bottom:20px;", "添加 Webhook 以接收文档、变更请求等事件的实时通知" }
                            button { class: "btn btn-primary", onclick: move |_| show_create_hook.set(true), "＋ 添加第一个 Webhook" }
                        }
                    },
                    Some(Ok(hooks)) => rsx! {
                        div { style: "display:flex;flex-direction:column;gap:10px;",
                            for hook in hooks.iter() {
                                WebhookRow { hook: hook.clone(), on_test: move |id: String| {
                                    spawn(async move {
                                        match dev_api::test_webhook(&id).await {
                                            Ok(r) => action_msg.set(format!("✅ 测试成功：{}ms", r.get("response_time_ms").and_then(|v| v.as_i64()).unwrap_or(0))),
                                            Err(e) => action_msg.set(format!("❌ 测试失败：{}", e)),
                                        }
                                    });
                                }}
                            }
                        }
                    },
                }
            }

            if active_tab() == "ai-users" {
                div { style: "display:flex;flex-direction:column;gap:16px;",
                    div { class: "highlight-block",
                        p { style: "font-weight:600;margin-bottom:4px;", "🤖 注册 AI 用户" }
                        p { style: "font-size:13px;color:var(--text3);", "AI 用户与人类用户共用同一套角色与权限体系。通过此 API 注册 AI agent 并绑定到 Space。" }
                    }
                    match &*ai_users_res.read() {
                        Some(Ok(users)) if users.is_empty() => rsx! {
                            div { style: "text-align:center;padding:40px;color:var(--muted);",
                                div { style: "font-size:48px;margin-bottom:12px;", "🤖" }
                                h3 { "暂无已注册 AI 用户" }
                                p { style: "font-size:13px;", "通过 API 注册 AI agent，使其能够执行文档操作" }
                            }
                        },
                        Some(Ok(users)) => rsx! {
                            for u in users.iter() {
                                div { class: "card",
                                    p { style: "font-weight:600;",
                                        "{u.get(\"display_name\").and_then(|v| v.as_str()).unwrap_or(u.get(\"username\").and_then(|v| v.as_str()).unwrap_or(\"-\"))}"
                                    }
                                    p { style: "font-size:12px;color:var(--muted);",
                                        "{u.get(\"email\").and_then(|v| v.as_str()).unwrap_or(\"-\")}"
                                    }
                                }
                            }
                        },
                        _ => rsx! { p { style: "color:var(--muted);", "加载中…" } },
                    }
                }
            }

            if active_tab() == "manifest" {
                div { class: "card",
                    div { class: "card-header", h3 { "能力清单（Capability Manifest）" } }
                    p { style: "font-size:13px;color:var(--muted);margin-bottom:14px;",
                        "公开访问地址：" code { style: "font-size:12px;background:var(--panel3);padding:2px 7px;border-radius:5px;", "/.well-known/soulbook-manifest.json" }
                    }
                    match &*manifest_res.read() {
                        Some(Ok(manifest)) => rsx! {
                            div { class: "code-block",
                                pre { style: "font-size:12px;overflow-x:auto;",
                                    "{serde_json::to_string_pretty(manifest).unwrap_or_default()}"
                                }
                            }
                        },
                        _ => rsx! { p { style: "color:var(--muted);", "加载中…" } },
                    }
                }
            }
        }
    }
}

#[component]
fn ApiKeyRow(key_data: Value, on_delete: EventHandler<String>) -> Element {
    let id = key_data
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim_start_matches("api_key:")
        .to_string();
    let name = key_data
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("-")
        .to_string();
    let prefix = key_data
        .get("key_prefix")
        .and_then(|v| v.as_str())
        .unwrap_or("sk-sd-")
        .to_string();
    let created = key_data
        .get("created_at")
        .and_then(|v| v.as_str())
        .unwrap_or("-")
        .to_string();
    rsx! {
        div { class: "card",
            div { style: "display:flex;align-items:center;justify-content:space-between;",
                div {
                    p { style: "font-weight:600;margin-bottom:2px;", "{name}" }
                    p { style: "font-size:12px;color:var(--muted);font-family:monospace;", "{prefix}••••••••••••••••" }
                    p { style: "font-size:11px;color:var(--muted2);margin-top:2px;", "创建于 {created}" }
                }
                button { class: "btn btn-sm", style: "color:#dc2626;",
                    onclick: move |_| on_delete.call(id.clone()),
                    "删除"
                }
            }
        }
    }
}

#[component]
fn WebhookRow(hook: Value, on_test: EventHandler<String>) -> Element {
    let id = hook
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim_start_matches("webhook:")
        .to_string();
    let name = hook
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("-")
        .to_string();
    let url = hook
        .get("url")
        .and_then(|v| v.as_str())
        .unwrap_or("-")
        .to_string();
    let enabled = hook
        .get("enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    rsx! {
        div { class: "card",
            div { style: "display:flex;align-items:center;justify-content:space-between;",
                div {
                    p { style: "font-weight:600;margin-bottom:2px;", "{name}" }
                    p { style: "font-size:12px;color:var(--muted);", "{url}" }
                }
                div { style: "display:flex;align-items:center;gap:8px;",
                    if enabled {
                        span { class: "badge badge-success", "启用" }
                    } else {
                        span { class: "badge badge-gray", "停用" }
                    }
                    button { class: "btn btn-sm", onclick: move |_| on_test.call(id.clone()), "测试" }
                }
            }
        }
    }
}

#[component]
fn TabBtn(
    id: &'static str,
    label: &'static str,
    active: &'static str,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: if active == id { "tab active" } else { "tab" }, onclick: move |e| onclick.call(e), "{label}" }
    }
}
