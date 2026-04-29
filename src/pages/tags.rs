use crate::api::tags as tags_api;
use crate::models::Tag;
use dioxus::prelude::*;

// ── Component ───────────────────────────────────────────────────────────────

#[component]
pub fn Tags() -> Element {
    let tags_res = use_resource(|| async move { tags_api::list_tags(None, 1, 100).await });
    let mut refresh = use_signal(|| 0u32);
    let mut show_create = use_signal(|| false);
    let mut new_name = use_signal(|| String::new());
    let mut new_color = use_signal(|| "#6366f1".to_string());
    let mut new_desc = use_signal(|| String::new());
    let mut create_err = use_signal(|| String::new());
    let mut creating = use_signal(|| false);
    let mut search_query = use_signal(|| String::new());
    let mut sort_by = use_signal(|| "usage".to_string());
    let _ = refresh; // suppress unused warning

    let do_create = move |_| {
        let name = new_name.read().trim().to_string();
        let color = new_color.read().clone();
        let desc = new_desc.read().trim().to_string();
        if name.is_empty() {
            return;
        }
        creating.set(true);
        create_err.set(String::new());
        spawn(async move {
            match tags_api::create_tag(tags_api::CreateTagRequest {
                name,
                color: Some(color),
                description: Some(desc),
                space_id: None,
            })
            .await
            {
                Ok(_) => {
                    show_create.set(false);
                    new_name.set(String::new());
                    new_desc.set(String::new());
                    let cur = *refresh.read();
                    refresh.set(cur + 1);
                }
                Err(e) => create_err.set(e),
            }
            creating.set(false);
        });
    };

    rsx! {
        document::Title { "标签管理 — SoulBook" }
        div { class: "page-content",

            // ── Top header bar ──────────────────────────────────────────
            div { style: "display:flex;align-items:center;justify-content:space-between;margin-bottom:24px;",
                h1 { style: "font-size:20px;font-weight:700;display:flex;align-items:center;gap:8px;",
                    span { style: "font-size:18px;", "⊕" }
                    span { "标签管理" }
                }
                div { style: "display:flex;align-items:center;gap:10px;",
                    button {
                        style: "padding:7px 14px;border-radius:7px;border:none;background:#3b82f6;color:#fff;font-size:13px;font-weight:500;cursor:pointer;",
                        onclick: move |_| show_create.set(true),
                        "+ 新建标签"
                    }
                }
            }

            // ── Create modal ────────────────────────────────────────────
            if show_create() {
                div {
                    style: "position:fixed;inset:0;background:rgba(0,0,0,.45);z-index:200;display:flex;align-items:center;justify-content:center;",
                    onclick: move |_| show_create.set(false),
                    div {
                        style: "width:440px;background:var(--panel2);border-radius:12px;padding:28px;box-shadow:0 20px 60px rgba(0,0,0,.25);",
                        onclick: move |e| e.stop_propagation(),
                        h3 { style: "font-size:16px;font-weight:700;margin-bottom:20px;", "新建标签" }
                        div { style: "margin-bottom:14px;",
                            label { style: "display:block;font-size:13px;font-weight:500;margin-bottom:6px;", "标签名称" }
                            input {
                                style: "width:100%;padding:9px 12px;border-radius:7px;border:1px solid var(--line);background:var(--panel);font-size:13px;box-sizing:border-box;",
                                placeholder: "输入标签名称",
                                value: "{new_name}",
                                oninput: move |e| new_name.set(e.value())
                            }
                        }
                        div { style: "margin-bottom:14px;",
                            label { style: "display:block;font-size:13px;font-weight:500;margin-bottom:6px;", "颜色" }
                            div { style: "display:flex;align-items:center;gap:10px;",
                                input {
                                    r#type: "color",
                                    value: "{new_color}",
                                    style: "width:44px;height:36px;border:none;cursor:pointer;border-radius:6px;padding:2px;",
                                    oninput: move |e| new_color.set(e.value())
                                }
                                input {
                                    style: "flex:1;padding:9px 12px;border-radius:7px;border:1px solid var(--line);background:var(--panel);font-size:13px;",
                                    value: "{new_color}",
                                    oninput: move |e| new_color.set(e.value())
                                }
                                {
                                    let color = new_color.read().clone();
                                    let name_preview = new_name.read().clone();
                                    rsx! {
                                        span {
                                            style: "padding:4px 12px;border-radius:20px;font-size:13px;background:{color}22;color:{color};border:1px solid {color}55;white-space:nowrap;",
                                            if name_preview.is_empty() { "预览" } else { "{name_preview}" }
                                        }
                                    }
                                }
                            }
                        }
                        div { style: "margin-bottom:14px;",
                            label { style: "display:block;font-size:13px;font-weight:500;margin-bottom:6px;", "描述（可选）" }
                            input {
                                style: "width:100%;padding:9px 12px;border-radius:7px;border:1px solid var(--line);background:var(--panel);font-size:13px;box-sizing:border-box;",
                                placeholder: "标签用途说明",
                                value: "{new_desc}",
                                oninput: move |e| new_desc.set(e.value())
                            }
                        }
                        if !create_err().is_empty() {
                            p { style: "color:#dc2626;font-size:13px;margin-bottom:10px;", "{create_err}" }
                        }
                        div { style: "display:flex;gap:10px;justify-content:flex-end;margin-top:8px;",
                            button {
                                style: "padding:8px 16px;border-radius:7px;border:1px solid var(--line);background:var(--panel2);font-size:13px;cursor:pointer;",
                                onclick: move |_| show_create.set(false),
                                "取消"
                            }
                            button {
                                style: "padding:8px 16px;border-radius:7px;border:none;background:#3b82f6;color:#fff;font-size:13px;font-weight:500;cursor:pointer;",
                                disabled: creating(),
                                onclick: do_create,
                                if creating() { "创建中…" } else { "创建标签" }
                            }
                        }
                    }
                }
            }

            // ── Body: render tags from API ──────────────────────────────
            {
                let tags_data = match &*tags_res.read() {
                    Some(Ok(data)) => data.tags.as_ref().cloned().unwrap_or_default(),
                    _ => vec![],
                };
                let total_tags = tags_data.len();
                let total_docs: u32 = tags_data.iter().map(|t| t.doc_count.unwrap_or(0)).sum();

                rsx! {
                    // ── Loading / error states ──────────────────────────
                    match &*tags_res.read() {
                        None => rsx! {
                            div { style: "padding:60px;text-align:center;color:var(--muted);font-size:14px;", "加载中…" }
                        },
                        Some(Err(e)) => rsx! {
                            div { style: "padding:60px;text-align:center;color:#dc2626;font-size:14px;", "加载失败：{e}" }
                        },
                        Some(Ok(_)) => rsx! {
                            // ── Two-column layout ───────────────────────────────────────
                            div { style: "display:grid;grid-template-columns:1fr 284px;gap:20px;align-items:start;",

                                // ── LEFT COLUMN ─────────────────────────────────────────
                                div {

                                    // ── Tag cloud card ──────────────────────────────────
                                    div { style: "background:var(--panel2);border:1px solid var(--line);border-radius:12px;margin-bottom:20px;overflow:hidden;",
                                        div { style: "padding:16px 20px;border-bottom:1px solid var(--line);display:flex;align-items:center;justify-content:space-between;",
                                            div { style: "display:flex;align-items:center;gap:8px;",
                                                span { style: "font-size:15px;", "☁" }
                                                span { style: "font-size:14px;font-weight:600;", "标签云" }
                                                span { style: "font-size:12.5px;color:var(--muted);", "共 {total_tags} 个标签" }
                                            }
                                            select {
                                                style: "padding:5px 10px;border-radius:6px;border:1px solid var(--line);background:var(--panel);font-size:12.5px;",
                                                value: "{sort_by}",
                                                onchange: move |e| sort_by.set(e.value()),
                                                option { value: "usage", "按使用频率 ▼" }
                                                option { value: "name", "按名称 A-Z" }
                                                option { value: "created", "按创建时间" }
                                            }
                                        }
                                        div { style: "padding:20px;display:flex;flex-wrap:wrap;gap:10px;min-height:80px;",
                                            if tags_data.is_empty() {
                                                span { style: "color:var(--muted);font-size:13px;", "暂无标签，点击右上角「+ 新建标签」开始" }
                                            } else {
                                                for tag in tags_data.iter() {
                                                    {
                                                        let color = tag.color.clone().unwrap_or_else(|| "#6366f1".into());
                                                        let name = tag.name.clone();
                                                        let cnt = tag.doc_count.unwrap_or(0);
                                                        let font_size = if cnt > 30 { "15px" } else if cnt > 10 { "13.5px" } else { "12.5px" };
                                                        rsx! {
                                                            span {
                                                                style: "display:inline-flex;align-items:center;gap:5px;padding:5px 14px;border-radius:20px;font-size:{font_size};font-weight:500;background:{color}22;color:{color};border:1px solid {color}44;cursor:pointer;",
                                                                "{name}"
                                                                span { style: "font-size:11px;opacity:.7;", "+{cnt}" }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // ── All tags card ───────────────────────────────────
                                    div { style: "background:var(--panel2);border:1px solid var(--line);border-radius:12px;overflow:hidden;",
                                        // Card header
                                        div { style: "padding:16px 20px;border-bottom:1px solid var(--line);display:flex;align-items:center;justify-content:space-between;",
                                            span { style: "font-size:14px;font-weight:600;", "所有标签" }
                                            input {
                                                style: "padding:6px 12px;border-radius:7px;border:1px solid var(--line);background:var(--panel);font-size:13px;width:180px;",
                                                placeholder: "搜索标签...",
                                                value: "{search_query}",
                                                oninput: move |e| search_query.set(e.value())
                                            }
                                        }
                                        // Filter row
                                        div { style: "padding:10px 20px;border-bottom:1px solid var(--line);display:flex;align-items:center;gap:10px;",
                                            select {
                                                style: "padding:6px 10px;border-radius:6px;border:1px solid var(--line);background:var(--panel);font-size:12.5px;",
                                                option { "全部空间" }
                                                option { "默认空间" }
                                            }
                                            select {
                                                style: "padding:6px 10px;border-radius:6px;border:1px solid var(--line);background:var(--panel);font-size:12.5px;",
                                                option { "所有颜色" }
                                                option { "蓝色" }
                                                option { "绿色" }
                                                option { "红色" }
                                            }
                                            div { style: "flex:1;" }
                                            span { style: "font-size:12.5px;color:var(--muted);", "共 {total_tags} 个标签" }
                                        }
                                        // Table header
                                        div { style: "display:grid;grid-template-columns:1fr 90px 90px 100px 80px 52px;padding:10px 20px;background:var(--panel);border-bottom:1px solid var(--line);",
                                            span { style: "font-size:11.5px;font-weight:600;color:var(--muted);text-transform:uppercase;letter-spacing:.04em;", "标签名称" }
                                            span { style: "font-size:11.5px;font-weight:600;color:var(--muted);text-transform:uppercase;letter-spacing:.04em;text-align:center;", "使用次数" }
                                            span { style: "font-size:11.5px;font-weight:600;color:var(--muted);text-transform:uppercase;letter-spacing:.04em;text-align:center;", "关联空间" }
                                            span { style: "font-size:11.5px;font-weight:600;color:var(--muted);text-transform:uppercase;letter-spacing:.04em;", "创建者" }
                                            span { style: "font-size:11.5px;font-weight:600;color:var(--muted);text-transform:uppercase;letter-spacing:.04em;", "类型" }
                                            span { style: "font-size:11.5px;font-weight:600;color:var(--muted);text-transform:uppercase;letter-spacing:.04em;", "" }
                                        }
                                        // Tag rows
                                        if tags_data.is_empty() {
                                            div { style: "padding:40px;text-align:center;color:var(--muted);font-size:13px;",
                                                "暂无标签"
                                            }
                                        } else {
                                            div {
                                                for tag in tags_data.iter() {
                                                    TagRow { tag: tag.clone() }
                                                }
                                            }
                                        }
                                        // Load more
                                        div { style: "padding:16px 20px;border-top:1px solid var(--line);text-align:center;",
                                            button {
                                                style: "padding:8px 20px;border-radius:8px;border:1px solid var(--line);background:var(--panel2);font-size:13px;cursor:pointer;color:var(--text);",
                                                "加载更多 ({total_tags} 个标签)"
                                            }
                                        }
                                    }
                                }

                                // ── RIGHT PANEL ─────────────────────────────────────────
                                div { style: "display:flex;flex-direction:column;gap:16px;",

                                    // Stats card
                                    div { style: "background:var(--panel2);border:1px solid var(--line);border-radius:12px;overflow:hidden;",
                                        div { style: "padding:14px 18px;border-bottom:1px solid var(--line);",
                                            span { style: "font-size:13.5px;font-weight:600;", "标签统计" }
                                        }
                                        div { style: "padding:16px 18px;display:grid;grid-template-columns:1fr 1fr;gap:14px;",
                                            div { style: "text-align:center;",
                                                p { style: "font-size:24px;font-weight:700;color:var(--text);line-height:1.1;", "{total_tags}" }
                                                p { style: "font-size:12px;color:var(--muted);margin-top:3px;", "总标签数" }
                                            }
                                            div { style: "text-align:center;",
                                                p { style: "font-size:24px;font-weight:700;color:var(--text);line-height:1.1;", "{total_docs}" }
                                                p { style: "font-size:12px;color:var(--muted);margin-top:3px;", "总标签已次" }
                                            }
                                            div { style: "text-align:center;",
                                                p { style: "font-size:24px;font-weight:700;color:var(--text);line-height:1.1;",
                                                    {
                                                        let avg = if total_tags > 0 { format!("{:.1}", total_docs as f64 / total_tags as f64) } else { "0.0".to_string() };
                                                        rsx! { "{avg}" }
                                                    }
                                                }
                                                p { style: "font-size:12px;color:var(--muted);margin-top:3px;", "平均每签" }
                                            }
                                            div { style: "text-align:center;",
                                                p { style: "font-size:24px;font-weight:700;color:var(--text);line-height:1.1;", "—" }
                                                p { style: "font-size:12px;color:var(--muted);margin-top:3px;", "本周新增" }
                                            }
                                        }
                                    }

                                    // Hottest tags card
                                    div { style: "background:var(--panel2);border:1px solid var(--line);border-radius:12px;overflow:hidden;",
                                        div { style: "padding:14px 18px;border-bottom:1px solid var(--line);",
                                            span { style: "font-size:13.5px;font-weight:600;", "最热门标签" }
                                        }
                                        div { style: "padding:14px 18px;display:flex;flex-direction:column;gap:12px;",
                                            if tags_data.is_empty() {
                                                p { style: "font-size:12.5px;color:var(--muted);text-align:center;padding:8px 0;",
                                                    "暂无标签数据"
                                                }
                                            } else {
                                                {
                                                    let mut sorted = tags_data.clone();
                                                    sorted.sort_by(|a, b| b.doc_count.unwrap_or(0).cmp(&a.doc_count.unwrap_or(0)));
                                                    let max_cnt = sorted.first().and_then(|t| t.doc_count).unwrap_or(1).max(1);
                                                    rsx! {
                                                        for (idx, tag) in sorted.iter().take(5).enumerate() {
                                                            {
                                                                let color = tag.color.clone().unwrap_or_else(|| "#6366f1".into());
                                                                let name = tag.name.clone();
                                                                let cnt = tag.doc_count.unwrap_or(0);
                                                                let _max = max_cnt;
                                                                let rank = idx + 1;
                                                                let bar_pct = cnt * 100 / _max;
                                                                rsx! {
                                                                    div { style: "display:flex;align-items:center;gap:8px;",
                                                                        span { style: "width:20px;font-size:12px;font-weight:700;color:{color};text-align:center;flex-shrink:0;", "{rank}" }
                                                                        div { style: "flex:1;min-width:0;",
                                                                            div { style: "display:flex;align-items:center;justify-content:space-between;margin-bottom:4px;",
                                                                                span { style: "font-size:13px;font-weight:500;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;", "{name}" }
                                                                                span { style: "font-size:12px;color:var(--muted);flex-shrink:0;margin-left:6px;", "{cnt}篇" }
                                                                            }
                                                                            div { style: "height:4px;background:var(--line);border-radius:2px;overflow:hidden;",
                                                                                div { style: "height:100%;background:{color};border-radius:2px;width:{bar_pct}%;" }
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

                                    // Orphan tags card
                                    div { style: "background:var(--panel2);border:1px solid var(--line);border-radius:12px;overflow:hidden;",
                                        div { style: "padding:14px 18px;border-bottom:1px solid var(--line);display:flex;align-items:center;gap:7px;",
                                            span { style: "font-size:14px;", "🗑" }
                                            span { style: "font-size:13.5px;font-weight:600;", "无效标签" }
                                        }
                                        div { style: "padding:20px 18px;text-align:center;color:var(--muted);",
                                            p { style: "font-size:13px;", "暂无未使用的标签" }
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

// ── Real API tag row component ─────────────────────────────────────────────

#[component]
fn TagRow(tag: Tag) -> Element {
    let tag_id = tag.id.clone().unwrap_or_default();
    let color = tag.color.clone().unwrap_or_else(|| "#6366f1".into());
    let name = tag.name.clone();
    let doc_count = tag.doc_count.unwrap_or(0);
    let mut deleting = use_signal(|| false);

    let do_delete = move |_| {
        let id = tag_id.clone();
        if id.is_empty() {
            return;
        }
        deleting.set(true);
        spawn(async move {
            let _ = tags_api::delete_tag(&id).await;
            deleting.set(false);
        });
    };

    rsx! {
        div { style: "display:grid;grid-template-columns:1fr 90px 90px 100px 80px 52px;padding:12px 20px;border-bottom:1px solid var(--line);align-items:center;",
            div { style: "display:flex;align-items:center;gap:8px;",
                span { style: "width:10px;height:10px;border-radius:50%;background:{color};display:inline-block;flex-shrink:0;" }
                span {
                    style: "padding:3px 10px;border-radius:20px;font-size:12.5px;font-weight:500;background:{color}22;color:{color};border:1px solid {color}44;",
                    "{name}"
                }
            }
            span { style: "font-size:13px;text-align:center;color:var(--text);font-weight:500;", "{doc_count}篇" }
            span { style: "font-size:13px;text-align:center;color:var(--muted);", "–" }
            span { style: "font-size:13px;color:var(--muted);", "–" }
            span { style: "padding:2px 8px;border-radius:10px;font-size:11.5px;background:#e0e7ff;color:#4338ca;", "人工" }
            button {
                style: "padding:4px 8px;border-radius:6px;border:1px solid #fecaca;background:#fff1f2;color:#dc2626;font-size:12px;cursor:pointer;",
                disabled: deleting(),
                onclick: do_delete,
                if deleting() { "…" } else { "删除" }
            }
        }
    }
}
