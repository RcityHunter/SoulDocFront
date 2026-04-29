use crate::api::templates as tpl_api;
use crate::routes::Route;
use dioxus::prelude::*;
use serde_json::Value;

#[component]
pub fn Templates() -> Element {
    let templates_res = use_resource(|| async move { tpl_api::list_templates().await });
    let mut show_create = use_signal(|| false);
    let mut new_name = use_signal(|| String::new());
    let mut new_category = use_signal(|| "产品".to_string());
    let mut creating = use_signal(|| false);
    let mut action_msg = use_signal(|| String::new());

    let do_create = move |_| {
        let name = new_name.read().trim().to_string();
        let cat = new_category.read().clone();
        if name.is_empty() {
            return;
        }
        creating.set(true);
        spawn(async move {
            match tpl_api::create_template(&serde_json::json!({ "name": name, "category": cat }))
                .await
            {
                Ok(_) => {
                    show_create.set(false);
                    new_name.set(String::new());
                    action_msg.set("模板已创建".to_string());
                }
                Err(e) => action_msg.set(format!("创建失败：{}", e)),
            }
            creating.set(false);
        });
    };

    rsx! {
        document::Title { "模板中心 — SoulBook" }
        div { class: "page-content",
            div { class: "page-header",
                div { class: "page-header-left",
                    h1 { "📋 模板中心" }
                    p { "管理文档模板，支持制作、切换、升级与回滚" }
                }
                div { class: "page-header-actions",
                    button { class: "btn btn-primary", onclick: move |_| show_create.set(true), "＋ 创建模板" }
                }
            }

            // Create modal
            if show_create() {
                div { style: "position:fixed;inset:0;background:rgba(0,0,0,.4);z-index:300;display:flex;align-items:center;justify-content:center;",
                    onclick: move |_| show_create.set(false),
                    div { class: "card", style: "width:420px;padding:24px;", onclick: move |e| e.stop_propagation(),
                        h3 { style: "font-size:15px;font-weight:700;margin-bottom:16px;", "创建模板" }
                        div { class: "form-group",
                            label { class: "form-label", "模板名称" }
                            input { class: "input", placeholder: "如：产品需求文档 PRD", value: "{new_name}",
                                oninput: move |e| new_name.set(e.value()) }
                        }
                        div { class: "form-group",
                            label { class: "form-label", "分类" }
                            select { class: "input", onchange: move |e| new_category.set(e.value()),
                                option { "产品" }
                                option { "技术" }
                                option { "帮助" }
                                option { "发布" }
                                option { "运营" }
                                option { "其他" }
                            }
                        }
                        div { style: "display:flex;gap:10px;justify-content:flex-end;",
                            button { class: "btn", onclick: move |_| show_create.set(false), "取消" }
                            button { class: "btn btn-primary", disabled: creating(), onclick: do_create,
                                if creating() { "创建中…" } else { "创建" }
                            }
                        }
                    }
                }
            }

            if !action_msg().is_empty() {
                div { style: "padding:10px 14px;background:var(--panel2);border:1px solid var(--line);border-radius:8px;font-size:13px;margin-bottom:16px;",
                    "{action_msg}"
                }
            }

            match &*templates_res.read() {
                None => rsx! { p { style: "color:var(--muted);", "加载中…" } },
                Some(Err(e)) => rsx! { p { style: "color:#dc2626;", "加载失败：{e}" } },
                Some(Ok(templates)) => rsx! {
                    div { class: "grid-3",
                        for tpl in templates.iter() {
                            TemplateCard { template: tpl.clone(), on_use: move |_| action_msg.set("已在编辑器中打开模板".to_string()) }
                        }
                    }
                },
            }
        }
    }
}

#[component]
fn TemplateCard(template: Value, on_use: EventHandler<()>) -> Element {
    let id = template
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim_start_matches("doc_template:")
        .to_string();
    let icon = template
        .get("icon")
        .and_then(|v| v.as_str())
        .unwrap_or("📋")
        .to_string();
    let name = template
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let category = template
        .get("category")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let usage = template
        .get("usage_count")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let desc = template
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    rsx! {
        div { class: "card",
            div { style: "display:flex;align-items:flex-start;justify-content:space-between;margin-bottom:10px;",
                div { style: "width:44px;height:44px;border-radius:12px;background:var(--primary-light);display:flex;align-items:center;justify-content:center;font-size:20px;",
                    "{icon}"
                }
                span { class: "badge badge-gray", "{category}" }
            }
            h3 { style: "font-size:15px;font-weight:600;margin-bottom:6px;", "{name}" }
            p { style: "font-size:13px;color:var(--muted);margin-bottom:14px;line-height:1.6;", "{desc}" }
            div { style: "display:flex;align-items:center;justify-content:space-between;",
                span { style: "font-size:12px;color:var(--muted);", "使用 {usage} 次" }
                div { style: "display:flex;gap:6px;",
                    button { class: "btn btn-sm", "预览" }
                    Link {
                        to: Route::Editor {},
                        class: "btn btn-sm btn-primary",
                        onclick: move |_| {
                            let id = id.clone();
                            spawn(async move {
                                let _ = tpl_api::use_template(&id).await;
                            });
                            on_use.call(());
                        },
                        "使用"
                    }
                }
            }
        }
    }
}
