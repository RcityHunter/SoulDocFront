use crate::api::ai_agent as agent_api;
use crate::api::developer as dev_api;
use crate::api::BASE_URL;
use dioxus::prelude::*;
use serde_json::Value;

// ── Sub-components ────────────────────────────────────────────────────────────

#[component]
fn KeyRow(
    name: String,
    prefix: String,
    id: String,
    created: String,
    scopes: Vec<String>,
    on_delete: EventHandler<String>,
) -> Element {
    rsx! {
        tr {
            td { style: "font-weight:600;color:var(--text2);", "{name}" }
            td {
                code {
                    style: "background:var(--panel2);border:1px solid var(--line);padding:2px 8px;border-radius:5px;font-size:12px;color:var(--text3);",
                    "{prefix}••••"
                }
            }
            td {
                div { style: "display:flex;gap:4px;flex-wrap:wrap;",
                    for scope in &scopes {
                        span { class: "badge badge-primary", style: "font-size:11px;", "{scope}" }
                    }
                }
            }
            td { style: "color:var(--muted);font-size:12.5px;", "{created}" }
            td {
                button {
                    class: "btn",
                    style: "color:var(--danger);border-color:transparent;font-size:12px;padding:5px 10px;",
                    onclick: {
                        let id = id.clone();
                        move |_| on_delete.call(id.clone())
                    },
                    "删除"
                }
            }
        }
    }
}

#[component]
fn AgentCard(letter: String, name: String, email: String, created: String) -> Element {
    rsx! {
        div { class: "card", style: "padding:20px;",
            div { style: "display:flex;align-items:center;gap:12px;margin-bottom:14px;",
                div {
                    style: "width:42px;height:42px;border-radius:12px;background:var(--gradient);display:flex;align-items:center;justify-content:center;color:#fff;font-size:17px;font-weight:700;flex-shrink:0;box-shadow:var(--shadow-primary);",
                    "{letter}"
                }
                div { style: "min-width:0;flex:1;",
                    div { style: "font-weight:700;font-size:14px;color:var(--text);", "{name}" }
                    div { style: "font-size:12px;color:var(--muted);margin-top:2px;", "{email}" }
                }
                span { class: "badge badge-purple", "AI" }
            }
            div { style: "display:flex;align-items:center;gap:6px;padding-top:12px;border-top:1px solid var(--line);",
                div { style: "width:6px;height:6px;border-radius:50%;background:var(--success);flex-shrink:0;" }
                span { style: "font-size:12px;color:var(--muted);", "接入于 {created}" }
            }
        }
    }
}

// ── Display data structs ──────────────────────────────────────────────────────

struct KeyData {
    id: String,
    name: String,
    prefix: String,
    created: String,
    scopes: Vec<String>,
}

struct AgentData {
    letter: String,
    name: String,
    email: String,
    created: String,
}

struct RequestData {
    reg_id: String,
    agent_name: String,
    agent_type: String,
    contact_email: String,
    description: String,
    status: String,
    reject_reason: String,
    created_at: String,
}

fn extract_keys(raw: &[Value]) -> Vec<KeyData> {
    raw.iter()
        .map(|k| {
            let raw_id = k.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let id = raw_id.strip_prefix("api_key:").unwrap_or(&raw_id).to_string();
            let name = k.get("name").and_then(|v| v.as_str()).unwrap_or("—").to_string();
            let prefix = k.get("key_prefix").and_then(|v| v.as_str()).unwrap_or("sk-sb-…").to_string();
            let created = k.get("created_at").and_then(|v| v.as_str()).and_then(|s| s.get(..10)).unwrap_or("—").to_string();
            let scopes = k.get("scopes").and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default();
            KeyData { id, name, prefix, created, scopes }
        })
        .collect()
}

fn extract_agents(raw: &[Value]) -> Vec<AgentData> {
    raw.iter()
        .map(|u| {
            let name = u.get("display_name").or_else(|| u.get("username"))
                .and_then(|v| v.as_str()).unwrap_or("AI Agent").to_string();
            let letter = name.chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_else(|| "A".to_string());
            let email = u.get("email").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let created = u.get("created_at").and_then(|v| v.as_str()).and_then(|s| s.get(..10)).unwrap_or("").to_string();
            AgentData { letter, name, email, created }
        })
        .collect()
}

fn extract_requests(raw: &[Value]) -> Vec<RequestData> {
    raw.iter()
        .map(|r| RequestData {
            reg_id:        r.get("reg_id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            agent_name:    r.get("agent_name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            agent_type:    r.get("agent_type").and_then(|v| v.as_str()).unwrap_or("custom").to_string(),
            contact_email: r.get("contact_email").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            description:   r.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            status:        r.get("status").and_then(|v| v.as_str()).unwrap_or("pending").to_string(),
            reject_reason: r.get("reject_reason").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            created_at:    r.get("created_at").and_then(|v| v.as_str()).and_then(|s| s.get(..10)).unwrap_or("").to_string(),
        })
        .collect()
}

// ── Main page ─────────────────────────────────────────────────────────────────

#[component]
pub fn AiAgent() -> Element {
    let mut active_tab     = use_signal(|| "guide");
    let mut keys_epoch     = use_signal(|| 0u32);
    let mut requests_epoch = use_signal(|| 0u32);

    let keys_res     = use_resource(move || async move { let _ = keys_epoch(); dev_api::list_api_keys().await });
    let ai_users_res = use_resource(|| async move { dev_api::list_ai_users().await });
    let health_res   = use_resource(|| async move { agent_api::get_agent_health().await });
    let requests_res = use_resource(move || async move {
        let _ = requests_epoch();
        agent_api::list_agent_requests().await
    });

    // ── Create API-key modal ──────────────────────────────────────────────
    let mut show_create_key = use_signal(|| false);
    let mut new_key_name    = use_signal(|| String::new());
    let mut creating_key    = use_signal(|| false);
    let mut created_key     = use_signal(|| String::new());
    let mut action_msg      = use_signal(|| String::new());

    // ── Agent registration form ───────────────────────────────────────────
    let mut reg_name    = use_signal(|| String::new());
    let mut reg_type    = use_signal(|| "custom".to_string());
    let mut reg_email   = use_signal(|| String::new());
    let mut reg_desc    = use_signal(|| String::new());
    let mut registering = use_signal(|| false);
    let mut reg_result  = use_signal(|| Option::<Value>::None);
    let mut reg_err     = use_signal(|| String::new());

    // ── Review modal (reject with reason) ────────────────────────────────
    let mut review_target = use_signal(|| String::new()); // reg_id being rejected
    let mut reject_reason = use_signal(|| String::new());
    let mut reviewing     = use_signal(|| false);
    let mut approved_key  = use_signal(|| String::new()); // shown after approval

    // ── Derived data ──────────────────────────────────────────────────────
    let health_ok = health_res.read().as_ref()
        .and_then(|r| r.as_ref().ok())
        .and_then(|v| v.get("data").and_then(|d| d.get("status")).and_then(|s| s.as_str()))
        .map(|s| s == "ok")
        .unwrap_or(false);

    let raw_keys: Vec<Value> = keys_res.read().as_ref()
        .and_then(|r| r.as_ref().ok()).cloned().unwrap_or_default();
    let raw_agents: Vec<Value> = ai_users_res.read().as_ref()
        .and_then(|r| r.as_ref().ok()).cloned().unwrap_or_default();
    let raw_requests: Vec<Value> = requests_res.read().as_ref()
        .and_then(|r| r.as_ref().ok())
        .and_then(|v| v.get("data").and_then(|d| d.get("items")).and_then(|i| i.as_array()).cloned())
        .unwrap_or_default();

    let key_data     = extract_keys(&raw_keys);
    let agent_data   = extract_agents(&raw_agents);
    let request_data = extract_requests(&raw_requests);

    let keys_count    = key_data.len();
    let agents_count  = agent_data.len();
    let pending_count = request_data.iter().filter(|r| r.status == "pending").count();
    let base = BASE_URL;

    let tab_active = "padding:8px 16px;border:none;background:none;border-bottom:2px solid var(--primary);color:var(--primary);font-weight:600;cursor:pointer;font-size:14px;margin-bottom:-1px;";
    let tab_idle   = "padding:8px 16px;border:none;background:none;border-bottom:2px solid transparent;color:var(--muted);cursor:pointer;font-size:14px;margin-bottom:-1px;";

    rsx! {
        document::Title { "AI Agent 接入 — SoulBook" }
        div { class: "page-content",

            // ── Header ────────────────────────────────────────────────────
            div { class: "page-header",
                div { class: "page-header-left",
                    h1 { "AI Agent 接入" }
                    p { "将外部 AI（龙虾、OpenClaw、Claude 等）接入 SoulBook，通过 API 发布和管理知识" }
                }
                div { class: "page-header-actions",
                    div {
                        style: if health_ok {
                            "display:flex;align-items:center;gap:6px;padding:5px 14px;background:#d1fae5;border:1px solid #6ee7b7;border-radius:20px;font-size:12.5px;font-weight:600;color:#065f46;"
                        } else {
                            "display:flex;align-items:center;gap:6px;padding:5px 14px;background:#fee2e2;border:1px solid #fca5a5;border-radius:20px;font-size:12.5px;font-weight:600;color:#991b1b;"
                        },
                        span { style: "width:7px;height:7px;border-radius:50%;background:currentColor;display:inline-block;flex-shrink:0;" }
                        if health_ok { "Agent 在线" } else { "Agent 离线" }
                    }
                    if active_tab() == "keys" {
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| { show_create_key.set(true); created_key.set(String::new()); },
                            "＋ 创建密钥"
                        }
                    }
                }
            }

            // ── Modal: create API key (manual) ────────────────────────────
            if show_create_key() {
                div {
                    style: "position:fixed;inset:0;background:rgba(0,0,0,.4);z-index:300;display:flex;align-items:center;justify-content:center;",
                    onclick: move |_| show_create_key.set(false),
                    div { class: "card", style: "width:440px;padding:24px;", onclick: move |e| e.stop_propagation(),
                        h3 { style: "font-size:15px;font-weight:700;margin-bottom:16px;", "创建 API 密钥" }
                        if !created_key().is_empty() {
                            div {
                                style: "margin-bottom:16px;padding:14px;background:#f0fdf4;border:1px solid #86efac;border-radius:8px;",
                                p { style: "font-weight:600;color:#16a34a;margin-bottom:8px;font-size:13.5px;", "密钥已创建，请立即保存（仅显示一次）" }
                                p {
                                    style: "font-size:12px;font-family:monospace;word-break:break-all;color:#166534;background:#dcfce7;padding:10px 12px;border-radius:6px;line-height:1.6;",
                                    "{created_key}"
                                }
                            }
                            button { class: "btn btn-primary", onclick: move |_| show_create_key.set(false), "完成" }
                        } else {
                            div { class: "form-group",
                                label { class: "form-label", "密钥用途说明" }
                                input { class: "input", placeholder: "如：龙虾 AI、昭财 Agent",
                                    value: "{new_key_name}", oninput: move |e| new_key_name.set(e.value()) }
                            }
                            div { style: "display:flex;gap:10px;justify-content:flex-end;margin-top:16px;",
                                button { class: "btn", onclick: move |_| show_create_key.set(false), "取消" }
                                button {
                                    class: "btn btn-primary", disabled: creating_key(),
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
                                                    keys_epoch.set(keys_epoch() + 1);
                                                }
                                                Err(e) => action_msg.set(format!("创建失败：{}", e)),
                                            }
                                            creating_key.set(false);
                                        });
                                    },
                                    if creating_key() { "创建中…" } else { "创建密钥" }
                                }
                            }
                        }
                    }
                }
            }

            // ── Modal: show approved API key to admin ─────────────────────
            if !approved_key().is_empty() {
                div {
                    style: "position:fixed;inset:0;background:rgba(0,0,0,.5);z-index:300;display:flex;align-items:center;justify-content:center;",
                    div { class: "card", style: "width:480px;padding:28px;",
                        h3 { style: "font-size:15px;font-weight:700;color:#16a34a;margin-bottom:4px;", "✓ 审批通过" }
                        p { style: "font-size:13px;color:var(--muted);margin-bottom:16px;",
                            "以下 API 密钥已自动生成并绑定到该 Agent 账号，仅显示一次，请通过安全渠道传递给 Agent。"
                        }
                        div {
                            style: "background:#f0fdf4;border:1px solid #86efac;border-radius:8px;padding:14px;margin-bottom:20px;",
                            p { style: "font-size:11px;font-weight:700;color:#166534;text-transform:uppercase;letter-spacing:.08em;margin-bottom:8px;", "API 密钥" }
                            p {
                                style: "font-size:13px;font-family:monospace;word-break:break-all;color:#166534;line-height:1.6;",
                                "{approved_key}"
                            }
                        }
                        button {
                            class: "btn btn-primary", style: "width:100%;",
                            onclick: move |_| { approved_key.set(String::new()); requests_epoch.set(requests_epoch() + 1); },
                            "已保存，关闭"
                        }
                    }
                }
            }

            // ── Modal: reject with reason ─────────────────────────────────
            if !review_target().is_empty() && approved_key().is_empty() {
                div {
                    style: "position:fixed;inset:0;background:rgba(0,0,0,.4);z-index:300;display:flex;align-items:center;justify-content:center;",
                    onclick: move |_| { review_target.set(String::new()); reject_reason.set(String::new()); },
                    div { class: "card", style: "width:440px;padding:24px;", onclick: move |e| e.stop_propagation(),
                        h3 { style: "font-size:15px;font-weight:700;margin-bottom:4px;", "拒绝申请" }
                        p { style: "font-size:13px;color:var(--muted);margin-bottom:16px;",
                            "请填写拒绝原因（可选），该原因将记录在申请记录中。"
                        }
                        div { class: "form-group",
                            label { class: "form-label", "拒绝原因" }
                            input { class: "input", placeholder: "如：联系邮箱无效、用途不符合规范…",
                                value: "{reject_reason}", oninput: move |e| reject_reason.set(e.value()) }
                        }
                        div { style: "display:flex;gap:10px;justify-content:flex-end;margin-top:16px;",
                            button { class: "btn", onclick: move |_| { review_target.set(String::new()); reject_reason.set(String::new()); }, "取消" }
                            button {
                                class: "btn", style: "color:#dc2626;border-color:#fca5a5;", disabled: reviewing(),
                                onclick: move |_| {
                                    let id     = review_target.read().clone();
                                    let reason = reject_reason.read().clone();
                                    reviewing.set(true);
                                    spawn(async move {
                                        match agent_api::reject_agent_request(&id, &reason).await {
                                            Ok(_)  => { review_target.set(String::new()); reject_reason.set(String::new()); requests_epoch.set(requests_epoch() + 1); }
                                            Err(e) => action_msg.set(format!("操作失败：{}", e)),
                                        }
                                        reviewing.set(false);
                                    });
                                },
                                if reviewing() { "处理中…" } else { "确认拒绝" }
                            }
                        }
                    }
                }
            }

            // ── Error banner ──────────────────────────────────────────────
            if !action_msg().is_empty() {
                div {
                    style: "margin-bottom:16px;padding:12px 16px;background:#fee2e2;border:1px solid #fca5a5;border-radius:8px;color:#991b1b;font-size:13px;display:flex;align-items:center;gap:10px;",
                    span { style: "flex:1;", "{action_msg}" }
                    button {
                        style: "font-size:12px;color:#991b1b;background:none;border:none;cursor:pointer;font-weight:600;",
                        onclick: move |_| action_msg.set(String::new()), "✕"
                    }
                }
            }

            // ── Tab bar ───────────────────────────────────────────────────
            div { style: "display:flex;gap:4px;margin-bottom:4px;border-bottom:1px solid var(--line);",
                button { style: if active_tab() == "guide"    { tab_active } else { tab_idle }, onclick: move |_| active_tab.set("guide"),    "接入指南" }
                button { style: if active_tab() == "apply"    { tab_active } else { tab_idle }, onclick: move |_| active_tab.set("apply"),    "申请接入" }
                button {
                    style: if active_tab() == "requests" { tab_active } else { tab_idle },
                    onclick: move |_| active_tab.set("requests"),
                    "待审批"
                    if pending_count > 0 {
                        span { style: "margin-left:6px;background:#fef3c7;color:#92400e;font-size:10.5px;font-weight:700;padding:1px 7px;border-radius:10px;", "{pending_count}" }
                    }
                }
                button {
                    style: if active_tab() == "keys" { tab_active } else { tab_idle },
                    onclick: move |_| active_tab.set("keys"),
                    "API 密钥"
                    if keys_count > 0 {
                        span { style: "margin-left:6px;background:var(--primary-light);color:var(--primary);font-size:10.5px;font-weight:700;padding:1px 7px;border-radius:10px;", "{keys_count}" }
                    }
                }
                button {
                    style: if active_tab() == "agents" { tab_active } else { tab_idle },
                    onclick: move |_| active_tab.set("agents"),
                    "已接入 Agent"
                    if agents_count > 0 {
                        span { style: "margin-left:6px;background:var(--primary-light);color:var(--primary);font-size:10.5px;font-weight:700;padding:1px 7px;border-radius:10px;", "{agents_count}" }
                    }
                }
            }

            // ════════════════════════════════════════════════════════════════
            // Tab: 接入指南
            // ════════════════════════════════════════════════════════════════
            if active_tab() == "guide" {
                div { style: "margin-top:24px;display:flex;flex-direction:column;gap:20px;",
                    div { style: "display:grid;grid-template-columns:repeat(3,1fr);gap:14px;",
                        div { class: "card", style: "padding:18px;",
                            div { style: "font-size:10.5px;text-transform:uppercase;letter-spacing:.1em;font-weight:700;color:var(--muted2);margin-bottom:8px;", "Base URL" }
                            div { style: "font-family:monospace;font-size:13px;color:var(--primary);font-weight:600;", "{base}" }
                            div { style: "margin-top:6px;font-size:12px;color:var(--muted);", "后端服务地址" }
                        }
                        div { class: "card", style: "padding:18px;",
                            div { style: "font-size:10.5px;text-transform:uppercase;letter-spacing:.1em;font-weight:700;color:var(--muted2);margin-bottom:8px;", "认证方式" }
                            div { style: "font-size:13px;font-weight:600;color:var(--text2);", "Bearer JWT Token" }
                            div { style: "margin-top:6px;font-size:12px;color:var(--muted);", "Token 有效期 365 天" }
                        }
                        div { class: "card", style: "padding:18px;",
                            div { style: "font-size:10.5px;text-transform:uppercase;letter-spacing:.1em;font-weight:700;color:var(--muted2);margin-bottom:8px;", "Agent API 前缀" }
                            div { style: "font-family:monospace;font-size:13px;color:var(--primary);font-weight:600;", "/agent/v1" }
                            div { style: "margin-top:6px;font-size:12px;color:var(--muted);", "专用于外部 AI 调用" }
                        }
                    }

                    div { class: "card", style: "padding:22px;",
                        div { style: "display:flex;align-items:center;gap:12px;margin-bottom:16px;",
                            div { style: "width:28px;height:28px;border-radius:50%;background:var(--gradient);color:#fff;font-size:13px;font-weight:700;display:flex;align-items:center;justify-content:center;flex-shrink:0;", "1" }
                            h3 { style: "font-size:14.5px;font-weight:700;", "Agent 提交接入申请" }
                        }
                        pre {
                            style: "background:#1e293b;color:#e2e8f0;padding:14px 18px;border-radius:10px;font-size:12px;line-height:1.8;overflow-x:auto;margin-bottom:10px;",
                            "curl -X POST {base}/agent/v1/register \\\n  -H \"Content-Type: application/json\" \\\n  -d '{{\"agent_name\":\"龙虾\",\"agent_type\":\"openclaw\",\n       \"contact_email\":\"admin@openclaw.ai\",\"description\":\"知识管理 AI\"}}'"
                        }
                        div { style: "font-size:12.5px;color:var(--muted);line-height:1.6;",
                            "返回 "
                            code { style: "background:var(--panel2);padding:1px 6px;border-radius:4px;font-size:12px;color:var(--primary);", "request_id" }
                            "，也可直接在「申请接入」Tab 填写表单提交。"
                        }
                    }

                    div { class: "card", style: "padding:22px;",
                        div { style: "display:flex;align-items:center;gap:12px;margin-bottom:16px;",
                            div { style: "width:28px;height:28px;border-radius:50%;background:var(--gradient);color:#fff;font-size:13px;font-weight:700;display:flex;align-items:center;justify-content:center;flex-shrink:0;", "2" }
                            h3 { style: "font-size:14.5px;font-weight:700;", "管理员在「待审批」Tab 审核" }
                        }
                        div { style: "font-size:12.5px;color:var(--muted);line-height:1.6;",
                            "通过后系统自动创建 AI 账号，生成 "
                            code { style: "background:var(--panel2);padding:1px 6px;border-radius:4px;font-size:12px;color:var(--primary);", "sk-sb-xxx" }
                            " 格式 API 密钥，管理员将密钥通过安全渠道传递给 Agent。"
                        }
                    }

                    div { class: "card", style: "padding:22px;",
                        div { style: "display:flex;align-items:center;gap:12px;margin-bottom:16px;",
                            div { style: "width:28px;height:28px;border-radius:50%;background:var(--gradient);color:#fff;font-size:13px;font-weight:700;display:flex;align-items:center;justify-content:center;flex-shrink:0;", "3" }
                            h3 { style: "font-size:14.5px;font-weight:700;", "Agent 轮询状态 / 配置密钥" }
                        }
                        div { style: "display:grid;grid-template-columns:1fr 1fr;gap:14px;",
                            div {
                                div { style: "font-size:11px;font-weight:700;color:var(--success);text-transform:uppercase;letter-spacing:.08em;margin-bottom:8px;", "轮询状态（无需认证）" }
                                pre { style: "background:#f8fafc;border:1px solid var(--line);border-radius:8px;padding:12px 16px;font-size:11.5px;line-height:2;color:var(--text3);",
                                    "GET /agent/v1/register/{{request_id}}\n← status: pending|approved|rejected\n← api_key: sk-sb-xxx  (仅首次返回)"
                                }
                            }
                            div {
                                div { style: "font-size:11px;font-weight:700;color:var(--muted2);text-transform:uppercase;letter-spacing:.08em;margin-bottom:8px;", "配置到 Agent" }
                                pre { style: "background:#f8fafc;border:1px solid var(--line);border-radius:8px;padding:12px 16px;font-size:11.5px;line-height:2;color:var(--text3);",
                                    "# CLAUDE.md / SOUL.md\nAuthorization: Bearer sk-sb-xxx"
                                }
                            }
                        }
                    }
                }
            }

            // ════════════════════════════════════════════════════════════════
            // Tab: 申请接入
            // ════════════════════════════════════════════════════════════════
            if active_tab() == "apply" {
                div { style: "margin-top:24px;max-width:600px;",
                    if let Some(result) = reg_result() {
                        {
                            let status  = result.get("data").and_then(|d| d.get("status")).and_then(|v| v.as_str()).unwrap_or("pending");
                            let req_id  = result.get("data").and_then(|d| d.get("request_id")).and_then(|v| v.as_str()).unwrap_or("");
                            let msg     = result.get("data").and_then(|d| d.get("message")).and_then(|v| v.as_str()).unwrap_or("");
                            rsx! {
                                div {
                                    style: "padding:28px;background:#eff6ff;border:1px solid #93c5fd;border-radius:12px;",
                                    div { style: "font-size:28px;margin-bottom:12px;", "⏳" }
                                    h3 { style: "font-size:15px;font-weight:700;margin-bottom:8px;", "申请已提交，等待审核" }
                                    p { style: "font-size:13px;color:var(--muted);margin-bottom:16px;", "{msg}" }
                                    div {
                                        style: "background:rgba(255,255,255,.8);border-radius:8px;padding:12px 14px;margin-bottom:16px;",
                                        p { style: "font-size:11px;font-weight:700;color:var(--muted2);text-transform:uppercase;letter-spacing:.08em;margin-bottom:6px;", "申请 ID（用于轮询状态）" }
                                        code { style: "font-size:12.5px;color:var(--primary);word-break:break-all;font-family:monospace;", "{req_id}" }
                                    }
                                    div {
                                        style: "background:rgba(255,255,255,.8);border-radius:8px;padding:12px 14px;margin-bottom:20px;font-size:12.5px;color:var(--muted);",
                                        p { style: "font-weight:600;color:var(--text2);margin-bottom:4px;", "轮询状态命令" }
                                        code { style: "font-size:11.5px;color:var(--text3);word-break:break-all;font-family:monospace;",
                                            "curl {base}/agent/v1/register/{req_id}"
                                        }
                                    }
                                    button { class: "btn", onclick: move |_| reg_result.set(None), "重新申请" }
                                }
                            }
                        }
                    } else {
                        div { class: "card", style: "padding:28px;",
                            h2 { style: "font-size:16px;font-weight:700;margin-bottom:4px;", "Agent 接入申请" }
                            p { style: "font-size:13px;color:var(--muted);margin-bottom:24px;",
                                "填写以下信息，管理员审核通过后自动为您生成 API 密钥（"
                                code { style: "font-size:12px;", "sk-sb-xxx" }
                                "）。"
                            }

                            div { class: "form-group",
                                label { class: "form-label", "Agent 名称 *" }
                                input { class: "input", placeholder: "如：龙虾、昭财、OpenClaw-Writer",
                                    value: "{reg_name}", oninput: move |e| reg_name.set(e.value()) }
                            }
                            div { class: "form-group",
                                label { class: "form-label", "Agent 类型" }
                                select { class: "input", value: "{reg_type}", onchange: move |e| reg_type.set(e.value()),
                                    option { value: "openclaw", "OpenClaw" }
                                    option { value: "claude",   "Claude (Anthropic)" }
                                    option { value: "gpt",      "GPT (OpenAI)" }
                                    option { value: "custom",   "自定义 / 其他" }
                                }
                            }
                            div { class: "form-group",
                                label { class: "form-label", "联系邮箱 *" }
                                input { class: "input", r#type: "email", placeholder: "admin@your-agent.ai",
                                    value: "{reg_email}", oninput: move |e| reg_email.set(e.value()) }
                            }
                            div { class: "form-group",
                                label { class: "form-label", "接入用途说明" }
                                input { class: "input", placeholder: "如：自动生成知识库文档、同步产品说明书…",
                                    value: "{reg_desc}", oninput: move |e| reg_desc.set(e.value()) }
                            }

                            if !reg_err().is_empty() {
                                p { style: "color:#dc2626;font-size:13px;margin-bottom:12px;", "{reg_err}" }
                            }

                            button {
                                class: "btn btn-primary", style: "width:100%;margin-top:8px;",
                                disabled: registering(),
                                onclick: move |_| {
                                    let name  = reg_name.read().trim().to_string();
                                    let atype = reg_type.read().clone();
                                    let email = reg_email.read().trim().to_string();
                                    let desc  = reg_desc.read().clone();
                                    if name.is_empty() || email.is_empty() {
                                        reg_err.set("Agent 名称和联系邮箱为必填项".to_string());
                                        return;
                                    }
                                    reg_err.set(String::new());
                                    registering.set(true);
                                    spawn(async move {
                                        match agent_api::agent_register(agent_api::AgentRegisterRequest {
                                            agent_name: &name, agent_type: &atype,
                                            contact_email: &email, description: &desc,
                                        }).await {
                                            Ok(v)  => { reg_result.set(Some(v)); requests_epoch.set(requests_epoch() + 1); }
                                            Err(e) => reg_err.set(format!("提交失败：{}", e)),
                                        }
                                        registering.set(false);
                                    });
                                },
                                if registering() { "提交中…" } else { "提交申请" }
                            }
                        }
                    }
                }
            }

            // ════════════════════════════════════════════════════════════════
            // Tab: 待审批
            // ════════════════════════════════════════════════════════════════
            if active_tab() == "requests" {
                div { style: "margin-top:24px;",
                    if request_data.is_empty() {
                        div { class: "empty-state",
                            div { class: "empty-state-icon", "📋" }
                            h3 { "暂无接入申请" }
                            p { "当 Agent 提交接入申请后，会在这里显示" }
                        }
                    } else {
                        div { class: "card", style: "overflow:hidden;",
                            div { class: "table-wrap",
                                table {
                                    thead {
                                        tr {
                                            th { "Agent 名称" }
                                            th { "类型" }
                                            th { "联系邮箱" }
                                            th { "用途说明" }
                                            th { "申请时间" }
                                            th { "状态" }
                                            th { style: "width:160px;", "操作" }
                                        }
                                    }
                                    tbody {
                                        for req in &request_data {
                                            {
                                                let reg_id  = req.reg_id.clone();
                                                let reg_id2 = req.reg_id.clone();
                                                let status  = req.status.clone();
                                                let reason  = req.reject_reason.clone();
                                                let is_pend = status == "pending";
                                                rsx! {
                                                    tr {
                                                        td { style: "font-weight:600;color:var(--text2);", "{req.agent_name}" }
                                                        td { span { class: "badge badge-primary", style: "font-size:11px;", "{req.agent_type}" } }
                                                        td { style: "font-size:12.5px;color:var(--muted);", "{req.contact_email}" }
                                                        td {
                                                            style: "font-size:12.5px;color:var(--muted);max-width:180px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;",
                                                            "{req.description}"
                                                        }
                                                        td { style: "font-size:12px;color:var(--muted);white-space:nowrap;", "{req.created_at}" }
                                                        td {
                                                            {
                                                                let (bg, color, label) = match status.as_str() {
                                                                    "approved" => ("#d1fae5", "#065f46", "已通过"),
                                                                    "rejected" => ("#fee2e2", "#991b1b", "已拒绝"),
                                                                    _          => ("#fef3c7", "#92400e", "待审核"),
                                                                };
                                                                rsx! {
                                                                    div {
                                                                        span {
                                                                            style: "padding:2px 10px;border-radius:20px;font-size:11.5px;font-weight:600;background:{bg};color:{color};",
                                                                            "{label}"
                                                                        }
                                                                        if !reason.is_empty() {
                                                                            p { style: "font-size:11px;color:var(--muted);margin-top:3px;", "原因：{reason}" }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        td {
                                                            if is_pend {
                                                                div { style: "display:flex;gap:6px;",
                                                                    button {
                                                                        class: "btn btn-sm btn-primary",
                                                                        onclick: move |_| {
                                                                            let id = reg_id.clone();
                                                                            reviewing.set(true);
                                                                            spawn(async move {
                                                                                match agent_api::approve_agent_request(&id).await {
                                                                                    Ok(data) => {
                                                                                        let key = data.get("data")
                                                                                            .and_then(|d| d.get("api_key"))
                                                                                            .and_then(|v| v.as_str())
                                                                                            .unwrap_or("").to_string();
                                                                                        approved_key.set(key);
                                                                                        requests_epoch.set(requests_epoch() + 1);
                                                                                    }
                                                                                    Err(e) => action_msg.set(format!("审批失败：{}", e)),
                                                                                }
                                                                                reviewing.set(false);
                                                                            });
                                                                        },
                                                                        "通过"
                                                                    }
                                                                    button {
                                                                        class: "btn btn-sm",
                                                                        style: "color:#dc2626;border-color:#fca5a5;",
                                                                        onclick: move |_| review_target.set(reg_id2.clone()),
                                                                        "拒绝"
                                                                    }
                                                                }
                                                            } else {
                                                                span { style: "font-size:12px;color:var(--muted);", "已处理" }
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

            // ════════════════════════════════════════════════════════════════
            // Tab: API 密钥
            // ════════════════════════════════════════════════════════════════
            if active_tab() == "keys" {
                div { style: "margin-top:24px;",
                    if key_data.is_empty() {
                        div { class: "empty-state",
                            div { class: "empty-state-icon", "🔑" }
                            h3 { "还没有 API 密钥" }
                            p { "为你的 AI Agent 创建一个专属密钥" }
                            button {
                                class: "btn btn-primary",
                                onclick: move |_| { show_create_key.set(true); created_key.set(String::new()); },
                                "＋ 创建 API 密钥"
                            }
                        }
                    } else {
                        div { class: "card", style: "overflow:hidden;",
                            div { class: "table-wrap",
                                table {
                                    thead {
                                        tr {
                                            th { "名称 / 用途" }
                                            th { "密钥前缀" }
                                            th { "权限范围" }
                                            th { "创建时间" }
                                            th { style: "width:80px;", "" }
                                        }
                                    }
                                    tbody {
                                        for kd in &key_data {
                                            KeyRow {
                                                key: "{kd.id}",
                                                name: kd.name.clone(), prefix: kd.prefix.clone(),
                                                id: kd.id.clone(), created: kd.created.clone(),
                                                scopes: kd.scopes.clone(),
                                                on_delete: move |del_id: String| {
                                                    spawn(async move {
                                                        match dev_api::delete_api_key(&del_id).await {
                                                            Ok(_) => keys_epoch.set(keys_epoch() + 1),
                                                            Err(e) => action_msg.set(format!("删除失败：{}", e)),
                                                        }
                                                    });
                                                },
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ════════════════════════════════════════════════════════════════
            // Tab: 已接入 Agent
            // ════════════════════════════════════════════════════════════════
            if active_tab() == "agents" {
                div { style: "margin-top:24px;",
                    if agent_data.is_empty() {
                        div { class: "empty-state",
                            div { class: "empty-state-icon", "🤖" }
                            h3 { "暂无已接入 Agent" }
                            p { "审批通过的 Agent 账号会在这里显示" }
                        }
                    } else {
                        div { style: "display:grid;grid-template-columns:repeat(auto-fill,minmax(280px,1fr));gap:14px;",
                            for ad in &agent_data {
                                AgentCard {
                                    key: "{ad.email}",
                                    letter: ad.letter.clone(), name: ad.name.clone(),
                                    email: ad.email.clone(), created: ad.created.clone(),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
