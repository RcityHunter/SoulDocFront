use crate::api::publications as pub_api;
use crate::api::spaces as spaces_api;
use dioxus::prelude::*;

#[component]
pub fn Workspace() -> Element {
    let spaces_res = use_resource(|| async move { spaces_api::list_spaces(1, 50).await });
    let mut selected_id = use_signal(|| String::new());

    use_effect(move || {
        if selected_id.read().is_empty() {
            if let Some(Ok(data)) = &*spaces_res.read() {
                if let Some(first) = data.spaces.as_ref().or(data.items.as_ref()).and_then(|s| s.first()) {
                    selected_id.set(first.id.clone().unwrap_or_else(|| first.slug.clone()));
                }
            }
        }
    });

    let pubs_res = use_resource(move || {
        let id = selected_id.read().clone();
        async move {
            if id.is_empty() {
                return Ok(vec![]);
            }
            pub_api::list_publications(&id).await
        }
    });

    rsx! {
        document::Title { "发布站点 — SoulBook" }
        div { class: "page-content",
            div { class: "page-header",
                div { class: "page-header-left",
                    h1 { "🖥️ 发布站点" }
                    p { "管理公开站点、站点分区与 Space 链接关系" }
                }
            }

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
                                        let id = space.id.clone().unwrap_or_else(|| space.slug.clone());
                                        let name = space.name.clone();
                                        let is_sel = *selected_id.read() == id;
                                        rsx! {
                                            button {
                                                class: if is_sel { "btn btn-primary btn-sm" } else { "btn btn-sm" },
                                                onclick: move |_| selected_id.set(id.clone()),
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

            // Publications
            if !selected_id.read().is_empty() {
                match &*pubs_res.read() {
                    None => rsx! { div { class: "text-muted", style: "padding:40px;text-align:center;", "加载中…" } },
                    Some(Err(e)) => rsx! { div { style: "color:#dc2626;padding:40px;text-align:center;", "加载失败：{e}" } },
                    Some(Ok(pubs)) => {
                        if pubs.is_empty() {
                            rsx! {
                                div { style: "text-align:center;padding:60px;color:var(--muted);",
                                    div { style: "font-size:48px;margin-bottom:12px;", "🖥️" }
                                    h3 { "暂无发布站点" }
                                    p { style: "font-size:13px;", "该空间还没有发布任何站点" }
                                }
                            }
                        } else {
                            rsx! {
                                div { class: "grid-2",
                                    for pub_ in pubs.iter() {
                                        div { class: "card",
                                            div { style: "display:flex;align-items:center;justify-content:space-between;margin-bottom:14px;",
                                                div { style: "display:flex;align-items:center;gap:10px;",
                                                    div { style: "width:40px;height:40px;border-radius:10px;background:var(--gradient);display:flex;align-items:center;justify-content:center;font-size:16px;",
                                                        "🌐"
                                                    }
                                                    div {
                                                        p { style: "font-weight:600;", "{pub_.name.as_deref().unwrap_or(\"-\")}" }
                                                        p { style: "font-size:12px;color:var(--muted);", "{pub_.slug.as_deref().unwrap_or(\"-\")}" }
                                                    }
                                                }
                                                {
                                                    let status = pub_.status.as_deref().unwrap_or("draft");
                                                    let (cls, label) = match status {
                                                        "published" => ("badge badge-success", "已发布"),
                                                        "unpublished" => ("badge badge-gray", "已下线"),
                                                        _ => ("badge badge-warning", "草稿"),
                                                    };
                                                    rsx! { span { class: cls, "{label}" } }
                                                }
                                            }
                                            if let Some(domain) = &pub_.domain {
                                                p { style: "font-size:12px;color:var(--muted);margin-bottom:12px;", "🌐 {domain}" }
                                            }
                                            if let Some(desc) = &pub_.description {
                                                p { style: "font-size:13px;color:var(--text3);margin-bottom:12px;", "{desc}" }
                                            }
                                            if let Some(updated_at) = &pub_.updated_at {
                                                p { style: "font-size:12px;color:var(--muted);", "更新：{updated_at}" }
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
