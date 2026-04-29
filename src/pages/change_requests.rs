use crate::api::change_requests as cr_api;
use crate::api::spaces as spaces_api;
use dioxus::prelude::*;

#[component]
pub fn ChangeRequests() -> Element {
    let mut active_tab = use_signal(|| "open".to_string());

    let crs_res = use_resource(move || {
        let tab = active_tab.read().clone();
        async move { cr_api::list_crs(Some(&tab), None).await }
    });

    let mut show_create = use_signal(|| false);
    let mut new_title = use_signal(|| String::new());
    let mut new_desc = use_signal(|| String::new());
    let mut new_space = use_signal(|| String::new());
    let mut new_doc = use_signal(|| String::new());
    let mut create_err = use_signal(|| String::new());
    let mut creating = use_signal(|| false);

    let spaces_res = use_resource(|| async move { spaces_api::list_spaces(1, 50).await });

    let do_create = move |_| {
        let title = new_title.read().trim().to_string();
        let space = new_space.read().trim().to_string();
        let doc = new_doc.read().trim().to_string();
        let desc = new_desc.read().trim().to_string();
        if title.is_empty() || space.is_empty() {
            return;
        }
        creating.set(true);
        create_err.set(String::new());
        spawn(async move {
            match cr_api::create_cr(cr_api::CreateCrRequest {
                title,
                description: Some(desc),
                space_id: space,
                document_id: doc.clone(),
                document_title: Some(doc),
                diff_content: None,
                reviewer_id: None,
            })
            .await
            {
                Ok(_) => {
                    show_create.set(false);
                    new_title.set(String::new());
                    new_desc.set(String::new());
                }
                Err(e) => create_err.set(e),
            }
            creating.set(false);
        });
    };

    rsx! {
        document::Title { "变更请求 — SoulBook" }
        div { class: "page-content",
            div { class: "page-header",
                div { class: "page-header-left",
                    h1 { "🧾 变更请求" }
                    p { "管理文档变更的提交、审批与合并流程" }
                }
                div { class: "page-header-actions",
                    button { class: "btn btn-primary", onclick: move |_| show_create.set(true), "＋ 提交变更请求" }
                }
            }

            // Tab bar
            div { style: "display:flex;gap:4px;margin-bottom:20px;border-bottom:1px solid var(--line);",
                for (tab, label) in [("open","待审核"),("merged","已合并"),("closed","已关闭")] {
                    button {
                        style: if *active_tab.read() == tab {
                            "padding:8px 16px;border:none;background:none;border-bottom:2px solid var(--primary);color:var(--primary);font-weight:600;cursor:pointer;"
                        } else {
                            "padding:8px 16px;border:none;background:none;border-bottom:2px solid transparent;color:var(--muted);cursor:pointer;"
                        },
                        onclick: move |_| active_tab.set(tab.to_string()),
                        "{label}"
                    }
                }
            }

            // Create modal
            if show_create() {
                div { style: "position:fixed;inset:0;background:rgba(0,0,0,.4);z-index:200;display:flex;align-items:center;justify-content:center;",
                    onclick: move |_| show_create.set(false),
                    div { class: "card", style: "width:480px;padding:24px;", onclick: move |e| e.stop_propagation(),
                        h3 { style: "font-size:15px;font-weight:700;margin-bottom:16px;", "提交变更请求" }
                        div { class: "form-group",
                            label { class: "form-label", "标题" }
                            input { class: "input", placeholder: "简述此次变更", value: "{new_title}",
                                oninput: move |e| new_title.set(e.value()) }
                        }
                        div { class: "form-group",
                            label { class: "form-label", "空间 ID" }
                            match &*spaces_res.read() {
                                Some(Ok(data)) => {
                                    let spaces = data.spaces.as_ref().or(data.items.as_ref()).cloned().unwrap_or_default();
                                    rsx! {
                                        select { class: "input", value: "{new_space}",
                                            onchange: move |e| new_space.set(e.value()),
                                            option { value: "", "— 选择空间 —" }
                                            for s in spaces.iter() {
                                                option { value: "{s.slug}", "{s.name}" }
                                            }
                                        }
                                    }
                                }
                                _ => rsx! { input { class: "input", placeholder: "space-slug", value: "{new_space}", oninput: move |e| new_space.set(e.value()) } }
                            }
                        }
                        div { class: "form-group",
                            label { class: "form-label", "文档 Slug（可选）" }
                            input { class: "input", placeholder: "document-slug", value: "{new_doc}",
                                oninput: move |e| new_doc.set(e.value()) }
                        }
                        div { class: "form-group",
                            label { class: "form-label", "描述（可选）" }
                            textarea { class: "input", style: "min-height:80px;", placeholder: "说明变更内容…",
                                value: "{new_desc}", oninput: move |e| new_desc.set(e.value()) }
                        }
                        if !create_err().is_empty() {
                            p { style: "color:#dc2626;font-size:13px;margin-bottom:10px;", "{create_err}" }
                        }
                        div { style: "display:flex;gap:10px;justify-content:flex-end;",
                            button { class: "btn", onclick: move |_| show_create.set(false), "取消" }
                            button { class: "btn btn-primary", disabled: creating(), onclick: do_create,
                                if creating() { "提交中…" } else { "提交" }
                            }
                        }
                    }
                }
            }

            // List
            match &*crs_res.read() {
                None => rsx! { div { class: "text-muted", style: "padding:40px;text-align:center;", "加载中…" } },
                Some(Err(e)) => rsx! { div { style: "color:#dc2626;padding:40px;text-align:center;", "加载失败：{e}" } },
                Some(Ok(crs)) => {
                    if crs.is_empty() {
                        rsx! {
                            div { style: "text-align:center;padding:60px;color:var(--muted);",
                                div { style: "font-size:48px;margin-bottom:12px;", "🧾" }
                                h3 { "暂无变更请求" }
                                p { style: "font-size:13px;", "点击「提交变更请求」开始" }
                            }
                        }
                    } else {
                        rsx! {
                            div { style: "display:flex;flex-direction:column;gap:12px;",
                                for cr in crs.iter() {
                                    CrCard { cr: cr.clone() }
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
fn CrCard(cr: cr_api::ChangeRequest) -> Element {
    let title = cr.title.clone().unwrap_or_else(|| "无标题".to_string());
    let status = cr.status.clone().unwrap_or_else(|| "open".to_string());
    let doc = cr
        .document_title
        .clone()
        .unwrap_or_else(|| cr.document_id.clone().unwrap_or_default());
    let created = cr.created_at.clone().unwrap_or_default();

    let id_str = cr
        .id
        .as_ref()
        .and_then(|v| v.get("id"))
        .and_then(|v| v.as_str())
        .or_else(|| cr.id.as_ref().and_then(|v| v.as_str()))
        .unwrap_or("")
        .to_string();

    let mut acting = use_signal(|| false);

    let do_approve = {
        let id = id_str.clone();
        move |_| {
            if id.is_empty() {
                return;
            }
            acting.set(true);
            let id2 = id.clone();
            spawn(async move {
                let _ = cr_api::approve_cr(&id2).await;
                acting.set(false);
            });
        }
    };
    let do_reject = {
        let id = id_str.clone();
        move |_| {
            if id.is_empty() {
                return;
            }
            acting.set(true);
            let id2 = id.clone();
            spawn(async move {
                let _ = cr_api::reject_cr(&id2).await;
                acting.set(false);
            });
        }
    };

    let (status_cls, status_label) = match status.as_str() {
        "merged" => ("badge badge-success", "已合并"),
        "closed" => ("badge badge-gray", "已关闭"),
        _ => ("badge badge-primary", "待审核"),
    };

    rsx! {
        div { class: "card", style: "padding:16px;",
            div { style: "display:flex;align-items:flex-start;justify-content:space-between;gap:12px;",
                div { style: "flex:1;",
                    div { style: "display:flex;align-items:center;gap:8px;margin-bottom:6px;",
                        span { class: status_cls, "{status_label}" }
                        span { style: "font-size:14px;font-weight:600;", "{title}" }
                    }
                    if !doc.is_empty() {
                        p { style: "font-size:13px;color:var(--muted);margin-bottom:4px;", "文档：{doc}" }
                    }
                    p { style: "font-size:12px;color:var(--muted);", "{created}" }
                }
                if status == "open" {
                    div { style: "display:flex;gap:8px;",
                        button { class: "btn btn-sm btn-primary", disabled: acting(), onclick: do_approve,
                            if acting() { "…" } else { "批准" }
                        }
                        button { class: "btn btn-sm", style: "color:#dc2626;", disabled: acting(), onclick: do_reject,
                            if acting() { "…" } else { "拒绝" }
                        }
                    }
                }
            }
        }
    }
}
