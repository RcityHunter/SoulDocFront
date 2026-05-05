use crate::api::ai_tasks as tasks_api;
use crate::api::spaces as spaces_api;
use dioxus::prelude::*;

#[component]
pub fn AiTasks() -> Element {
    let mut filter = use_signal(|| "".to_string());

    let tasks_res = use_resource(move || {
        let f = filter.read().clone();
        async move { tasks_api::list_tasks(if f.is_empty() { None } else { Some(f.as_str()) }).await }
    });

    let mut show_create = use_signal(|| false);
    let mut new_type = use_signal(|| "summarize".to_string());
    let mut new_doc = use_signal(|| String::new());
    let mut new_space = use_signal(|| String::new());
    let mut new_lang = use_signal(|| String::new());
    let mut new_model = use_signal(|| "claude-3.5".to_string());
    let mut create_err = use_signal(|| String::new());
    let mut creating = use_signal(|| false);

    let spaces_res = use_resource(|| async move { spaces_api::list_spaces(1, 50).await });

    let do_create = move |_| {
        let doc = new_doc.read().trim().to_string();
        if doc.is_empty() {
            create_err.set("请输入文档 Slug".into());
            return;
        }
        creating.set(true);
        create_err.set(String::new());
        let task_type = new_type.read().clone();
        let space_id = new_space.read().clone();
        let model = new_model.read().clone();
        let lang = new_lang.read().clone();
        spawn(async move {
            match tasks_api::create_task(tasks_api::CreateTaskRequest {
                task_type,
                document_id: doc.clone(),
                document_title: Some(doc),
                space_id: if space_id.is_empty() {
                    None
                } else {
                    Some(space_id)
                },
                model: Some(model),
                target_language: if lang.is_empty() { None } else { Some(lang) },
            })
            .await
            {
                Ok(_) => {
                    show_create.set(false);
                    new_doc.set(String::new());
                }
                Err(e) => create_err.set(e),
            }
            creating.set(false);
        });
    };

    rsx! {
        document::Title { "AI 任务中心 — SoulBook" }
        div { class: "page-content",
            div { class: "page-header",
                div { class: "page-header-left",
                    h1 { "✨ AI 任务中心" }
                    p { "管理摘要、翻译、SEO 检查等 AI 异步任务" }
                }
                div { class: "page-header-actions",
                    button { class: "btn btn-primary", onclick: move |_| show_create.set(true), "＋ 新建任务" }
                }
            }

            // Stats
            match &*tasks_res.read() {
                Some(Ok(tl)) => rsx! {
                    div { class: "grid-4", style: "margin-bottom:20px;",
                        StatCard { label: "运行中", value: tl.stats.running, color: "#eef2ff" }
                        StatCard { label: "已完成", value: tl.stats.completed, color: "#d1fae5" }
                        StatCard { label: "待执行", value: tl.stats.pending, color: "#fef3c7" }
                        StatCard { label: "失败", value: tl.stats.failed, color: "#fee2e2" }
                    }
                },
                _ => rsx! { div {} }
            }

            // Filters
            div { style: "display:flex;gap:8px;margin-bottom:16px;",
                for (val, label) in [("","全部"),("pending","待执行"),("running","运行中"),("completed","已完成"),("failed","失败")] {
                    button {
                        class: if *filter.read() == val { "btn btn-primary btn-sm" } else { "btn btn-sm" },
                        onclick: move |_| filter.set(val.to_string()),
                        "{label}"
                    }
                }
            }

            // Create modal
            if show_create() {
                div { style: "position:fixed;inset:0;background:rgba(0,0,0,.4);z-index:200;display:flex;align-items:center;justify-content:center;",
                    onclick: move |_| show_create.set(false),
                    div { class: "card", style: "width:480px;padding:24px;", onclick: move |e| e.stop_propagation(),
                        h3 { style: "font-size:15px;font-weight:700;margin-bottom:16px;", "新建 AI 任务" }
                        div { class: "form-group",
                            label { class: "form-label", "任务类型" }
                            select { class: "input", value: "{new_type}", onchange: move |e| new_type.set(e.value()),
                                option { value: "summarize", "📝 摘要生成" }
                                option { value: "translate", "🌍 文档翻译" }
                                option { value: "seo_check", "🔍 SEO 检查" }
                                option { value: "proofread", "✅ 内容校对" }
                                option { value: "faq", "❓ FAQ 生成" }
                            }
                        }
                        div { class: "form-group",
                            label { class: "form-label", "文档 Slug" }
                            input { class: "input", placeholder: "document-slug", value: "{new_doc}",
                                oninput: move |e| new_doc.set(e.value()) }
                        }
                        div { class: "form-group",
                            label { class: "form-label", "所属空间（可选）" }
                            match &*spaces_res.read() {
                                Some(Ok(data)) => {
                                    let spaces = data.spaces.as_ref().or(data.items.as_ref()).cloned().unwrap_or_default();
                                    rsx! {
                                        select { class: "input", value: "{new_space}",
                                            onchange: move |e| new_space.set(e.value()),
                                            option { value: "", "— 可选 —" }
                                            for s in spaces.iter() { option { value: "{s.slug}", "{s.name}" } }
                                        }
                                    }
                                }
                                _ => rsx! { input { class: "input", placeholder: "space-slug", value: "{new_space}", oninput: move |e| new_space.set(e.value()) } }
                            }
                        }
                        if *new_type.read() == "translate" {
                            div { class: "form-group",
                                label { class: "form-label", "目标语言" }
                                select { class: "input", value: "{new_lang}", onchange: move |e| new_lang.set(e.value()),
                                    option { value: "en-US", "English (en-US)" }
                                    option { value: "ja-JP", "日本語 (ja-JP)" }
                                    option { value: "ko-KR", "한국어 (ko-KR)" }
                                    option { value: "fr-FR", "Français (fr-FR)" }
                                    option { value: "de-DE", "Deutsch (de-DE)" }
                                }
                            }
                        }
                        div { class: "form-group",
                            label { class: "form-label", "AI 模型" }
                            select { class: "input", value: "{new_model}", onchange: move |e| new_model.set(e.value()),
                                option { value: "claude-3.5", "Claude 3.5 Sonnet" }
                                option { value: "gpt-4o", "GPT-4o" }
                            }
                        }
                        if !create_err().is_empty() {
                            p { style: "color:#dc2626;font-size:13px;margin-bottom:10px;", "{create_err}" }
                        }
                        div { style: "display:flex;gap:10px;justify-content:flex-end;",
                            button { class: "btn", onclick: move |_| show_create.set(false), "取消" }
                            button { class: "btn btn-primary", disabled: creating(), onclick: do_create,
                                if creating() { "创建中…" } else { "创建任务" }
                            }
                        }
                    }
                }
            }

            // Task list
            match &*tasks_res.read() {
                None => rsx! { div { class: "text-muted", style: "padding:40px;text-align:center;", "加载中…" } },
                Some(Err(e)) => rsx! { div { style: "color:#dc2626;padding:40px;text-align:center;", "加载失败：{e}" } },
                Some(Ok(tl)) => {
                    if tl.items.is_empty() {
                        rsx! {
                            div { style: "text-align:center;padding:60px;color:var(--muted);",
                                div { style: "font-size:48px;margin-bottom:12px;", "✨" }
                                h3 { "暂无 AI 任务" }
                                p { style: "font-size:13px;", "点击「新建任务」让 AI 帮你处理文档" }
                            }
                        }
                    } else {
                        rsx! {
                            div { class: "card",
                                table { style: "width:100%;border-collapse:collapse;",
                                    thead {
                                        tr { style: "border-bottom:1px solid var(--line);",
                                            th { style: "text-align:left;padding:10px 16px;font-size:12px;color:var(--muted);font-weight:600;", "任务" }
                                            th { style: "text-align:left;padding:10px 16px;font-size:12px;color:var(--muted);font-weight:600;", "文档" }
                                            th { style: "text-align:left;padding:10px 16px;font-size:12px;color:var(--muted);font-weight:600;", "模型" }
                                            th { style: "text-align:left;padding:10px 16px;font-size:12px;color:var(--muted);font-weight:600;", "状态" }
                                            th { style: "text-align:right;padding:10px 16px;font-size:12px;color:var(--muted);font-weight:600;", "操作" }
                                        }
                                    }
                                    tbody {
                                        for task in tl.items.iter() {
                                            TaskRow { task: task.clone() }
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
fn StatCard(label: &'static str, value: i64, color: &'static str) -> Element {
    rsx! {
        div { class: "metric-card",
            div { class: "metric-icon", style: "background:{color};", "✨" }
            div { class: "metric-value", "{value}" }
            div { class: "metric-label", "{label}" }
        }
    }
}

#[component]
fn TaskRow(task: tasks_api::AiTask) -> Element {
    let task_type = task.task_type.clone().unwrap_or_default();
    let doc = task
        .document_title
        .clone()
        .or(task.document_id.clone())
        .unwrap_or_default();
    let model = task.model.clone().unwrap_or_default();
    let status = task.status.clone().unwrap_or_default();
    let progress = task.progress.unwrap_or(0);
    let result_content = task
        .result
        .as_ref()
        .and_then(|v| v.get("content"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let result_title = task
        .result
        .as_ref()
        .and_then(|v| v.get("title"))
        .and_then(|v| v.as_str())
        .unwrap_or("AI 生成结果")
        .to_string();

    let id_str = task
        .id
        .as_ref()
        .and_then(|v| v.get("id"))
        .and_then(|v| v.as_str())
        .or_else(|| task.id.as_ref().and_then(|v| v.as_str()))
        .unwrap_or("")
        .to_string();

    let (type_icon, type_label) = match task_type.as_str() {
        "translate" => ("🌍", "翻译"),
        "summarize" => ("📝", "摘要"),
        "seo_check" => ("🔍", "SEO"),
        "proofread" => ("✅", "校对"),
        "outline" => ("📋", "大纲"),
        "extract_tags" => ("🏷", "标签"),
        "polish" => ("✨", "润色"),
        "expand" => ("📝", "扩写"),
        "compress" => ("📉", "压缩"),
        "dialog_to_doc" => ("💬", "对话转文档"),
        "faq" => ("❓", "FAQ"),
        other => ("⚡", other),
    };

    let (status_cls, status_label) = match status.as_str() {
        "completed" => ("badge badge-success", "已完成"),
        "running" => ("badge badge-primary", "运行中"),
        "failed" => ("badge", "失败"),
        "cancelled" => ("badge badge-gray", "已取消"),
        _ => ("badge badge-gray", "待执行"),
    };

    let mut acting = use_signal(|| false);
    let mut show_result = use_signal(|| false);

    let do_cancel = {
        let id = id_str.clone();
        move |_| {
            if id.is_empty() {
                return;
            }
            acting.set(true);
            let id2 = id.clone();
            spawn(async move {
                let _ = tasks_api::cancel_task(&id2).await;
                acting.set(false);
            });
        }
    };
    let do_retry = {
        let id = id_str.clone();
        move |_| {
            if id.is_empty() {
                return;
            }
            acting.set(true);
            let id2 = id.clone();
            spawn(async move {
                let _ = tasks_api::retry_task(&id2).await;
                acting.set(false);
            });
        }
    };

    rsx! {
        tr { style: "border-bottom:1px solid var(--line);",
            td { style: "padding:12px 16px;",
                div { style: "display:flex;align-items:center;gap:8px;",
                    span { style: "font-size:18px;", "{type_icon}" }
                    span { style: "font-size:13.5px;font-weight:500;", "{type_label}" }
                }
            }
            td { style: "padding:12px 16px;font-size:13px;color:var(--muted);max-width:200px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;",
                "{doc}"
            }
            td { style: "padding:12px 16px;font-size:12px;color:var(--muted);", "{model}" }
            td { style: "padding:12px 16px;",
                span { class: status_cls, "{status_label}" }
                if status == "running" && progress > 0 {
                    div { style: "height:3px;background:var(--line);border-radius:2px;margin-top:4px;width:80px;",
                        div { style: "height:3px;background:var(--primary);border-radius:2px;width:{progress}%;" }
                    }
                }
            }
            td { style: "padding:12px 16px;text-align:right;",
                div { style: "display:flex;gap:6px;justify-content:flex-end;",
                    if status == "completed" && !result_content.is_empty() {
                        button {
                            class: "btn btn-sm btn-primary",
                            onclick: move |_| show_result.set(true),
                            "查看结果"
                        }
                    }
                    if status == "running" {
                        button { class: "btn btn-sm", disabled: acting(), onclick: do_cancel, "取消" }
                    }
                    if status == "failed" || status == "cancelled" {
                        button { class: "btn btn-sm btn-primary", disabled: acting(), onclick: do_retry, "重试" }
                    }
                }
            }
        }
        if show_result() {
            div {
                style: "position:fixed;inset:0;background:rgba(15,23,42,.45);z-index:220;display:flex;align-items:center;justify-content:center;padding:20px;",
                onclick: move |_| show_result.set(false),
                div {
                    class: "card",
                    style: "width:min(760px,96vw);max-height:82vh;overflow:auto;padding:22px;",
                    onclick: move |e| e.stop_propagation(),
                    div { style: "display:flex;align-items:center;justify-content:space-between;gap:12px;margin-bottom:14px;",
                        h3 { style: "font-size:16px;font-weight:800;", "{result_title}" }
                        button { class: "btn btn-sm", onclick: move |_| show_result.set(false), "关闭" }
                    }
                    p { style: "font-size:12px;color:var(--muted);margin-bottom:10px;", "当前结果已保存在 AI 任务记录中，后续可复制到文档或标签。" }
                    pre {
                        style: "white-space:pre-wrap;background:#0f172a;color:#e2e8f0;border-radius:12px;padding:16px;font-size:13px;line-height:1.7;overflow:auto;",
                        "{result_content}"
                    }
                }
            }
        }
    }
}
