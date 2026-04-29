use crate::api::documents as docs_api;
use crate::api::spaces as spaces_api;
use crate::models::DocumentTreeNode;
use crate::routes::Route;
use crate::state::CreateDocTrigger;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};

#[component]
pub fn Docs() -> Element {
    let spaces_res = use_resource(|| async move { spaces_api::list_spaces(1, 50).await });
    let mut selected_space    = use_signal(|| String::new());
    let mut selected_space_name = use_signal(|| String::new());
    let mut active_doc_slug   = use_signal(|| String::new());
    let mut tree_search       = use_signal(|| String::new());
    let mut space_dropdown    = use_signal(|| false);

    use_effect(move || {
        if selected_space.read().is_empty() {
            if let Some(Ok(data)) = &*spaces_res.read() {
                if let Some(first) = data.spaces.as_ref().or(data.items.as_ref()).and_then(|s| s.first()) {
                    selected_space.set(first.slug.clone());
                    selected_space_name.set(first.name.clone());
                }
            }
        }
    });

    let tree_res = use_resource(move || {
        let slug = selected_space.read().clone();
        async move {
            if slug.is_empty() { return Ok(vec![]); }
            docs_api::get_document_tree(&slug).await
        }
    });

    let doc_res = use_resource(move || {
        let space = selected_space.read().clone();
        let doc   = active_doc_slug.read().clone();
        async move {
            if space.is_empty() || doc.is_empty() { return Ok(None); }
            docs_api::get_document(&space, &doc).await.map(Some)
        }
    });

    let mut show_create = use_signal(|| false);
    let mut new_title   = use_signal(|| String::new());
    let mut new_slug    = use_signal(|| String::new());
    let mut creating    = use_signal(|| false);
    let mut create_err  = use_signal(|| String::new());
    let navigator = use_navigator();

    let mut create_trigger = use_context::<Signal<CreateDocTrigger>>();
    use_effect(move || {
        if create_trigger.read().0 {
            create_trigger.write().0 = false;
            show_create.set(true);
        }
    });

    let do_create = move |_| {
        let space = selected_space.read().clone();
        let title = new_title.read().trim().to_string();
        let slug  = new_slug.read().trim().to_string();
        if space.is_empty() || title.is_empty() || slug.is_empty() { return; }
        creating.set(true);
        create_err.set(String::new());
        spawn(async move {
            match docs_api::create_document(
                &space,
                docs_api::CreateDocumentRequest {
                    title,
                    slug: slug.clone(),
                    content: None,
                    parent_id: None,
                    status: Some("draft".to_string()),
                },
            )
            .await
            {
                Ok(_) => {
                    let _ = LocalStorage::set("editor_space", space.clone());
                    let _ = LocalStorage::set("editor_doc", slug.clone());
                    show_create.set(false);
                    navigator.replace(Route::Editor {});
                }
                Err(e) => create_err.set(e),
            }
            creating.set(false);
        });
    };

    rsx! {
        document::Title { "文档中心 — SoulBook" }

        // ── Create doc modal ─────────────────────────────────────────────────
        if show_create() {
            div {
                style: "position:fixed;inset:0;background:rgba(0,0,0,.4);z-index:300;display:flex;align-items:center;justify-content:center;",
                onclick: move |_| show_create.set(false),
                div {
                    class: "card",
                    style: "width:420px;padding:24px;",
                    onclick: move |e| e.stop_propagation(),
                    h3 { style: "font-size:15px;font-weight:700;margin-bottom:16px;", "新建文档" }
                    div { class: "form-group",
                        label { class: "form-label", "文档标题" }
                        input { class: "input", placeholder: "输入文档标题", value: "{new_title}",
                            oninput: move |e| new_title.set(e.value()) }
                    }
                    div { class: "form-group",
                        label { class: "form-label", "Slug" }
                        input { class: "input", placeholder: "document-slug", value: "{new_slug}",
                            oninput: move |e| new_slug.set(e.value()) }
                    }
                    if !create_err().is_empty() {
                        p { style: "color:#dc2626;font-size:13px;margin-bottom:10px;", "{create_err}" }
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

        // ── Three-column layout ──────────────────────────────────────────────
        div { style: "display:flex;height:100vh;overflow:hidden;background:var(--bg);",

            // ── Left: document tree panel (260 px) ───────────────────────────
            div {
                style: "width:260px;flex-shrink:0;background:var(--panel);border-right:1px solid var(--line);display:flex;flex-direction:column;overflow:hidden;",

                // Current space dropdown button
                div { style: "padding:12px 12px 0;",
                    div { style: "position:relative;",
                        button {
                            style: "display:flex;align-items:center;justify-content:space-between;width:100%;padding:8px 12px;border-radius:8px;border:1px solid var(--line);background:var(--panel2);font-size:13px;font-weight:500;color:var(--text2);cursor:pointer;",
                            onclick: move |_| { let v = *space_dropdown.read(); space_dropdown.set(!v); },
                            span {
                                style: "overflow:hidden;text-overflow:ellipsis;white-space:nowrap;",
                                if selected_space_name.read().is_empty() {
                                    "— 选择空间 —"
                                } else {
                                    "{selected_space_name}"
                                }
                            }
                            span { "▾" }
                        }
                        if *space_dropdown.read() {
                            div {
                                style: "position:absolute;top:calc(100% + 4px);left:0;right:0;background:var(--panel);border:1px solid var(--line);border-radius:10px;box-shadow:var(--shadow-md);z-index:200;overflow:hidden;max-height:200px;overflow-y:auto;",
                                match &*spaces_res.read() {
                                    None => rsx! { p { style: "padding:12px;font-size:13px;color:var(--muted);", "加载中…" } },
                                    Some(Err(_)) => rsx! { p { style: "padding:12px;font-size:13px;color:#dc2626;", "加载失败" } },
                                    Some(Ok(data)) => {
                                        let spaces = data.spaces.as_ref().or(data.items.as_ref()).cloned().unwrap_or_default();
                                        rsx! {
                                            for s in spaces.iter() {
                                                {
                                                    let slug = s.slug.clone();
                                                    let name = s.name.clone();
                                                    let is_sel = *selected_space.read() == slug;
                                                    rsx! {
                                                        button {
                                                            style: if is_sel {
                                                                "display:block;width:100%;text-align:left;padding:9px 12px;border:none;background:var(--primary-light);color:var(--primary);font-size:13px;font-weight:500;cursor:pointer;"
                                                            } else {
                                                                "display:block;width:100%;text-align:left;padding:9px 12px;border:none;background:transparent;color:var(--text3);font-size:13px;cursor:pointer;"
                                                            },
                                                            onclick: move |_| {
                                                                selected_space.set(slug.clone());
                                                                selected_space_name.set(name.clone());
                                                                active_doc_slug.set(String::new());
                                                                space_dropdown.set(false);
                                                            },
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
                    }
                }

                // Search box
                div { style: "padding:10px 12px 0;",
                    div { style: "position:relative;",
                        span { style: "position:absolute;left:9px;top:50%;transform:translateY(-50%);color:var(--muted);font-size:13px;pointer-events:none;", "🔍" }
                        input {
                            style: "width:100%;padding:7px 10px 7px 30px;border:1px solid var(--line);border-radius:8px;background:var(--panel2);font-size:12.5px;color:var(--text);outline:none;",
                            placeholder: "搜索文档…",
                            value: "{tree_search}",
                            oninput: move |e| tree_search.set(e.value()),
                        }
                    }
                }

                // Tree area (scrollable)
                div { style: "flex:1;overflow-y:auto;padding:8px 6px;",
                    match &*tree_res.read() {
                        None => rsx! { p { style: "padding:12px;font-size:13px;color:var(--muted);", "加载中…" } },
                        Some(Err(e)) => rsx! { p { style: "padding:12px;font-size:13px;color:#dc2626;", "加载失败：{e}" } },
                        Some(Ok(nodes)) if nodes.is_empty() => rsx! {
                            if selected_space.read().is_empty() {
                                p { style: "padding:12px;font-size:13px;color:var(--muted);", "请先选择空间" }
                            } else {
                                div { style: "padding:20px 12px;text-align:center;color:var(--muted);",
                                    div { style: "font-size:32px;margin-bottom:8px;", "📄" }
                                    p { style: "font-size:13px;", "暂无文档" }
                                }
                            }
                        },
                        Some(Ok(nodes)) => rsx! {
                            for node in nodes.iter() {
                                TreeNodeEl {
                                    node: node.clone(),
                                    active: active_doc_slug(),
                                    onclick: move |slug: String| active_doc_slug.set(slug)
                                }
                            }
                        },
                    }
                }

                // New document button
                div { style: "padding:10px 12px;border-top:1px solid var(--line);",
                    button {
                        style: "display:flex;align-items:center;justify-content:center;gap:6px;width:100%;padding:8px 14px;border-radius:8px;border:1px dashed var(--line2);background:transparent;font-size:13px;color:var(--muted);cursor:pointer;transition:var(--transition);",
                        disabled: selected_space.read().is_empty(),
                        onclick: move |_| show_create.set(true),
                        "＋ 新建文档"
                    }
                }
            }

            // ── Center: document content panel ───────────────────────────────
            div { style: "flex:1;display:flex;flex-direction:column;overflow:hidden;background:var(--panel2);",
                match &*doc_res.read() {
                    None => rsx! {
                        div { style: "display:flex;align-items:center;justify-content:center;height:100%;color:var(--muted);font-size:14px;",
                            "加载中…"
                        }
                    },
                    Some(Err(e)) => rsx! {
                        div { style: "display:flex;align-items:center;justify-content:center;height:100%;color:#dc2626;font-size:14px;",
                            "加载失败：{e}"
                        }
                    },
                    Some(Ok(None)) => rsx! {
                        div { style: "display:flex;flex-direction:column;align-items:center;justify-content:center;height:100%;color:var(--muted);",
                            div { style: "font-size:64px;margin-bottom:16px;", "📄" }
                            h3 { style: "font-size:16px;font-weight:600;margin-bottom:8px;color:var(--text2);", "选择一篇文档" }
                            p { style: "font-size:13px;", "从左侧文档树中选择要查看的文档" }
                        }
                    },
                    Some(Ok(Some(doc))) => rsx! {
                        // Status bar / breadcrumb
                        div {
                            style: "display:flex;align-items:center;justify-content:space-between;padding:12px 24px;border-bottom:1px solid var(--line);background:var(--panel);flex-shrink:0;",
                            // Breadcrumb
                            div { style: "display:flex;align-items:center;gap:6px;font-size:13px;color:var(--muted);",
                                Link { to: Route::Spaces {}, style: "color:var(--muted);text-decoration:none;", "空间" }
                                span { "›" }
                                span { "{selected_space}" }
                                span { "›" }
                                span { style: "color:var(--text2);font-weight:500;", "{doc.title}" }
                            }
                            // Status badge + edit button
                            div { style: "display:flex;align-items:center;gap:8px;",
                                {
                                    let status = doc.status.as_deref().unwrap_or("draft");
                                    let (cls, label) = match status {
                                        "published" => ("badge badge-success", "已发布"),
                                        "review"    => ("badge badge-warning", "审核中"),
                                        _           => ("badge badge-gray",    "草稿"),
                                    };
                                    rsx! { span { class: "{cls}", "{label}" } }
                                }
                                Link { to: Route::Editor {}, class: "btn btn-sm btn-primary", "编辑" }
                            }
                        }
                        // Content body (scrollable)
                        div { style: "flex:1;overflow-y:auto;padding:36px 48px;background:var(--panel);",
                            h1 { style: "font-size:28px;font-weight:800;letter-spacing:-.5px;margin-bottom:10px;", "{doc.title}" }
                            div { style: "display:flex;align-items:center;gap:12px;font-size:12.5px;color:var(--muted);margin-bottom:28px;padding-bottom:16px;border-bottom:1px solid var(--line);",
                                if let Some(updated_at) = &doc.updated_at {
                                    span { "🕐 {updated_at}" }
                                }
                                span { "🔗 {doc.slug}" }
                                if let Some(tags) = &doc.tags {
                                    if !tags.is_empty() {
                                        for tag in tags.iter() {
                                            span { style: "padding:2px 8px;border-radius:20px;background:var(--primary-light);color:var(--primary);font-size:11px;font-weight:500;", "{tag}" }
                                        }
                                    }
                                }
                            }
                            if let Some(content) = &doc.content {
                                if content.is_empty() {
                                    p { style: "color:var(--muted);font-size:14px;font-style:italic;", "该文档暂无内容，点击「编辑」开始写作" }
                                } else {
                                    div { style: "color:var(--text2);line-height:1.85;font-size:15px;white-space:pre-wrap;", "{content}" }
                                }
                            } else {
                                p { style: "color:var(--muted);font-size:14px;font-style:italic;", "该文档暂无内容，点击「编辑」开始写作" }
                            }
                        }
                    },
                }
            }

            // ── Right: metadata panel (280 px) ───────────────────────────────
            div {
                style: "width:280px;flex-shrink:0;background:var(--panel);border-left:1px solid var(--line);overflow-y:auto;",
                match &*doc_res.read() {
                    Some(Ok(Some(doc))) => rsx! {
                        // Doc info section
                        div { style: "padding:20px 16px 12px;border-bottom:1px solid var(--line);",
                            p { style: "font-size:10.5px;font-weight:700;text-transform:uppercase;letter-spacing:.12em;color:var(--muted2);margin-bottom:12px;", "文档信息" }
                            div { style: "display:flex;flex-direction:column;gap:10px;font-size:13px;",
                                div { style: "display:flex;justify-content:space-between;align-items:center;",
                                    span { style: "color:var(--muted);", "状态" }
                                    {
                                        let status = doc.status.as_deref().unwrap_or("draft");
                                        let (cls, label) = match status {
                                            "published" => ("badge badge-success", "已发布"),
                                            "review"    => ("badge badge-warning", "审核中"),
                                            _           => ("badge badge-gray",    "草稿"),
                                        };
                                        rsx! { span { class: "{cls}", style: "font-size:11.5px;", "{label}" } }
                                    }
                                }
                                div { style: "display:flex;justify-content:space-between;align-items:center;",
                                    span { style: "color:var(--muted);", "Slug" }
                                    span { style: "font-weight:500;font-size:12px;font-family:monospace;color:var(--text2);", "{doc.slug}" }
                                }
                                if let Some(updated_at) = &doc.updated_at {
                                    div { style: "display:flex;justify-content:space-between;align-items:center;",
                                        span { style: "color:var(--muted);", "更新时间" }
                                        span { style: "font-weight:500;font-size:11.5px;color:var(--text2);", "{updated_at}" }
                                    }
                                }
                            }
                        }
                        // Tags section
                        if let Some(tags) = &doc.tags {
                            if !tags.is_empty() {
                                div { style: "padding:16px 16px 12px;border-bottom:1px solid var(--line);",
                                    p { style: "font-size:10.5px;font-weight:700;text-transform:uppercase;letter-spacing:.12em;color:var(--muted2);margin-bottom:10px;", "标签" }
                                    div { style: "display:flex;flex-wrap:wrap;gap:6px;",
                                        for tag in tags.iter() {
                                            span { style: "padding:3px 10px;border-radius:20px;background:var(--primary-light);color:var(--primary);font-size:12px;font-weight:500;border:1px solid var(--primary-border);", "{tag}" }
                                        }
                                    }
                                }
                            }
                        }
                        // Actions section
                        div { style: "padding:16px 16px 12px;",
                            p { style: "font-size:10.5px;font-weight:700;text-transform:uppercase;letter-spacing:.12em;color:var(--muted2);margin-bottom:10px;", "操作" }
                            div { style: "display:flex;flex-direction:column;gap:6px;",
                                Link { to: Route::Versions {}, class: "btn btn-sm", style: "justify-content:center;", "版本历史" }
                                Link { to: Route::ChangeRequests {}, class: "btn btn-sm", style: "justify-content:center;", "变更请求" }
                                Link { to: Route::Editor {}, class: "btn btn-sm btn-primary", style: "justify-content:center;", "编辑文档" }
                            }
                        }
                    },
                    _ => rsx! {
                        div { style: "padding:20px 16px;",
                            p { style: "font-size:12.5px;color:var(--muted);", "选择文档后显示元数据" }
                        }
                    },
                }
            }
        }
    }
}

// ── Tree node component ───────────────────────────────────────────────────────

#[component]
fn TreeNodeEl(node: DocumentTreeNode, active: String, onclick: EventHandler<String>) -> Element {
    let mut expanded = use_signal(|| true);
    let slug    = node.slug.clone();
    let title   = node.title.clone();
    let is_active = active == slug;
    let has_children = node.children.as_ref().map(|c| !c.is_empty()).unwrap_or(false);
    let status  = node.status.as_deref().unwrap_or("draft");

    let doc_icon = match status {
        "published" => "📌",
        "review"    => "⏳",
        _           => "📝",
    };

    rsx! {
        div {
            // Node row
            div {
                style: if is_active {
                    "display:flex;align-items:center;gap:6px;padding:7px 10px;border-radius:8px;background:var(--primary-light);color:var(--primary);cursor:pointer;margin-bottom:1px;font-size:13px;font-weight:500;"
                } else {
                    "display:flex;align-items:center;gap:6px;padding:7px 10px;border-radius:8px;background:transparent;color:var(--text3);cursor:pointer;margin-bottom:1px;font-size:13px;"
                },
                // Expand toggle (only if has children)
                if has_children {
                    span {
                        style: "font-size:10px;color:var(--muted);width:14px;flex-shrink:0;text-align:center;",
                        onclick: move |e| { e.stop_propagation(); let v = *expanded.read(); expanded.set(!v); },
                        if *expanded.read() { "▼" } else { "▶" }
                    }
                } else {
                    span { style: "width:14px;flex-shrink:0;", "" }
                }
                // Icon
                span { style: "font-size:14px;flex-shrink:0;", "{doc_icon}" }
                // Title (click to open doc)
                span {
                    style: "overflow:hidden;text-overflow:ellipsis;white-space:nowrap;flex:1;",
                    onclick: {
                        let slug = slug.clone();
                        move |_| onclick.call(slug.clone())
                    },
                    "{title}"
                }
            }
            // Children (if expanded)
            if has_children && *expanded.read() {
                if let Some(children) = &node.children {
                    div { style: "padding-left:18px;",
                        for child in children.iter() {
                            TreeNodeEl {
                                node: child.clone(),
                                active: active.clone(),
                                onclick: onclick
                            }
                        }
                    }
                }
            }
        }
    }
}
