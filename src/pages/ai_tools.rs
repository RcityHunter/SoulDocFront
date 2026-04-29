use crate::api::tool_configs as cfg_api;
use dioxus::prelude::*;
use serde_json::Value;

#[component]
pub fn AiTools() -> Element {
    let configs_res = use_resource(|| async move { cfg_api::list_tool_configs().await });

    rsx! {
        document::Title { "AI 工具配置 — SoulBook" }
        div { class: "page-content",
            div { class: "page-header",
                div { class: "page-header-left",
                    h1 { "🛠️ AI 工具配置" }
                    p { "管理 AI 能力族、模型绑定和执行策略" }
                }
            }

            div { class: "highlight-block ai-highlight", style: "margin-bottom:24px;",
                div { style: "display:flex;align-items:flex-start;gap:14px;",
                    span { style: "font-size:24px;", "🤖" }
                    div {
                        p { style: "font-weight:700;font-size:15px;margin-bottom:6px;", "AI 是第一公民" }
                        p { style: "font-size:13.5px;color:rgba(255,255,255,.75);line-height:1.7;",
                            "AI 与人类共用 user、role、space_member 模型。AI 工具通过能力族（Capability Family）定义允许的操作范围，通过执行边界和审批链控制自动化行为。"
                        }
                    }
                }
            }

            match &*configs_res.read() {
                None => rsx! { p { style: "color:var(--muted);", "加载中…" } },
                Some(Err(e)) => rsx! { p { style: "color:#dc2626;", "加载失败：{e}" } },
                Some(Ok(configs)) => rsx! {
                    div { class: "grid-2", style: "gap:16px;",
                        for cfg in configs.iter() {
                            ToolCard { config: cfg.clone() }
                        }
                    }
                },
            }
        }
    }
}

#[component]
fn ToolCard(config: Value) -> Element {
    let id = config
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim_start_matches("tool_config:")
        .to_string();
    let title = config
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let icon = config
        .get("icon")
        .and_then(|v| v.as_str())
        .unwrap_or("🛠️")
        .to_string();
    let desc = config
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let model = config
        .get("model")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let approval = config
        .get("approval_required")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let actions: Vec<String> = config
        .get("actions")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    let mut enabled = use_signal(|| {
        config
            .get("enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(true)
    });
    let mut show_config = use_signal(|| false);
    let mut test_result = use_signal(|| String::new());

    let id_clone = id.clone();
    let title_clone = title.clone();
    let model_clone = model.clone();

    let do_test = move |_| {
        let id = id_clone.clone();
        let t = title_clone.clone();
        let m = model_clone.clone();
        test_result.set("正在测试…".to_string());
        spawn(async move {
            match cfg_api::test_tool_config(&id).await {
                Ok(_) => test_result.set(format!(
                    "✅ {} 配置正常\n模型：{}\n状态：已就绪\n执行模式：{}",
                    t,
                    m,
                    if approval {
                        "需要审批"
                    } else {
                        "自动执行"
                    }
                )),
                Err(e) => test_result.set(format!("❌ 测试失败：{}", e)),
            }
        });
    };

    rsx! {
        div { class: "tool-card",
            div { class: "tool-card-header",
                div { class: "tool-icon-wrap", style: "background:var(--primary-light);", "{icon}" }
                div {
                    h3 { style: "font-size:15px;font-weight:600;margin-bottom:3px;", "{title}" }
                    p { style: "font-size:12px;color:var(--muted);", "{model}" }
                }
                div { style: "margin-left:auto;",
                    div {
                        class: if enabled() { "toggle on" } else { "toggle" },
                        onclick: move |_| {
                            let new_val = !enabled();
                            enabled.set(new_val);
                            let id = id.clone();
                            spawn(async move {
                                let _ = cfg_api::update_tool_config(&id, &serde_json::json!({ "enabled": new_val })).await;
                            });
                        },
                    }
                }
            }
            p { style: "font-size:13px;color:var(--text3);margin-bottom:12px;line-height:1.6;", "{desc}" }
            div { style: "display:flex;flex-wrap:wrap;gap:6px;margin-bottom:12px;",
                for action in actions.iter() {
                    span { class: "tag", key: "{action}",
                        code { style: "font-size:11px;", "{action}" }
                    }
                }
            }
            if !test_result().is_empty() {
                div { style: "margin-bottom:10px;padding:8px 10px;background:var(--panel2);border-radius:6px;font-size:12px;white-space:pre-line;color:var(--text2);border:1px solid var(--line);",
                    "{test_result}"
                }
            }
            div { style: "display:flex;align-items:center;justify-content:space-between;",
                if approval {
                    span { class: "badge badge-warning", "需要审批" }
                } else {
                    span { class: "badge badge-success", "自动执行" }
                }
                div { style: "display:flex;gap:6px;",
                    button { class: "btn btn-sm", onclick: move |_| show_config.set(true), "配置" }
                    button { class: "btn btn-sm btn-primary", onclick: do_test, "测试" }
                }
            }
        }

        if show_config() {
            div {
                style: "position:fixed;inset:0;background:rgba(0,0,0,.4);z-index:300;display:flex;align-items:center;justify-content:center;",
                onclick: move |_| show_config.set(false),
                div {
                    class: "card",
                    style: "width:480px;padding:28px;max-height:80vh;overflow-y:auto;",
                    onclick: move |e| e.stop_propagation(),
                    h3 { style: "font-size:16px;font-weight:700;margin-bottom:20px;", "{icon} {title} — 配置" }
                    div { class: "form-group",
                        label { class: "form-label", "绑定模型" }
                        select { class: "input",
                            option { value: "claude-3-5-sonnet", selected: model == "claude-3-5-sonnet", "Claude-3.5 Sonnet" }
                            option { value: "claude-3-haiku", "Claude-3 Haiku" }
                            option { value: "gpt-4o", selected: model == "gpt-4o", "GPT-4o" }
                            option { value: "gpt-4o-mini", "GPT-4o mini" }
                        }
                    }
                    div { class: "form-group",
                        label { class: "form-label", "执行模式" }
                        select { class: "input",
                            option { value: "auto", selected: !approval, "自动执行" }
                            option { value: "approval", selected: approval, "需要审批" }
                        }
                    }
                    div { class: "form-group",
                        label { class: "form-label", "最大 Token 数" }
                        input { class: "input", r#type: "number",
                            value: "{config.get(\"max_tokens\").and_then(|v| v.as_i64()).unwrap_or(4096)}"
                        }
                    }
                    div { class: "form-group",
                        label { class: "form-label", "超时时间（秒）" }
                        input { class: "input", r#type: "number",
                            value: "{config.get(\"timeout_secs\").and_then(|v| v.as_i64()).unwrap_or(60)}"
                        }
                    }
                    div { style: "display:flex;gap:10px;justify-content:flex-end;margin-top:8px;",
                        button { class: "btn", onclick: move |_| show_config.set(false), "取消" }
                        button { class: "btn btn-primary", onclick: move |_| show_config.set(false), "保存配置" }
                    }
                }
            }
        }
    }
}
