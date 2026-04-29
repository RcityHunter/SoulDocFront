use crate::api::settings as settings_api;
use dioxus::prelude::*;
use serde_json::Value;

#[component]
pub fn Settings() -> Element {
    let mut active_tab = use_signal(|| "general");
    let settings_res = use_resource(|| async move { settings_api::get_settings().await });
    let mut saving = use_signal(|| false);
    let mut save_msg = use_signal(|| String::new());

    // local mutable overrides for toggles
    let mut notif_email = use_signal(|| true);
    let mut notif_browser = use_signal(|| true);
    let mut notif_ai = use_signal(|| false);
    let mut dark_mode = use_signal(|| false);

    // sync toggles from loaded settings
    use_effect(move || {
        if let Some(Ok(data)) = &*settings_res.read() {
            let notif = data.get("notifications");
            notif_email.set(
                notif
                    .and_then(|n| n.get("email"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true),
            );
            notif_browser.set(
                notif
                    .and_then(|n| n.get("browser"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true),
            );
            notif_ai.set(
                notif
                    .and_then(|n| n.get("ai_tasks"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
            );
            dark_mode.set(
                data.get("appearance")
                    .and_then(|a| a.get("dark_mode"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
            );
        }
    });

    let save_notifications = move |_| {
        saving.set(true);
        save_msg.set(String::new());
        let body = serde_json::json!({
            "email": notif_email(),
            "browser": notif_browser(),
            "ai_tasks": notif_ai(),
        });
        spawn(async move {
            match settings_api::update_notifications(&body).await {
                Ok(_) => save_msg.set("已保存".to_string()),
                Err(e) => save_msg.set(format!("保存失败：{}", e)),
            }
            saving.set(false);
        });
    };

    let save_appearance = move |_| {
        saving.set(true);
        save_msg.set(String::new());
        let body = serde_json::json!({ "dark_mode": dark_mode() });
        spawn(async move {
            match settings_api::update_appearance(&body).await {
                Ok(_) => save_msg.set("已保存".to_string()),
                Err(e) => save_msg.set(format!("保存失败：{}", e)),
            }
            saving.set(false);
        });
    };

    rsx! {
        document::Title { "系统设置 — SoulBook" }
        div { class: "page-content",
            div { class: "page-header",
                div { class: "page-header-left",
                    h1 { "⚙️ 系统设置" }
                    p { "管理平台全局配置、AI 设置、安全与通知偏好" }
                }
                if !save_msg().is_empty() {
                    span { style: "font-size:13px;color:var(--muted);margin-left:auto;", "{save_msg}" }
                }
            }

            div { class: "grid-2", style: "gap:0;align-items:start;",
                // 左侧导航
                div { style: "padding-right:24px;",
                    div { class: "card", style: "padding:8px;",
                        div { style: "display:flex;flex-direction:column;gap:2px;",
                            SettingsNavItem { id: "general", label: "通用设置", icon: "⚙️", active: active_tab(), onclick: move |_| active_tab.set("general") }
                            SettingsNavItem { id: "ai", label: "AI 配置", icon: "✨", active: active_tab(), onclick: move |_| active_tab.set("ai") }
                            SettingsNavItem { id: "notifications", label: "通知偏好", icon: "🔔", active: active_tab(), onclick: move |_| active_tab.set("notifications") }
                            SettingsNavItem { id: "security", label: "安全设置", icon: "🔒", active: active_tab(), onclick: move |_| active_tab.set("security") }
                            SettingsNavItem { id: "appearance", label: "外观", icon: "🎨", active: active_tab(), onclick: move |_| active_tab.set("appearance") }
                        }
                    }
                }

                // 右侧内容
                div {
                    {
                        let data: Value = match &*settings_res.read() {
                            Some(Ok(v)) => v.clone(),
                            _ => serde_json::json!({}),
                        };

                        let general = data.get("general").cloned().unwrap_or(serde_json::json!({}));
                        let ai_cfg = data.get("ai").cloned().unwrap_or(serde_json::json!({}));
                        let security = data.get("security").cloned().unwrap_or(serde_json::json!({}));

                        let platform_name = general.get("platform_name").and_then(|v| v.as_str()).unwrap_or("SoulBook").to_string();
                        let default_model = ai_cfg.get("default_model").and_then(|v| v.as_str()).unwrap_or("claude-3-5-sonnet").to_string();
                        let session_timeout = security.get("session_timeout").and_then(|v| v.as_i64()).unwrap_or(480).to_string();

                        rsx! {
                            if active_tab() == "general" {
                                div { class: "card",
                                    div { class: "card-header", h3 { "通用设置" } }
                                    div { class: "form-group",
                                        label { class: "form-label", "平台名称" }
                                        input { class: "input", value: "{platform_name}" }
                                        p { class: "form-hint", "显示在浏览器标签和邮件通知中" }
                                    }
                                    div { class: "form-group",
                                        label { class: "form-label", "默认语言" }
                                        select { class: "input select",
                                            option { "中文 (zh-CN)" }
                                            option { "English (en-US)" }
                                            option { "日本語 (ja-JP)" }
                                        }
                                    }
                                    div { class: "form-group",
                                        label { class: "form-label", "时区" }
                                        select { class: "input select",
                                            option { "Asia/Shanghai (UTC+8)" }
                                            option { "America/New_York (UTC-5)" }
                                            option { "Europe/London (UTC+0)" }
                                        }
                                    }
                                    div { style: "margin-top:4px;",
                                        button {
                                            class: "btn btn-primary",
                                            disabled: saving(),
                                            onclick: move |_| {
                                                saving.set(true);
                                                save_msg.set(String::new());
                                                let body = serde_json::json!({
                                                    "platform_name": platform_name.clone(),
                                                    "default_language": "zh-CN",
                                                    "timezone": "Asia/Shanghai",
                                                    "default_visibility": "private"
                                                });
                                                spawn(async move {
                                                    match settings_api::update_general(&body).await {
                                                        Ok(_) => save_msg.set("已保存".to_string()),
                                                        Err(e) => save_msg.set(format!("保存失败：{}", e)),
                                                    }
                                                    saving.set(false);
                                                });
                                            },
                                            if saving() { "保存中…" } else { "保存更改" }
                                        }
                                    }
                                }
                            }

                            if active_tab() == "ai" {
                                div { class: "card",
                                    div { class: "card-header", h3 { "AI 配置" } }
                                    div { class: "form-group",
                                        label { class: "form-label", "默认 AI 模型" }
                                        select { class: "input select",
                                            option { selected: default_model == "claude-3-5-sonnet", value: "claude-3-5-sonnet", "Claude-3.5 Sonnet" }
                                            option { selected: default_model == "gpt-4o", value: "gpt-4o", "GPT-4o" }
                                            option { "Gemini Pro" }
                                        }
                                    }
                                    div { class: "form-group",
                                        label { class: "form-label", "API 密钥（Anthropic）" }
                                        input { class: "input", r#type: "password", placeholder: "sk-ant-••••••••••••••••" }
                                    }
                                    div { class: "form-group",
                                        label { class: "form-label", "AI 任务并发数" }
                                        input { class: "input", r#type: "number", value: "4" }
                                    }
                                    button {
                                        class: "btn btn-primary",
                                        style: "margin-top:8px;",
                                        disabled: saving(),
                                        onclick: move |_| {
                                            saving.set(true);
                                            save_msg.set(String::new());
                                            let body = serde_json::json!({ "default_model": default_model.clone() });
                                            spawn(async move {
                                                match settings_api::update_ai(&body).await {
                                                    Ok(_) => save_msg.set("已保存".to_string()),
                                                    Err(e) => save_msg.set(format!("保存失败：{}", e)),
                                                }
                                                saving.set(false);
                                            });
                                        },
                                        if saving() { "保存中…" } else { "保存更改" }
                                    }
                                }
                            }

                            if active_tab() == "notifications" {
                                div { class: "card",
                                    div { class: "card-header", h3 { "通知偏好" } }
                                    ToggleSettingSignal { label: "邮件通知", desc: "变更请求、成员邀请等重要事件", checked: notif_email(), onchange: move |v| notif_email.set(v) }
                                    ToggleSettingSignal { label: "浏览器通知", desc: "实时推送 AI 任务完成等通知", checked: notif_browser(), onchange: move |v| notif_browser.set(v) }
                                    ToggleSettingSignal { label: "AI 任务通知", desc: "AI 翻译、摘要任务完成时通知", checked: notif_ai(), onchange: move |v| notif_ai.set(v) }
                                    button { class: "btn btn-primary", style: "margin-top:8px;", disabled: saving(), onclick: save_notifications,
                                        if saving() { "保存中…" } else { "保存偏好" }
                                    }
                                }
                            }

                            if active_tab() == "security" {
                                div { class: "card",
                                    div { class: "card-header", h3 { "安全设置" } }
                                    div { class: "form-group",
                                        label { class: "form-label", "会话超时（分钟）" }
                                        input { class: "input", r#type: "number", value: "{session_timeout}" }
                                    }
                                    div { style: "padding:14px 16px;background:var(--danger-light);border:1px solid #fca5a5;border-radius:var(--radius-sm);margin-top:16px;",
                                        p { style: "font-weight:600;color:var(--danger);margin-bottom:4px;", "危险操作" }
                                        p { style: "font-size:13px;color:var(--danger);", "以下操作不可逆，请谨慎执行" }
                                        div { style: "display:flex;gap:8px;margin-top:10px;",
                                            button { class: "btn btn-sm btn-danger", "清除所有会话" }
                                        }
                                    }
                                }
                            }

                            if active_tab() == "appearance" {
                                div { class: "card",
                                    div { class: "card-header", h3 { "外观设置" } }
                                    ToggleSettingSignal { label: "深色模式", desc: "切换到深色主题", checked: dark_mode(), onchange: move |v| dark_mode.set(v) }
                                    div { class: "form-group", style: "margin-top:16px;",
                                        label { class: "form-label", "主色调" }
                                        div { style: "display:flex;gap:10px;margin-top:8px;",
                                            ColorSwatch { color: "#4f46e5", label: "靛蓝（默认）" }
                                            ColorSwatch { color: "#2563eb", label: "蓝色" }
                                            ColorSwatch { color: "#059669", label: "绿色" }
                                            ColorSwatch { color: "#dc2626", label: "红色" }
                                        }
                                    }
                                    button { class: "btn btn-primary", style: "margin-top:8px;", disabled: saving(), onclick: save_appearance,
                                        if saving() { "保存中…" } else { "保存外观" }
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
fn SettingsNavItem(
    id: &'static str,
    label: &'static str,
    icon: &'static str,
    active: &'static str,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div {
            class: if active == id { "nav-item active" } else { "nav-item" },
            onclick: move |e| onclick.call(e),
            span { class: "nav-icon", "{icon}" }
            "{label}"
        }
    }
}

#[component]
fn ToggleSettingSignal(
    label: &'static str,
    desc: &'static str,
    checked: bool,
    onchange: EventHandler<bool>,
) -> Element {
    rsx! {
        div { style: "display:flex;align-items:center;justify-content:space-between;padding:12px 0;border-bottom:1px solid var(--line);",
            div {
                p { style: "font-size:13.5px;font-weight:500;", "{label}" }
                p { style: "font-size:12px;color:var(--muted);margin-top:2px;", "{desc}" }
            }
            div {
                class: if checked { "toggle on" } else { "toggle" },
                onclick: move |_| onchange.call(!checked),
            }
        }
    }
}

#[component]
fn ColorSwatch(color: &'static str, label: &'static str) -> Element {
    rsx! {
        div { style: "display:flex;flex-direction:column;align-items:center;gap:6px;cursor:pointer;",
            div { style: "width:32px;height:32px;border-radius:50%;background:{color};border:3px solid transparent;box-shadow:0 0 0 2px {color};" }
            span { style: "font-size:11px;color:var(--muted);", "{label}" }
        }
    }
}
