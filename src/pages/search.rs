use crate::api::search as search_api;
use crate::models::SearchResult;
use dioxus::prelude::*;

#[component]
pub fn Search() -> Element {
    let mut query = use_signal(|| String::new());
    let mut results = use_signal(|| Vec::<SearchResult>::new());
    let mut loading = use_signal(|| false);
    let mut total = use_signal(|| 0u64);
    let mut err_msg = use_signal(|| String::new());

    let run_search = use_callback(move |_: ()| {
        let q = query.read().trim().to_string();
        if q.is_empty() {
            results.set(vec![]);
            return;
        }
        loading.set(true);
        err_msg.set(String::new());
        spawn(async move {
            match search_api::search(&q, 1, 20).await {
                Ok(data) => {
                    total.set(data.total.unwrap_or(0));
                    results.set(data.results.unwrap_or_default());
                }
                Err(e) => err_msg.set(e),
            }
            loading.set(false);
        });
    });

    rsx! {
        document::Title { "全局搜索 — SoulBook" }
        div { class: "page-content",
            div { class: "page-header",
                div { class: "page-header-left",
                    h1 { "🔍 全局搜索" }
                    p { "搜索所有空间的文档、标签和内容" }
                }
            }

            div { class: "card", style: "margin-bottom:20px;",
                div { style: "display:flex;gap:10px;",
                    div { style: "flex:1;position:relative;",
                        span { style: "position:absolute;left:12px;top:50%;transform:translateY(-50%);font-size:16px;", "🔍" }
                        input {
                            class: "input",
                            style: "padding-left:38px;",
                            placeholder: "搜索文档、标签、内容…",
                            value: "{query}",
                            oninput: move |e| query.set(e.value()),
                            onkeydown: move |e: KeyboardEvent| {
                                if e.key() == Key::Enter { run_search(()) }
                            },
                        }
                    }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| run_search(()),
                        if loading() { "搜索中…" } else { "搜索" }
                    }
                }
            }

            if !err_msg().is_empty() {
                div { style: "color:#dc2626;margin-bottom:16px;padding:12px;background:#fef2f2;border-radius:8px;font-size:13px;", "{err_msg}" }
            }

            if !results.read().is_empty() {
                div { class: "card",
                    div { class: "card-header",
                        h3 { "找到 {total} 条结果" }
                    }
                    div {
                        for r in results.read().iter() {
                            SearchRow { result: r.clone() }
                        }
                    }
                }
            } else if !loading() && !query.read().is_empty() && err_msg().is_empty() {
                div { style: "text-align:center;padding:60px;color:var(--muted);",
                    div { style: "font-size:48px;margin-bottom:12px;", "🔍" }
                    h3 { "没有找到结果" }
                    p { style: "font-size:13px;", "尝试换个关键词，或检查拼写" }
                }
            } else if query.read().is_empty() {
                div { style: "text-align:center;padding:60px;color:var(--muted);",
                    div { style: "font-size:48px;margin-bottom:12px;", "🔍" }
                    p { "输入关键词后按回车或点击搜索" }
                }
            }
        }
    }
}

#[component]
fn SearchRow(result: SearchResult) -> Element {
    let title = result.title.clone();
    let snippet = result.snippet.clone().unwrap_or_default();
    let space = result.space_name.clone().unwrap_or_default();
    let tags = result.tags.clone().unwrap_or_default();
    rsx! {
        div { style: "padding:16px;border-bottom:1px solid var(--line);",
            div { style: "display:flex;align-items:flex-start;gap:12px;",
                div { style: "width:36px;height:36px;border-radius:8px;background:#eef2ff;display:flex;align-items:center;justify-content:center;font-size:16px;flex-shrink:0;", "📄" }
                div { style: "flex:1;min-width:0;",
                    div { style: "display:flex;align-items:center;gap:8px;margin-bottom:4px;",
                        span { style: "font-size:14px;font-weight:600;", "{title}" }
                        if !space.is_empty() {
                            span { style: "font-size:12px;color:var(--muted);", "· {space}" }
                        }
                    }
                    if !snippet.is_empty() {
                        p { style: "font-size:13px;color:var(--text3);line-height:1.6;margin-bottom:6px;overflow:hidden;display:-webkit-box;-webkit-line-clamp:2;-webkit-box-orient:vertical;", "{snippet}" }
                    }
                    div { style: "display:flex;gap:6px;flex-wrap:wrap;",
                        for tag in tags.iter() {
                            span { class: "badge badge-gray", style: "font-size:11px;", "{tag}" }
                        }
                    }
                }
            }
        }
    }
}
