use crate::api::language as lang_api;
use crate::api::spaces as spaces_api;
use dioxus::prelude::*;

#[component]
pub fn Language() -> Element {
    let spaces_res = use_resource(|| async move { spaces_api::list_spaces(1, 50).await });
    let mut selected_slug = use_signal(|| String::new());

    use_effect(move || {
        if selected_slug.read().is_empty() {
            if let Some(Ok(data)) = &*spaces_res.read() {
                if let Some(first) = data.spaces.as_ref().or(data.items.as_ref()).and_then(|s| s.first()) {
                    selected_slug.set(first.slug.clone());
                }
            }
        }
    });

    let langs_res = use_resource(move || {
        let slug = selected_slug.read().clone();
        async move {
            if slug.is_empty() {
                return Err("请选择空间".to_string());
            }
            lang_api::list_space_languages(&slug).await
        }
    });

    let mut show_add = use_signal(|| false);
    let mut new_code = use_signal(|| "en-US".to_string());
    let mut adding = use_signal(|| false);
    let mut add_err = use_signal(|| String::new());

    let do_add = move |_| {
        let slug = selected_slug.read().clone();
        if slug.is_empty() {
            return;
        }
        adding.set(true);
        add_err.set(String::new());
        let code = new_code.read().clone();
        spawn(async move {
            match lang_api::add_space_language(&slug, code).await {
                Ok(_) => {
                    show_add.set(false);
                }
                Err(e) => add_err.set(e),
            }
            adding.set(false);
        });
    };

    rsx! {
        document::Title { "语言版本 — SoulBook" }
        div { class: "page-content",
            div { class: "page-header",
                div { class: "page-header-left",
                    h1 { "🌍 语言版本" }
                    p { "管理同一文档的多语言独立版本" }
                }
                div { class: "page-header-actions",
                    if !selected_slug.read().is_empty() {
                        button { class: "btn btn-primary", onclick: move |_| show_add.set(true), "＋ 添加语言" }
                    }
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
                                        let slug = space.slug.clone();
                                        let name = space.name.clone();
                                        let is_sel = *selected_slug.read() == slug;
                                        rsx! {
                                            button {
                                                class: if is_sel { "btn btn-primary btn-sm" } else { "btn btn-sm" },
                                                onclick: move |_| selected_slug.set(slug.clone()),
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

            // Add language modal
            if show_add() {
                div { style: "position:fixed;inset:0;background:rgba(0,0,0,.4);z-index:200;display:flex;align-items:center;justify-content:center;",
                    onclick: move |_| show_add.set(false),
                    div { class: "card", style: "width:360px;padding:24px;", onclick: move |e| e.stop_propagation(),
                        h3 { style: "font-size:15px;font-weight:700;margin-bottom:16px;", "添加语言" }
                        div { class: "form-group",
                            label { class: "form-label", "语言" }
                            select { class: "input", value: "{new_code}", onchange: move |e| new_code.set(e.value()),
                                option { value: "en-US", "English (en-US)" }
                                option { value: "ja-JP", "日本語 (ja-JP)" }
                                option { value: "ko-KR", "한국어 (ko-KR)" }
                                option { value: "fr-FR", "Français (fr-FR)" }
                                option { value: "de-DE", "Deutsch (de-DE)" }
                                option { value: "es-ES", "Español (es-ES)" }
                                option { value: "pt-BR", "Português (pt-BR)" }
                            }
                        }
                        if !add_err().is_empty() {
                            p { style: "color:#dc2626;font-size:13px;margin-bottom:10px;", "{add_err}" }
                        }
                        div { style: "display:flex;gap:10px;justify-content:flex-end;",
                            button { class: "btn", onclick: move |_| show_add.set(false), "取消" }
                            button { class: "btn btn-primary", disabled: adding(), onclick: do_add,
                                if adding() { "添加中…" } else { "添加" }
                            }
                        }
                    }
                }
            }

            // Language list
            if !selected_slug.read().is_empty() {
                match &*langs_res.read() {
                    None => rsx! { div { class: "text-muted", style: "padding:40px;text-align:center;", "加载中…" } },
                    Some(Err(e)) => rsx! { div { style: "color:#dc2626;padding:40px;text-align:center;", "加载失败：{e}" } },
                    Some(Ok(langs)) => {
                        rsx! {
                            div { class: "grid-4", style: "margin-bottom:20px;",
                                for lang in langs.iter() {
                                    div { class: "card", style: "padding:20px;",
                                        div { style: "display:flex;align-items:center;justify-content:space-between;margin-bottom:8px;",
                                            span { style: "font-size:16px;font-weight:700;", "🌐" }
                                            if lang.is_default.unwrap_or(false) {
                                                span { class: "badge badge-primary", "默认" }
                                            }
                                        }
                                        p { style: "font-size:14px;font-weight:600;", "{lang.language_name.as_deref().unwrap_or(\"-\")}" }
                                        p { style: "font-size:12px;color:var(--muted);", "{lang.language_code.as_deref().unwrap_or(\"-\")}" }
                                        if lang.enabled.unwrap_or(true) {
                                            span { class: "badge badge-success", style: "margin-top:8px;display:inline-block;", "启用" }
                                        }
                                    }
                                }
                            }
                            if langs.is_empty() {
                                div { style: "text-align:center;padding:40px;color:var(--muted);",
                                    div { style: "font-size:36px;margin-bottom:10px;", "🌍" }
                                    p { "该空间还没有添加语言，点击「添加语言」开始" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
