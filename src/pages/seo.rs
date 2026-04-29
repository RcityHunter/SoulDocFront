use crate::api::publish as pub_api;
use crate::api::spaces as spaces_api;
use dioxus::prelude::*;
use serde_json::Value;

#[component]
pub fn Seo() -> Element {
    let mut active_tab = use_signal(|| "seo");
    let spaces_res = use_resource(|| async move { spaces_api::list_spaces(1, 50).await });
    let mut selected_space = use_signal(|| String::new());

    use_effect(move || {
        if selected_space.read().is_empty() {
            if let Some(Ok(data)) = &*spaces_res.read() {
                if let Some(first) = data.spaces.as_ref().or(data.items.as_ref()).and_then(|s| s.first()) {
                    selected_space.set(first.slug.clone());
                }
            }
        }
    });

    let seo_res = use_resource(move || {
        let slug = selected_space.read().clone();
        async move {
            if slug.is_empty() {
                return Ok(serde_json::json!({}));
            }
            pub_api::get_seo_metadata(&slug).await
        }
    });

    let targets_res = use_resource(|| async move { pub_api::list_publish_targets().await });
    let history_res = use_resource(|| async move { pub_api::list_release_history().await });

    let mut seo_title = use_signal(|| String::new());
    let mut seo_desc = use_signal(|| String::new());
    let mut seo_keywords = use_signal(|| String::new());
    let mut saving = use_signal(|| false);
    let mut save_msg = use_signal(|| String::new());
    let mut publish_msg = use_signal(|| String::new());

    use_effect(move || {
        if let Some(Ok(data)) = &*seo_res.read() {
            seo_title.set(
                data.get("seo_title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
            );
            seo_desc.set(
                data.get("seo_description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
            );
            seo_keywords.set(
                data.get("keywords")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
            );
        }
    });

    let do_save_seo = move |_| {
        let slug = selected_space.read().clone();
        if slug.is_empty() {
            save_msg.set("请先选择空间".to_string());
            return;
        }
        saving.set(true);
        save_msg.set(String::new());
        let body = serde_json::json!({
            "seo_title": seo_title.read().clone(),
            "seo_description": seo_desc.read().clone(),
            "keywords": seo_keywords.read().clone(),
            "url_slug": slug.clone(),
        });
        spawn(async move {
            match pub_api::update_seo_metadata(&slug, &body).await {
                Ok(_) => save_msg.set("✅ SEO 设置已保存".to_string()),
                Err(e) => save_msg.set(format!("保存失败：{}", e)),
            }
            saving.set(false);
        });
    };

    rsx! {
        document::Title { "SEO 与发布 — SoulBook" }
        div { class: "page-content",
            div { class: "page-header",
                div { class: "page-header-left",
                    h1 { "🌐 发布 & SEO" }
                    p { "配置 SEO 元数据、管理发布目标和公开站点结构" }
                }
                div { class: "page-header-actions",
                    button { class: "btn btn-sm", "预览公开页" }
                    button { class: "btn btn-primary",
                        onclick: move |_| {
                            publish_msg.set("发布中…".to_string());
                            spawn(async move {
                                match pub_api::trigger_publish("prod", "v0.0.1").await {
                                    Ok(_) => publish_msg.set("✅ 发布成功".to_string()),
                                    Err(e) => publish_msg.set(format!("❌ 发布失败：{}", e)),
                                }
                            });
                        },
                        "🚀 发布"
                    }
                }
            }

            if !publish_msg().is_empty() {
                div { style: "padding:10px 14px;background:var(--panel2);border:1px solid var(--line);border-radius:8px;font-size:13px;margin-bottom:16px;",
                    "{publish_msg}"
                }
            }

            // Space selector
            div { style: "margin-bottom:16px;",
                match &*spaces_res.read() {
                    Some(Ok(data)) => {
                        let spaces = data.spaces.as_ref().or(data.items.as_ref()).cloned().unwrap_or_default();
                        rsx! {
                            select {
                                class: "input",
                                style: "max-width:280px;font-size:13px;",
                                onchange: move |e| {
                                    selected_space.set(e.value());
                                    seo_title.set(String::new());
                                    seo_desc.set(String::new());
                                    seo_keywords.set(String::new());
                                    save_msg.set(String::new());
                                },
                                option { value: "", "— 选择空间 —" }
                                for s in spaces.iter() {
                                    option { value: "{s.slug}", "{s.name}" }
                                }
                            }
                        }
                    },
                    _ => rsx! { p { style: "font-size:13px;color:var(--muted);", "加载空间中…" } }
                }
            }

            div { class: "tabs",
                TabBtn { id: "seo", label: "SEO 设置", active: active_tab(), onclick: move |_| active_tab.set("seo") }
                TabBtn { id: "publish", label: "发布目标", active: active_tab(), onclick: move |_| active_tab.set("publish") }
                TabBtn { id: "history", label: "发布历史", active: active_tab(), onclick: move |_| active_tab.set("history") }
            }

            if active_tab() == "seo" {
                div { class: "grid-2", style: "gap:20px;",
                    div { class: "card",
                        div { class: "card-header", h3 { "元数据配置" } }
                        div { class: "form-group",
                            label { class: "form-label", "SEO 标题" }
                            input { class: "input", value: "{seo_title}", oninput: move |e| seo_title.set(e.value()) }
                            p { class: "form-hint", "建议 50-60 字符" }
                        }
                        div { class: "form-group",
                            label { class: "form-label", "SEO 描述" }
                            textarea { class: "input textarea", value: "{seo_desc}", oninput: move |e| seo_desc.set(e.value()) }
                            p { class: "form-hint", "建议 120-160 字符" }
                        }
                        div { class: "form-group",
                            label { class: "form-label", "关键词" }
                            input { class: "input", value: "{seo_keywords}", oninput: move |e| seo_keywords.set(e.value()) }
                        }
                        if !save_msg().is_empty() {
                            p { style: "font-size:13px;margin-bottom:8px;color:var(--muted);", "{save_msg}" }
                        }
                        button { class: "btn btn-primary", disabled: saving(), onclick: do_save_seo,
                            if saving() { "保存中…" } else { "保存 SEO 设置" }
                        }
                    }

                    div { style: "display:flex;flex-direction:column;gap:16px;",
                        div { class: "card",
                            div { class: "card-header", h3 { "Google 预览" } }
                            div { class: "seo-preview",
                                p { class: "seo-title", if seo_title().is_empty() { "（标题）" } else { "{seo_title}" } }
                                p { class: "seo-url", "docs.soulbook.io › {selected_space}" }
                                p { class: "seo-desc", if seo_desc().is_empty() { "（描述）" } else { "{seo_desc}" } }
                            }
                        }
                        div { class: "card",
                            div { class: "card-header",
                                h3 { "SEO 评分" }
                                button {
                                    class: "btn btn-sm",
                                    onclick: move |_| {
                                        let slug = selected_space.read().clone();
                                        if slug.is_empty() { return; }
                                        spawn(async move {
                                            let _ = pub_api::ai_analyze_seo(&slug).await;
                                            save_msg.set("✅ AI SEO 分析完成".to_string());
                                        });
                                    },
                                    "AI 检查"
                                }
                            }
                            {
                                let title_len = seo_title().len();
                                let desc_len = seo_desc().len();
                                let title_score = if title_len >= 30 && title_len <= 60 { "✅ 优秀" } else if title_len == 0 { "⚠️ 未填写" } else { "⚠️ 待优化" };
                                let desc_score = if desc_len >= 80 && desc_len <= 160 { "✅ 优秀" } else if desc_len == 0 { "⚠️ 未填写" } else { "⚠️ 待优化" };
                                let kw_score = if seo_keywords().is_empty() { "⚠️ 待填写" } else { "✅ 已配置" };
                                rsx! {
                                    div { style: "display:flex;flex-direction:column;gap:10px;",
                                        SeoScoreItem { label: "标题长度", score: "{title_score}" }
                                        SeoScoreItem { label: "描述长度", score: "{desc_score}" }
                                        SeoScoreItem { label: "关键词", score: "{kw_score}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if active_tab() == "publish" {
                match &*targets_res.read() {
                    None => rsx! { p { style: "color:var(--muted);", "加载中…" } },
                    Some(Err(e)) => rsx! { p { style: "color:#dc2626;", "加载失败：{e}" } },
                    Some(Ok(targets)) => rsx! {
                        div { style: "display:flex;flex-direction:column;gap:14px;",
                            for target in targets.iter() {
                                PublishTargetCard { target: target.clone(), on_publish: move |id: String| {
                                    publish_msg.set("发布中…".to_string());
                                    spawn(async move {
                                        match pub_api::trigger_publish(&id, "latest").await {
                                            Ok(_) => publish_msg.set("✅ 发布成功".to_string()),
                                            Err(e) => publish_msg.set(format!("❌ 发布失败：{}", e)),
                                        }
                                    });
                                }}
                            }
                        }
                    },
                }
            }

            if active_tab() == "history" {
                match &*history_res.read() {
                    None => rsx! { p { style: "color:var(--muted);", "加载中…" } },
                    Some(Err(e)) => rsx! { p { style: "color:#dc2626;", "加载失败：{e}" } },
                    Some(Ok(items)) => rsx! {
                        div { class: "card",
                            table {
                                thead {
                                    tr {
                                        th { "版本" }
                                        th { "目标" }
                                        th { "状态" }
                                        th { "操作者" }
                                        th { "发布时间" }
                                    }
                                }
                                tbody {
                                    for item in items.iter() {
                                        tr {
                                            td { span { class: "badge badge-primary", "{item.get(\"version\").and_then(|v| v.as_str()).unwrap_or(\"-\")}" } }
                                            td { "{item.get(\"target\").and_then(|v| v.as_str()).unwrap_or(\"-\")}" }
                                            td { span { class: "badge badge-success", "{item.get(\"status\").and_then(|v| v.as_str()).unwrap_or(\"-\")}" } }
                                            td { "{item.get(\"triggered_by\").and_then(|v| v.as_str()).unwrap_or(\"-\")}" }
                                            td { class: "text-muted", "{item.get(\"published_at\").and_then(|v| v.as_str()).unwrap_or(\"-\")}" }
                                        }
                                    }
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn TabBtn(
    id: &'static str,
    label: &'static str,
    active: &'static str,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: if active == id { "tab active" } else { "tab" }, onclick: move |e| onclick.call(e), "{label}" }
    }
}

#[component]
fn SeoScoreItem(label: &'static str, score: String) -> Element {
    rsx! {
        div { style: "display:flex;align-items:center;justify-content:space-between;font-size:13.5px;",
            span { style: "color:var(--text3);", "{label}" }
            span { "{score}" }
        }
    }
}

#[component]
fn PublishTargetCard(target: Value, on_publish: EventHandler<String>) -> Element {
    let id = target
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim_start_matches("publish_target:")
        .to_string();
    let name = target
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("-")
        .to_string();
    let channel = target
        .get("channel")
        .and_then(|v| v.as_str())
        .unwrap_or("-")
        .to_string();
    let domain = target
        .get("domain")
        .and_then(|v| v.as_str())
        .unwrap_or("-")
        .to_string();
    let last_release = target
        .get("last_release")
        .and_then(|v| v.as_str())
        .unwrap_or("-")
        .to_string();
    rsx! {
        div { class: "card",
            div { style: "display:flex;align-items:center;justify-content:space-between;",
                div { style: "display:flex;align-items:center;gap:12px;",
                    div { style: "width:40px;height:40px;border-radius:10px;background:var(--primary-light);display:flex;align-items:center;justify-content:center;font-size:18px;",
                        "🌐"
                    }
                    div {
                        p { style: "font-weight:600;", "{name}" }
                        p { style: "font-size:12px;color:var(--muted);", "{channel} · {domain}" }
                    }
                }
                div { style: "display:flex;align-items:center;gap:8px;",
                    span { class: "badge badge-success", style: "font-size:12px;", "{last_release}" }
                    button {
                        class: "btn btn-sm btn-primary",
                        onclick: move |_| on_publish.call(id.clone()),
                        "发布"
                    }
                }
            }
        }
    }
}
