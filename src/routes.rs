use crate::components::layout::AppLayout;
use crate::pages::*;
use dioxus::prelude::*;

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    // Auth — no layout wrapper
    #[route("/login")]
    Login {},
    #[route("/install")]
    Install {},

    // App shell — sidebar + topbar
    #[layout(AppLayout)]
        #[route("/")]
        Dashboard {},
        #[route("/spaces")]
        Spaces {},
        #[route("/space")]
        SpaceOverview {},
        #[route("/docs")]
        Docs {},
        #[route("/editor")]
        Editor {},
        #[route("/versions")]
        Versions {},
        #[route("/change-requests")]
        ChangeRequests {},
        #[route("/members")]
        Members {},
        #[route("/tags")]
        Tags {},
        #[route("/files")]
        Files {},
        #[route("/search")]
        Search {},
        #[route("/language")]
        Language {},
        #[route("/ai-agent")]
        AiAgent {},
        #[route("/ai-tasks")]
        AiTasks {},
        #[route("/ai-tools")]
        AiTools {},
        #[route("/seo")]
        Seo {},
        #[route("/git-sync")]
        GitSync {},
        #[route("/developer")]
        Developer {},
        #[route("/notifications")]
        Notifications {},
        #[route("/profile")]
        Profile {},
        #[route("/settings")]
        Settings {},
        #[route("/workspace")]
        Workspace {},
        #[route("/templates")]
        Templates {},
    #[end_layout]

    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    let path = if segments.is_empty() {
        "/".into()
    } else {
        format!("/{}", segments.join("/"))
    };
    rsx! {
        div { class: "page-content",
            style: "min-height:100vh;display:flex;align-items:center;justify-content:center;",
            div { class: "card", style: "max-width:480px;text-align:center;padding:48px;",
                p { style: "font-size:48px;margin-bottom:16px;", "404" }
                h2 { style: "margin-bottom:8px;", "页面不存在" }
                p { class: "text-muted", style: "margin-bottom:24px;", "未找到：{path}" }
                Link { to: Route::Dashboard {}, class: "btn btn-primary", "返回工作台" }
            }
        }
    }
}
