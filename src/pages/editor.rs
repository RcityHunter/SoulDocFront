use crate::api::ai_tasks;
use crate::api::documents as docs_api;
use crate::api::spaces as spaces_api;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};

// JS helper: wrap selected text in the editor textarea
fn fmt_inline_js(prefix: &str, suffix: &str, placeholder: &str) -> String {
    format!(
        r#"(function(){{
  var ta=document.getElementById('soulbook-editor-ta');
  if(!ta){{dioxus.send(null);return;}}
  var s=ta.selectionStart,e=ta.selectionEnd,v=ta.value;
  var sel=v.substring(s,e)||'{placeholder}';
  var rep='{prefix}'+sel+'{suffix}';
  var nv=v.substring(0,s)+rep+v.substring(e);
  dioxus.send(nv);
}})();"#
    )
}

// JS helper: prepend current line with a prefix (for headings, lists, quotes)
fn fmt_line_js(prefix: &str) -> String {
    format!(
        r#"(function(){{
  var ta=document.getElementById('soulbook-editor-ta');
  if(!ta){{dioxus.send(null);return;}}
  var s=ta.selectionStart,v=ta.value;
  var ls=v.lastIndexOf('\n',s-1)+1;
  var le=v.indexOf('\n',s); if(le===-1) le=v.length;
  var line=v.substring(ls,le);
  var stripped=line.replace(/^(#{{1,3}} |> |- |\d+\. )/,'');
  var nv=v.substring(0,ls)+'{prefix}'+stripped+v.substring(le);
  dioxus.send(nv);
}})();"#
    )
}

// JS helper: wrap with block delimiters (code block, etc.)
fn fmt_block_js(open: &str, close: &str, placeholder: &str) -> String {
    format!(
        r#"(function(){{
  var ta=document.getElementById('soulbook-editor-ta');
  if(!ta){{dioxus.send(null);return;}}
  var s=ta.selectionStart,e=ta.selectionEnd,v=ta.value;
  var sel=v.substring(s,e)||'{placeholder}';
  var rep='\n{open}\n'+sel+'\n{close}\n';
  dioxus.send(v.substring(0,s)+rep+v.substring(e));
}})();"#
    )
}

// JS helper: insert colored span
fn fmt_color_js(color: &str) -> String {
    format!(
        r#"(function(){{
  var ta=document.getElementById('soulbook-editor-ta');
  if(!ta){{dioxus.send(null);return;}}
  var s=ta.selectionStart,e=ta.selectionEnd,v=ta.value;
  var sel=v.substring(s,e)||'彩色文字';
  var rep='<span style="color:{color}">'+sel+'</span>';
  dioxus.send(v.substring(0,s)+rep+v.substring(e));
}})();"#
    )
}

fn fmt_size_js(size: &str) -> String {
    format!(
        r#"(function(){{
  var ta=document.getElementById('soulbook-editor-ta');
  if(!ta){{dioxus.send(null);return;}}
  var s=ta.selectionStart,e=ta.selectionEnd,v=ta.value;
  var sel=v.substring(s,e)||'文字';
  var rep='<span style="font-size:{size}">'+sel+'</span>';
  dioxus.send(v.substring(0,s)+rep+v.substring(e));
}})();"#
    )
}

#[component]
pub fn Editor() -> Element {
    let spaces_res = use_resource(|| async move { spaces_api::list_spaces(1, 50).await });
    let mut selected_space = use_signal(|| String::new());
    let mut selected_doc_slug = use_signal(|| String::new());
    let mut content = use_signal(|| String::new());
    let mut title = use_signal(|| String::new());
    let mut doc_status = use_signal(|| "draft".to_string());
    let mut saving = use_signal(|| false);
    let mut save_msg = use_signal(|| String::new());
    let mut active_panel = use_signal(|| "ai");
    let mut ai_msg = use_signal(|| String::new());
    let mut ai_loading = use_signal(|| false);
    let mut word_count = use_signal(|| 0usize);

    let docs_res = use_resource(move || {
        let slug = selected_space.read().clone();
        async move {
            if slug.is_empty() {
                return Ok(vec![]);
            }
            let data = docs_api::list_documents(&slug, 1, 100).await?;
            Ok::<_, String>(data.documents.or(data.items).unwrap_or_default())
        }
    });

    let doc_res = use_resource(move || {
        let space = selected_space.read().clone();
        let doc = selected_doc_slug.read().clone();
        async move {
            if space.is_empty() || doc.is_empty() {
                return Ok(None);
            }
            docs_api::get_document(&space, &doc).await.map(Some)
        }
    });

    // Pre-select from LocalStorage when navigated from create flow
    use_effect(move || {
        if let (Ok(space), Ok(doc)) = (
            LocalStorage::get::<String>("editor_space"),
            LocalStorage::get::<String>("editor_doc"),
        ) {
            if !space.is_empty() && !doc.is_empty() {
                selected_space.set(space);
                selected_doc_slug.set(doc);
                let _ = LocalStorage::delete("editor_space");
                let _ = LocalStorage::delete("editor_doc");
            }
        }
    });

    // Sync doc content to signals
    use_effect(move || {
        if let Some(Ok(Some(doc))) = &*doc_res.read() {
            title.set(doc.title.clone());
            let c = doc.content.clone().unwrap_or_default();
            word_count.set(c.split_whitespace().count());
            content.set(c);
            doc_status.set(doc.status.clone().unwrap_or_else(|| "draft".to_string()));
        }
    });

    // Update word count as user types
    use_effect(move || {
        word_count.set(content.read().split_whitespace().count());
    });

    let mut trigger_save = move || {
        let space = selected_space.read().clone();
        let doc = selected_doc_slug.read().clone();
        if space.is_empty() || doc.is_empty() {
            return;
        }
        saving.set(true);
        save_msg.set(String::new());
        let t = title.read().clone();
        let c = content.read().clone();
        let s = doc_status.read().clone();
        spawn(async move {
            match docs_api::update_document(
                &space,
                &doc,
                docs_api::UpdateDocumentRequest {
                    title: Some(t),
                    content: Some(c),
                    status: Some(s),
                    tags: None,
                },
            )
            .await
            {
                Ok(_) => save_msg.set("✅ 已保存".to_string()),
                Err(e) => save_msg.set(format!("❌ 保存失败：{}", e)),
            }
            saving.set(false);
        });
    };
    let do_save = move |_| trigger_save();

    // Generic format action: run JS, receive new content string
    let apply_fmt = move |js: String| {
        spawn(async move {
            let mut eval = document::eval(&js);
            match eval.recv::<serde_json::Value>().await {
                Ok(serde_json::Value::String(nv)) => {
                    content.set(nv);
                    save_msg.set(String::new());
                }
                _ => {}
            }
        });
    };

    rsx! {
        document::Title { "编辑器 — SoulBook" }
        div { class: "editor-layout",

            // ── Left sidebar ──────────────────────────────────────────
            div { class: "editor-sidebar",
                div { style: "margin-bottom:12px;",
                    p { style: "font-size:11px;font-weight:600;color:var(--muted);text-transform:uppercase;letter-spacing:.08em;margin-bottom:6px;", "空间" }
                    match &*spaces_res.read() {
                        None => rsx! { p { style: "font-size:12px;color:var(--muted);", "加载中…" } },
                        Some(Err(_)) => rsx! { p { style: "font-size:12px;color:#dc2626;", "加载失败" } },
                        Some(Ok(data)) => {
                            let spaces = data.spaces.as_ref().or(data.items.as_ref()).cloned().unwrap_or_default();
                            rsx! {
                                select {
                                    class: "input",
                                    style: "font-size:12px;padding:6px 8px;",
                                    value: "{selected_space}",
                                    onchange: move |e| {
                                        selected_space.set(e.value());
                                        selected_doc_slug.set(String::new());
                                        title.set(String::new());
                                        content.set(String::new());
                                    },
                                    option { value: "", "— 选择空间 —" }
                                    for s in spaces.iter() {
                                        option {
                                            value: "{s.slug}",
                                            selected: s.slug == *selected_space.read(),
                                            "{s.name}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div { style: "margin-bottom:12px;",
                    p { style: "font-size:11px;font-weight:600;color:var(--muted);text-transform:uppercase;letter-spacing:.08em;margin-bottom:6px;", "文档" }
                    match &*docs_res.read() {
                        None => rsx! { p { style: "font-size:12px;color:var(--muted);", "加载中…" } },
                        Some(Err(_)) => rsx! { p { style: "font-size:12px;color:#dc2626;", "加载失败" } },
                        Some(Ok(docs)) => {
                            rsx! {
                                select {
                                    class: "input",
                                    style: "font-size:12px;padding:6px 8px;",
                                    onchange: move |e| {
                                        selected_doc_slug.set(e.value());
                                        title.set(String::new());
                                        content.set(String::new());
                                        save_msg.set(String::new());
                                    },
                                    option { value: "", "— 选择文档 —" }
                                    for doc in docs.iter() {
                                        option { value: "{doc.slug}", "{doc.title}" }
                                    }
                                }
                            }
                        }
                    }
                }
                if !selected_doc_slug.read().is_empty() {
                    div { style: "margin-bottom:12px;",
                        p { style: "font-size:11px;font-weight:600;color:var(--muted);text-transform:uppercase;letter-spacing:.08em;margin-bottom:6px;", "状态" }
                        select {
                            class: "input",
                            style: "font-size:12px;padding:6px 8px;",
                            value: "{doc_status}",
                            onchange: move |e| doc_status.set(e.value()),
                            option { value: "draft", selected: *doc_status.read() == "draft", "草稿" }
                            option { value: "review", selected: *doc_status.read() == "review", "审核中" }
                            option { value: "published", selected: *doc_status.read() == "published", "已发布" }
                        }
                    }
                    div { style: "font-size:12px;color:var(--muted);padding:8px 0;border-top:1px solid var(--line);margin-top:8px;",
                        p { "字数：{word_count}" }
                        p { "字符：{content.read().len()}" }
                        p { "行数：{content.read().lines().count()}" }
                    }
                }
            }

            // ── Main editor ───────────────────────────────────────────
            div { class: "editor-main",

                // Toolbar
                div { class: "editor-toolbar",
                    if !selected_doc_slug.read().is_empty() {
                        div { style: "display:flex;align-items:center;gap:4px;flex-wrap:wrap;flex:1;",

                            // Paragraph style selector
                            div { class: "toolbar-group",
                                select {
                                    style: "font-size:12.5px;padding:3px 6px;border:1px solid var(--line);border-radius:6px;background:var(--panel2);color:var(--text);height:28px;cursor:pointer;",
                                    onchange: move |e| {
                                        let js = match e.value().as_str() {
                                            "h1" => fmt_line_js("# "),
                                            "h2" => fmt_line_js("## "),
                                            "h3" => fmt_line_js("### "),
                                            _ => fmt_line_js(""),
                                        };
                                        apply_fmt(js);
                                    },
                                    option { value: "p", "正文" }
                                    option { value: "h1", "H1" }
                                    option { value: "h2", "H2" }
                                    option { value: "h3", "H3" }
                                }
                            }

                            // Inline format
                            div { class: "toolbar-group",
                                button { class: "toolbar-btn", title: "粗体 (Ctrl+B)", style: "font-weight:700;",
                                    onclick: move |_| { let js = fmt_inline_js("**", "**", "粗体"); apply_fmt(js); }, "B" }
                                button { class: "toolbar-btn", title: "斜体 (Ctrl+I)", style: "font-style:italic;",
                                    onclick: move |_| { let js = fmt_inline_js("*", "*", "斜体"); apply_fmt(js); }, "I" }
                                button { class: "toolbar-btn", title: "下划线", style: "text-decoration:underline;",
                                    onclick: move |_| { let js = fmt_inline_js("<u>", "</u>", "下划线"); apply_fmt(js); }, "U" }
                                button { class: "toolbar-btn", title: "删除线", style: "text-decoration:line-through;color:var(--muted);",
                                    onclick: move |_| { let js = fmt_inline_js("~~", "~~", "删除线"); apply_fmt(js); }, "S" }
                                button { class: "toolbar-btn", title: "行内代码", style: "font-family:monospace;font-size:11px;",
                                    onclick: move |_| { let js = fmt_inline_js("`", "`", "code"); apply_fmt(js); }, "`" }
                            }

                            // Lists
                            div { class: "toolbar-group",
                                button { class: "toolbar-btn", title: "无序列表",
                                    onclick: move |_| { let js = fmt_line_js("- "); apply_fmt(js); }, "≡" }
                                button { class: "toolbar-btn", title: "有序列表",
                                    onclick: move |_| { let js = fmt_line_js("1. "); apply_fmt(js); }, "1. 列表" }
                                button { class: "toolbar-btn", title: "水平分隔线",
                                    style: "letter-spacing:-1px;",
                                    onclick: move |_| {
                                        let js = r#"(function(){var ta=document.getElementById('soulbook-editor-ta');if(!ta){dioxus.send(null);return;}var s=ta.selectionStart,v=ta.value;dioxus.send(v.substring(0,s)+'\n---\n'+v.substring(s));})();"#;
                                        apply_fmt(js.to_string());
                                    }, "───" }
                            }

                            // Link / Image / Table / Code block
                            div { class: "toolbar-group",
                                button { class: "toolbar-btn", title: "插入链接",
                                    onclick: move |_| { let js = fmt_inline_js("[", "](https://)", "链接文字"); apply_fmt(js); }, "🔗" }
                                button { class: "toolbar-btn", title: "插入图片",
                                    onclick: move |_| { let js = fmt_inline_js("![", "](https://)", "图片描述"); apply_fmt(js); }, "🖼" }
                                button { class: "toolbar-btn", title: "插入表格",
                                    onclick: move |_| {
                                        let js = r#"(function(){var ta=document.getElementById('souldoc-editor-ta');if(!ta){dioxus.send(null);return;}var s=ta.selectionStart,v=ta.value;var t='\n| 列1 | 列2 | 列3 |\n|---|---|---|\n| 内容 | 内容 | 内容 |\n';dioxus.send(v.substring(0,s)+t+v.substring(s));})();"#;
                                        apply_fmt(js.to_string());
                                    }, "⊞" }
                                button { class: "toolbar-btn", title: "代码块", style: "font-family:monospace;font-size:11px;",
                                    onclick: move |_| { let js = fmt_block_js("```", "```", "代码"); apply_fmt(js); }, "{{}}" }
                            }

                            // AI button
                            div { class: "toolbar-group",
                                button {
                                    style: "display:flex;align-items:center;gap:4px;padding:4px 10px;border:none;border-radius:6px;background:linear-gradient(135deg,#6366f1,#8b5cf6);color:#fff;font-size:12px;font-weight:600;cursor:pointer;height:28px;",
                                    onclick: move |_| active_panel.set("ai"),
                                    "✦ AI"
                                }
                            }
                        }
                    }

                    // Right: save status + button
                    div { style: "margin-left:auto;display:flex;align-items:center;gap:8px;flex-shrink:0;",
                        if !save_msg().is_empty() {
                            span { style: "font-size:12px;color:var(--muted);", "{save_msg}" }
                        }
                        if !selected_doc_slug.read().is_empty() {
                            button {
                                class: "btn btn-sm btn-primary",
                                disabled: saving(),
                                onclick: do_save,
                                if saving() { "保存中…" } else { "保存" }
                            }
                        }
                    }
                }

                // Editor body
                div { class: "editor-body",
                    match &*doc_res.read() {
                        None if !selected_doc_slug.read().is_empty() => rsx! {
                            div { style: "display:flex;align-items:center;justify-content:center;height:100%;color:var(--muted);",
                                "加载中…"
                            }
                        },
                        Some(Err(e)) => rsx! {
                            div { style: "display:flex;align-items:center;justify-content:center;height:100%;color:#dc2626;",
                                "加载失败：{e}"
                            }
                        },
                        _ if selected_doc_slug.read().is_empty() => rsx! {
                            div { style: "display:flex;flex-direction:column;align-items:center;justify-content:center;height:100%;color:var(--muted);",
                                div { style: "font-size:48px;margin-bottom:16px;", "✏️" }
                                p { "请先选择空间和文档" }
                            }
                        },
                        _ => rsx! {
                            div { style: "max-width:720px;margin:0 auto;padding:8px 0;",
                                input {
                                    style: "font-size:28px;font-weight:800;letter-spacing:-.5px;outline:none;border:none;background:transparent;width:100%;margin-bottom:16px;color:var(--text);",
                                    placeholder: "文档标题",
                                    value: "{title}",
                                    oninput: move |e| title.set(e.value()),
                                }
                                textarea {
                                    id: "soulbook-editor-ta",
                                    style: "width:100%;min-height:calc(100vh - 260px);outline:none;border:none;background:transparent;color:var(--text2);line-height:1.9;font-size:14.5px;resize:none;font-family:inherit;tab-size:4;",
                                    placeholder: "开始写作… 支持 Markdown 与 HTML 内联样式\n\n快捷操作：工具栏可格式化选中文字，颜色/字号按钮包裹选中内容",
                                    value: "{content}",
                                    oninput: move |e| content.set(e.value()),
                                    onkeydown: move |e| {
                                        if e.modifiers().ctrl() {
                                            match e.key() {
                                                Key::Character(c) if c == "b" => {
                                                    let js = fmt_inline_js("**", "**", "粗体");
                                                    apply_fmt(js);
                                                }
                                                Key::Character(c) if c == "i" => {
                                                    let js = fmt_inline_js("*", "*", "斜体");
                                                    apply_fmt(js);
                                                }
                                                Key::Character(c) if c == "s" => {
                                                    trigger_save();
                                                }
                                                _ => {}
                                            }
                                        }
                                    },
                                }
                            }
                        },
                    }
                }
            }

            // ── Right panel ───────────────────────────────────────────
            div { class: "editor-right",
                // Tab bar
                div { class: "editor-right-tabs",
                    div {
                        class: if active_panel() == "ai" { "editor-right-tab active" } else { "editor-right-tab" },
                        onclick: move |_| active_panel.set("ai"),
                        "✦ 智能"
                    }
                    div {
                        class: if active_panel() == "lang" { "editor-right-tab active" } else { "editor-right-tab" },
                        onclick: move |_| active_panel.set("lang"),
                        "语言"
                    }
                    div {
                        class: if active_panel() == "info" { "editor-right-tab active" } else { "editor-right-tab" },
                        onclick: move |_| active_panel.set("info"),
                        "信息"
                    }
                }

                // ── 智能 tab ──────────────────────────────────────────
                if active_panel() == "ai" {
                    div { class: "editor-right-body", style: "padding:0;",

                        // 智能写作 block (dark)
                        div { style: "background:#1e1b4b;border-radius:10px;margin:12px 12px 0;padding:14px;",
                            div { style: "display:flex;align-items:center;gap:6px;margin-bottom:12px;",
                                span { style: "color:#a78bfa;font-size:14px;", "✦" }
                                span { style: "color:#e2e8f0;font-size:13px;font-weight:700;", "智能写作" }
                            }
                            {
                                let writing_tasks: &[(&str, &str, &str)] = &[
                                    ("📄", "生成文档摘要", "summarize"),
                                    ("📋", "AI 优化大纲",  "outline"),
                                    ("🏷", "智能提取标签", "extract_tags"),
                                    ("❓", "生成 FAQ",     "faq"),
                                ];
                                rsx! {
                                    div { style: "display:flex;flex-direction:column;gap:8px;",
                                        for (icon, label, task_type) in writing_tasks.iter() {
                                            {
                                                let tt = task_type.to_string();
                                                let lbl = label.to_string();
                                                let ic = icon.to_string();
                                                let space = selected_space.read().clone();
                                                let doc = selected_doc_slug.read().clone();
                                                let doc_title = title.read().clone();
                                                rsx! {
                                                    button {
                                                        style: "display:flex;align-items:center;gap:8px;padding:8px 12px;background:#312e81;border:none;border-radius:8px;color:#e2e8f0;font-size:12.5px;cursor:pointer;text-align:left;width:100%;",
                                                        disabled: ai_loading() || doc.is_empty(),
                                                        onclick: move |_| {
                                                            if doc.is_empty() { return; }
                                                            let t = tt.clone(); let l = lbl.clone();
                                                            let sp = space.clone(); let dc = doc.clone(); let dt = doc_title.clone();
                                                            ai_loading.set(true);
                                                            ai_msg.set(format!("正在创建「{}」任务…", l));
                                                            spawn(async move {
                                                                match ai_tasks::create_task(ai_tasks::CreateTaskRequest {
                                                                    task_type: t, document_id: dc,
                                                                    document_title: Some(dt), space_id: Some(sp),
                                                                    model: None, target_language: None,
                                                                }).await {
                                                                    Ok(_) => ai_msg.set(format!("✅ 「{}」任务已完成，可到 AI 任务中心查看结果", l)),
                                                                    Err(e) => ai_msg.set(format!("❌ 失败：{}", e)),
                                                                }
                                                                ai_loading.set(false);
                                                            });
                                                        },
                                                        span { style: "font-size:14px;", "{ic}" }
                                                        "{lbl}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // 写作辅助 block
                        div { style: "padding:12px 12px 4px;",
                            p { style: "font-size:11px;font-weight:600;color:var(--muted);text-transform:uppercase;letter-spacing:.08em;margin-bottom:8px;", "写作辅助" }
                            div { style: "display:flex;flex-direction:column;gap:2px;",
                                {
                                    let assist_items: &[(&str, &str, &str)] = &[
                                        ("💬", "对话转文档",   "dialog_to_doc"),
                                        ("✨", "润色选中内容", "polish"),
                                        ("📝", "扩写选中段落", "expand"),
                                        ("📉", "压缩选中段落", "compress"),
                                        ("🌐", "翻译文档",     "translate"),
                                        ("🔍", "AI 审校检查",  "proofread"),
                                    ];
                                    rsx! {
                                        for (icon, label, task_type) in assist_items.iter() {
                                            {
                                                let tt = task_type.to_string();
                                                let lbl = label.to_string();
                                                let ic = icon.to_string();
                                                let space = selected_space.read().clone();
                                                let doc = selected_doc_slug.read().clone();
                                                let doc_title = title.read().clone();
                                                rsx! {
                                                    button {
                                                        style: "display:flex;align-items:center;gap:8px;padding:7px 8px;background:transparent;border:none;border-radius:6px;color:var(--text2);font-size:12.5px;cursor:pointer;text-align:left;width:100%;",
                                                        disabled: ai_loading() || doc.is_empty(),
                                                        onclick: move |_| {
                                                            if doc.is_empty() { return; }
                                                            let t = tt.clone(); let l = lbl.clone();
                                                            let sp = space.clone(); let dc = doc.clone(); let dt = doc_title.clone();
                                                            ai_loading.set(true);
                                                            ai_msg.set(format!("正在创建「{}」任务…", l));
                                                            spawn(async move {
                                                                match ai_tasks::create_task(ai_tasks::CreateTaskRequest {
                                                                    task_type: t, document_id: dc,
                                                                    document_title: Some(dt), space_id: Some(sp),
                                                                    model: None, target_language: None,
                                                                }).await {
                                                                    Ok(_) => ai_msg.set(format!("✅ 「{}」任务已完成，可到 AI 任务中心查看结果", l)),
                                                                    Err(e) => ai_msg.set(format!("❌ 失败：{}", e)),
                                                                }
                                                                ai_loading.set(false);
                                                            });
                                                        },
                                                        span { style: "font-size:14px;width:18px;text-align:center;", "{ic}" }
                                                        "{lbl}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // 知识搜索 block
                        div { style: "padding:4px 12px 12px;border-top:1px solid var(--line);margin-top:8px;",
                            p { style: "font-size:11px;font-weight:600;color:var(--muted);text-transform:uppercase;letter-spacing:.08em;margin:10px 0 8px;", "知识搜索" }
                            button {
                                style: "width:100%;padding:8px 12px;background:var(--primary);border:none;border-radius:8px;color:#fff;font-size:12.5px;cursor:pointer;font-weight:500;",
                                "🔎 语义检索"
                            }
                        }

                        // AI result message
                        if !ai_msg().is_empty() {
                            div { style: "margin:8px 12px;font-size:12px;padding:10px 12px;background:var(--panel2);border-radius:8px;border:1px solid var(--line);line-height:1.6;",
                                "{ai_msg}"
                            }
                        }
                    }
                }

                // ── 语言 tab ──────────────────────────────────────────
                if active_panel() == "lang" {
                    div { class: "editor-right-body",
                        div { style: "padding:8px 0;",
                            p { style: "font-size:11.5px;font-weight:600;color:var(--muted);text-transform:uppercase;letter-spacing:.08em;margin-bottom:10px;", "语言版本" }
                            div { style: "display:flex;flex-direction:column;gap:8px;",
                                {
                                    let langs = [("中文 (简体)", "zh-CN", true), ("English", "en", false), ("日本語", "ja", false)];
                                    rsx! {
                                        for (name, code, active) in langs.iter() {
                                            div { style: if *active { "display:flex;align-items:center;justify-content:space-between;padding:8px 10px;border-radius:8px;border:1px solid var(--primary);background:var(--primary)0f;" } else { "display:flex;align-items:center;justify-content:space-between;padding:8px 10px;border-radius:8px;border:1px solid var(--line);background:var(--panel2);" },
                                                div {
                                                    p { style: "font-size:13px;font-weight:500;", "{name}" }
                                                    p { style: "font-size:11px;color:var(--muted);", "{code}" }
                                                }
                                                if *active {
                                                    span { style: "font-size:11px;color:var(--primary);font-weight:600;", "当前" }
                                                } else {
                                                    button { style: "font-size:11px;color:var(--muted);border:1px solid var(--line);border-radius:4px;padding:2px 8px;background:transparent;cursor:pointer;", "切换" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            div { style: "margin-top:14px;padding-top:14px;border-top:1px solid var(--line);",
                                button { style: "width:100%;padding:8px;border:1px dashed var(--line);border-radius:8px;background:transparent;color:var(--muted);font-size:12.5px;cursor:pointer;",
                                    "+ 添加语言版本"
                                }
                            }
                        }
                    }
                }

                // ── 信息 tab ──────────────────────────────────────────
                if active_panel() == "info" {
                    div { class: "editor-right-body",
                        match &*doc_res.read() {
                            Some(Ok(Some(doc))) => rsx! {
                                div { class: "form-group",
                                    label { class: "form-label", "文档 ID" }
                                    p { style: "font-size:11.5px;color:var(--muted);font-family:monospace;word-break:break-all;", "{doc.id.as_deref().unwrap_or(\"-\")}" }
                                }
                                div { class: "form-group",
                                    label { class: "form-label", "Slug" }
                                    p { style: "font-size:12px;color:var(--muted);", "{doc.slug}" }
                                }
                                div { class: "form-group",
                                    label { class: "form-label", "状态" }
                                    {
                                        let status = doc.status.as_deref().unwrap_or("draft");
                                        let (cls, lbl) = match status {
                                            "published" => ("badge badge-success", "已发布"),
                                            "review" => ("badge badge-warning", "审核中"),
                                            _ => ("badge badge-gray", "草稿"),
                                        };
                                        rsx! { span { class: cls, "{lbl}" } }
                                    }
                                }
                                if let Some(created_at) = &doc.created_at {
                                    div { class: "form-group",
                                        label { class: "form-label", "创建时间" }
                                        p { style: "font-size:11px;color:var(--muted);", "{created_at}" }
                                    }
                                }
                                if let Some(updated_at) = &doc.updated_at {
                                    div { class: "form-group",
                                        label { class: "form-label", "更新时间" }
                                        p { style: "font-size:11px;color:var(--muted);", "{updated_at}" }
                                    }
                                }
                                div { class: "form-group",
                                    label { class: "form-label", "字数统计" }
                                    p { style: "font-size:12px;color:var(--muted);",
                                        "{word_count} 词 · {content.read().len()} 字符 · {content.read().lines().count()} 行"
                                    }
                                }
                            },
                            _ => rsx! {
                                p { style: "font-size:12px;color:var(--muted);padding:12px;", "选择文档后显示信息" }
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn GuideRow(md: &'static str, preview: &'static str) -> Element {
    rsx! {
        div { style: "display:flex;justify-content:space-between;padding:2px 0;border-bottom:1px solid var(--line);",
            code { style: "font-size:11px;color:var(--primary);", "{md}" }
            span { style: "font-size:11px;color:var(--muted);", "{preview}" }
        }
    }
}
