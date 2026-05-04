use crate::api::spaces as spaces_api;
use crate::models::Space;
use crate::routes::Route;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};

fn normalize_slug(input: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;

    for ch in input.trim().to_lowercase().chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch);
            last_was_dash = false;
        } else if !last_was_dash && !slug.is_empty() {
            slug.push('-');
            last_was_dash = true;
        }
    }

    let slug = slug.trim_matches('-').chars().take(50).collect::<String>();
    let slug = slug.trim_matches('-').to_string();
    if slug.is_empty() {
        let suffix = (js_sys::Date::now() as u64) % 1_000_000;
        format!("space-{suffix}")
    } else {
        slug
    }
}

#[component]
pub fn Spaces() -> Element {
    let mut spaces_epoch = use_signal(|| 0u32);
    let spaces_res = use_resource(move || async move {
        let _ = spaces_epoch();
        spaces_api::list_spaces(1, 50).await
    });

    let mut show_create = use_signal(|| false);
    let mut new_name = use_signal(|| String::new());
    let mut new_slug = use_signal(|| String::new());
    let mut new_desc = use_signal(|| String::new());
    let mut create_err = use_signal(|| String::new());
    let mut creating = use_signal(|| false);

    use_effect(move || {
        if LocalStorage::get::<String>("soulbook_open_create_space")
            .ok()
            .as_deref()
            == Some("1")
        {
            LocalStorage::delete("soulbook_open_create_space");
            show_create.set(true);
        }
    });

    let do_create = move |_| {
        let name = new_name.read().trim().to_string();
        let raw_slug = new_slug.read().trim().to_string();
        let slug = normalize_slug(if raw_slug.is_empty() { &name } else { &raw_slug });
        if name.is_empty() {
            return;
        }
        creating.set(true);
        create_err.set(String::new());
        spawn(async move {
            let req = crate::models::CreateSpaceRequest {
                name,
                slug,
                description: Some(new_desc.read().trim().to_string()),
                is_public: false,
            };
            match spaces_api::create_space(req).await {
                Ok(_) => {
                    show_create.set(false);
                    new_name.set(String::new());
                    new_slug.set(String::new());
                    new_desc.set(String::new());
                    spaces_epoch.set(spaces_epoch() + 1);
                }
                Err(e) => create_err.set(e),
            }
            creating.set(false);
        });
    };

    rsx! {
        document::Title { "知识空间 — SoulBook" }
        div { class: "page-content",
            div { class: "page-header",
                div { class: "page-header-left",
                    h1 { "🗂️ 知识空间" }
                    p { "管理您的个人空间、组织空间与内容项目，明确个人沉淀和组织协作的边界" }
                }
                div { class: "page-header-actions",
                    button { class: "btn btn-primary", onclick: move |_| show_create.set(true), "＋ 创建空间" }
                }
            }

            // Create space modal
            if show_create() {
                div { style: "position:fixed;inset:0;background:rgba(0,0,0,.4);z-index:200;display:flex;align-items:center;justify-content:center;",
                    onclick: move |_| show_create.set(false),
                    div { class: "card", style: "width:460px;padding:28px;",
                        onclick: move |e| e.stop_propagation(),
                        h3 { style: "font-size:16px;font-weight:700;margin-bottom:18px;", "创建知识空间" }
                        div { class: "form-group",
                            label { class: "form-label", "名称" }
                            input { class: "input", placeholder: "空间名称", value: "{new_name}",
                                oninput: move |e| new_name.set(e.value()) }
                        }
                        div { class: "form-group",
                            label { class: "form-label", "Slug (URL 标识)" }
                            input { class: "input", placeholder: "my-space", value: "{new_slug}",
                                oninput: move |e| new_slug.set(normalize_slug(&e.value())) }
                            p { style: "font-size:12px;color:var(--muted);margin-top:6px;",
                                "只能包含小写字母、数字和连字符；不填会根据名称自动生成。"
                            }
                        }
                        div { class: "form-group",
                            label { class: "form-label", "描述（可选）" }
                            input { class: "input", placeholder: "简单描述这个空间的用途", value: "{new_desc}",
                                oninput: move |e| new_desc.set(e.value()) }
                        }
                        if !create_err().is_empty() {
                            p { style: "color:#dc2626;font-size:13px;margin-bottom:12px;", "{create_err}" }
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

            // Space list
            match &*spaces_res.read() {
                None => rsx! { div { class: "text-muted", style: "padding:40px;text-align:center;", "加载中…" } },
                Some(Err(e)) => rsx! { div { style: "color:#dc2626;padding:40px;text-align:center;", "加载失败：{e}" } },
                Some(Ok(data)) => {
                    let spaces: Vec<Space> = data.spaces.as_ref()
                        .or(data.items.as_ref())
                        .cloned()
                        .unwrap_or_default();
                    if spaces.is_empty() {
                        rsx! {
                            div { style: "text-align:center;padding:60px;color:var(--muted);",
                                div { style: "font-size:48px;margin-bottom:12px;", "🗂️" }
                                h3 { "还没有空间" }
                                p { style: "font-size:13px;", "点击「创建空间」开始" }
                            }
                        }
                    } else {
                        rsx! {
                            div { style: "margin-bottom:24px;",
                                div { style: "display:flex;align-items:center;justify-content:space-between;margin-bottom:14px;",
                                    h3 { style: "font-size:15px;font-weight:600;", "所有空间 ({spaces.len()})" }
                                }
                                div { class: "grid-3",
                                    for space in spaces {
                                        SpaceCard { space }
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
fn SpaceCard(space: Space) -> Element {
    let desc = space.description.clone().unwrap_or_default();
    let docs = space.doc_count.unwrap_or(0);
    let members = space.member_count.unwrap_or(0);
    let tags = space.tag_count.unwrap_or(0);

    rsx! {
        Link { to: Route::SpaceOverview {},
            div { class: "space-card",
                div { style: "display:flex;align-items:center;justify-content:space-between;",
                    span { style: "font-size:28px;", "🗂️" }
                    div { style: "display:flex;gap:6px;flex-wrap:wrap;justify-content:flex-end;",
                        if space.is_public {
                            span { class: "badge badge-success", "公共" }
                        } else {
                            span { class: "badge badge-gray", "私有" }
                        }
                    }
                }
                div {
                    h3 { "{space.name}" }
                    p { "{desc}" }
                }
                div { class: "space-card-meta",
                    span { "📄 {docs} 文档" }
                    span { "👥 {members} 成员" }
                    span { "🏷️ {tags} 标签" }
                }
                div { class: "space-card-actions",
                    button { class: "btn btn-sm", onclick: move |e| e.stop_propagation(), "打开" }
                }
            }
        }
    }
}
