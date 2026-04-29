use crate::components::{sidebar::Sidebar, topbar::Topbar};
use crate::routes::Route;
use crate::state::AuthState;
use dioxus::prelude::*;

#[component]
pub fn AppLayout() -> Element {
    let auth = use_context::<Signal<AuthState>>();
    let navigator = use_navigator();

    use_effect(move || {
        if !auth.read().is_authenticated() {
            navigator.replace(Route::Login {});
        }
    });

    if !auth.read().is_authenticated() {
        return rsx! {
            div {
                style: "min-height:100vh;display:flex;align-items:center;justify-content:center;background:var(--bg);",
                div { style: "text-align:center;color:var(--muted);",
                    p { style: "font-size:16px;", "正在跳转登录页…" }
                }
            }
        };
    }

    rsx! {
        div { class: "app-layout",
            Sidebar {}
            div { class: "main-area",
                Topbar {}
                Outlet::<Route> {}
            }
        }
    }
}
