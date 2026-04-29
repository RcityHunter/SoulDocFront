use crate::routes::Route;
use crate::state::{AuthState, CreateDocTrigger};
use dioxus::prelude::*;

#[component]
pub fn Topbar() -> Element {
    let auth = use_context::<Signal<AuthState>>();
    let mut create_trigger = use_context::<Signal<CreateDocTrigger>>();
    let navigator = use_navigator();
    let initial = {
        let state = auth.read();
        state
            .user
            .as_ref()
            .and_then(|u| u.username.as_deref().or(Some(u.email.as_str())))
            .and_then(|s| s.chars().next())
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_else(|| "U".to_string())
    };

    rsx! {
        div { class: "topbar",
            div { class: "topbar-search",
                span { class: "topbar-search-icon", "🔍" }
                input {
                    r#type: "text",
                    placeholder: "搜索文档、空间、标签… (⌘K)",
                    style: "width:100%;padding:9px 14px 9px 38px;border:1px solid var(--line);border-radius:var(--radius);background:var(--panel2);outline:none;",
                    onfocus: move |e| {
                        e.stop_propagation();
                    }
                }
            }
            div { class: "topbar-right",
                span { class: "ai-badge", "AI 已就绪" }
                Link { to: Route::GitSync {}, class: "btn btn-sm", "连接 GitHub" }
                Link { to: Route::Members {}, class: "btn btn-sm", "邀请成员" }
                button {
                    class: "btn btn-sm btn-primary",
                    onclick: move |_| {
                        create_trigger.write().0 = true;
                        navigator.push(Route::Docs {});
                    },
                    "＋ 新建文档"
                }
                Link { to: Route::Profile {},
                    div { class: "avatar",
                        style: "cursor:pointer;background:var(--gradient);",
                        "{initial}"
                    }
                }
            }
        }
    }
}
