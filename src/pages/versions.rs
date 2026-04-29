use crate::api::documents as docs_api;
use crate::api::spaces as spaces_api;
use crate::api::versions as ver_api;
use crate::models::Version;
use dioxus::prelude::*;

#[component]
pub fn Versions() -> Element {
    let spaces_res = use_resource(|| async move { spaces_api::list_spaces(1, 50).await });
    let mut selected_space = use_signal(|| String::new());
    let mut doc_slug = use_signal(|| String::new());
    let mut doc_id = use_signal(|| String::new());

    use_effect(move || {
        if selected_space.read().is_empty() {
            if let Some(Ok(data)) = &*spaces_res.read() {
                if let Some(first) = data.spaces.as_ref().or(data.items.as_ref()).and_then(|s| s.first()) {
                    selected_space.set(first.slug.clone());
                }
            }
        }
    });

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

    let vers_res = use_resource(move || {
        let id = doc_id.read().clone();
        async move {
            if id.is_empty() {
                return Ok(vec![]);
            }
            ver_api::list_versions(&id, 1, 50).await
        }
    });

    rsx! {
        document::Title { "版本历史 — SoulBook" }
        div { class: "page-content",
            div { class: "page-header",
                div { class: "page-header-left",
                    h1 { "🕓 版本历史" }
                    p { "查看文档的版本时间线与变更记录" }
                }
            }

            // Space selector
            div { class: "card", style: "margin-bottom:16px;",
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
                                        let is_sel = *selected_space.read() == slug;
                                        rsx! {
                                            button {
                                                class: if is_sel { "btn btn-primary btn-sm" } else { "btn btn-sm" },
                                                onclick: move |_| { selected_space.set(slug.clone()); doc_id.set(String::new()); doc_slug.set(String::new()); },
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

            // Document selector
            if !selected_space.read().is_empty() {
                div { class: "card", style: "margin-bottom:16px;",
                    div { class: "card-header", h3 { "选择文档" } }
                    match &*docs_res.read() {
                        None => rsx! { p { class: "text-muted", style: "padding:12px;", "加载中…" } },
                        Some(Err(e)) => rsx! { p { style: "color:#dc2626;padding:12px;", "{e}" } },
                        Some(Ok(docs)) => {
                            if docs.is_empty() {
                                rsx! { p { style: "padding:12px;color:var(--muted);font-size:13px;", "该空间还没有文档" } }
                            } else {
                                rsx! {
                                    div { style: "display:flex;flex-wrap:wrap;gap:8px;padding:4px 0;",
                                        for doc in docs.iter() {
                                            {
                                                let slug = doc.slug.clone();
                                                let title = doc.title.clone();
                                                let id = doc.id.clone().unwrap_or_default();
                                                let is_sel = *doc_slug.read() == slug;
                                                rsx! {
                                                    button {
                                                        class: if is_sel { "btn btn-primary btn-sm" } else { "btn btn-sm" },
                                                        onclick: move |_| { doc_slug.set(slug.clone()); doc_id.set(id.clone()); },
                                                        "{title}"
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

            // Versions
            if !doc_id.read().is_empty() {
                match &*vers_res.read() {
                    None => rsx! { div { class: "text-muted", style: "padding:40px;text-align:center;", "加载中…" } },
                    Some(Err(e)) => rsx! { div { style: "color:#dc2626;padding:40px;text-align:center;", "加载失败：{e}" } },
                    Some(Ok(vers)) => {
                        if vers.is_empty() {
                            rsx! {
                                div { class: "card",
                                    div { style: "padding:40px;text-align:center;color:var(--muted);",
                                        div { style: "font-size:36px;margin-bottom:10px;", "🕓" }
                                        p { "该文档还没有版本记录" }
                                    }
                                }
                            }
                        } else {
                            rsx! {
                                div { class: "card",
                                    div { class: "card-header", h3 { "共 {vers.len()} 个版本" } }
                                    div { style: "padding:4px 0;",
                                        for ver in vers.iter() {
                                            VersionRow { ver: ver.clone() }
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
fn VersionRow(ver: Version) -> Element {
    let num = ver
        .version_number
        .map(|n| format!("v{}", n))
        .unwrap_or_else(|| "—".into());
    let label = ver.label.clone().unwrap_or_default();
    let summary = ver.summary.clone().unwrap_or_default();
    let created = ver.created_at.clone().unwrap_or_default();
    let by = ver.created_by.clone().unwrap_or_default();

    rsx! {
        div { style: "display:flex;align-items:flex-start;gap:16px;padding:16px;border-bottom:1px solid var(--line);",
            div { style: "display:flex;flex-direction:column;align-items:center;gap:4px;",
                div { style: "width:32px;height:32px;border-radius:50%;background:#eef2ff;display:flex;align-items:center;justify-content:center;font-size:12px;font-weight:700;color:var(--primary);",
                    "{num}"
                }
                div { style: "width:1px;flex:1;background:var(--line);min-height:16px;" }
            }
            div { style: "flex:1;",
                div { style: "display:flex;align-items:center;gap:8px;margin-bottom:4px;",
                    span { style: "font-size:13.5px;font-weight:600;", "{num}" }
                    if !label.is_empty() {
                        span { class: "badge badge-gray", "{label}" }
                    }
                }
                if !summary.is_empty() {
                    p { style: "font-size:13px;color:var(--text3);margin-bottom:4px;", "{summary}" }
                }
                p { style: "font-size:12px;color:var(--muted);",
                    if !by.is_empty() { "by {by} · " }
                    "{created}"
                }
            }
        }
    }
}
