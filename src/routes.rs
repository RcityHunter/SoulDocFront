use dioxus::prelude::*;

use crate::prototype::{PrototypeId, PrototypeView};

macro_rules! static_prototype_route {
    ($name:ident, $page:expr) => {
        #[component]
        fn $name() -> Element {
            rsx! { PrototypeView { page: $page } }
        }
    };
}

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Root {},
    #[route("/index.html")]
    Index {},
    #[route("/workspace.html")]
    Workspace {},
    #[route("/spaces.html")]
    Spaces {},
    #[route("/space.html")]
    Space {},
    #[route("/docs.html")]
    Docs {},
    #[route("/templates.html")]
    Templates {},
    #[route("/editor.html")]
    Editor {},
    #[route("/versions.html")]
    Versions {},
    #[route("/change-request.html")]
    ChangeRequest {},
    #[route("/members.html")]
    Members {},
    #[route("/tags.html")]
    Tags {},
    #[route("/files.html")]
    Files {},
    #[route("/notifications.html")]
    Notifications {},
    #[route("/profile.html")]
    Profile {},
    #[route("/search.html")]
    Search {},
    #[route("/language.html")]
    Language {},
    #[route("/ai-tasks.html")]
    AiTasks {},
    #[route("/ai-tools.html")]
    AiTools {},
    #[route("/seo.html")]
    Seo {},
    #[route("/public-doc.html")]
    PublicDoc {},
    #[route("/git-sync.html")]
    GitSync {},
    #[route("/developer.html")]
    Developer {},
    #[route("/organization.html")]
    Organization {},
    #[route("/settings.html")]
    Settings {},
    #[route("/install.html")]
    Install {},
    #[route("/login.html")]
    Login {},
    #[route("/ai-center.html")]
    AiCenter {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

static_prototype_route!(Root, PrototypeId::Index);
static_prototype_route!(Index, PrototypeId::Index);
static_prototype_route!(Workspace, PrototypeId::Workspace);
static_prototype_route!(Spaces, PrototypeId::Spaces);
static_prototype_route!(Space, PrototypeId::Space);
static_prototype_route!(Docs, PrototypeId::Docs);
static_prototype_route!(Templates, PrototypeId::Templates);
static_prototype_route!(Editor, PrototypeId::Editor);
static_prototype_route!(Versions, PrototypeId::Versions);
static_prototype_route!(ChangeRequest, PrototypeId::ChangeRequest);
static_prototype_route!(Members, PrototypeId::Members);
static_prototype_route!(Tags, PrototypeId::Tags);
static_prototype_route!(Files, PrototypeId::Files);
static_prototype_route!(Notifications, PrototypeId::Notifications);
static_prototype_route!(Profile, PrototypeId::Profile);
static_prototype_route!(Search, PrototypeId::Search);
static_prototype_route!(Language, PrototypeId::Language);
static_prototype_route!(AiTasks, PrototypeId::AiTasks);
static_prototype_route!(AiTools, PrototypeId::AiTools);
static_prototype_route!(Seo, PrototypeId::Seo);
static_prototype_route!(PublicDoc, PrototypeId::PublicDoc);
static_prototype_route!(GitSync, PrototypeId::GitSync);
static_prototype_route!(Developer, PrototypeId::Developer);
static_prototype_route!(Organization, PrototypeId::Organization);
static_prototype_route!(Settings, PrototypeId::Settings);
static_prototype_route!(Install, PrototypeId::Install);
static_prototype_route!(Login, PrototypeId::Login);

#[component]
fn AiCenter() -> Element {
    let navigator = use_navigator();

    use_effect(move || {
        let _ = navigator.replace(Route::Language {});
    });

    rsx! { PrototypeView { page: PrototypeId::AiCenter } }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    let missing = if segments.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", segments.join("/"))
    };

    rsx! {
        document::Title { "页面不存在 — SoulDoc" }
        div {
            class: "page-content",
            style: "min-height:100vh;display:flex;align-items:center;justify-content:center;padding:48px;",
            div {
                class: "card",
                style: "max-width:560px;text-align:center;",
                h1 { style: "font-size:28px;margin-bottom:10px;", "页面不存在" }
                p {
                    class: "text-muted",
                    style: "margin-bottom:20px;",
                    "未找到路由：{missing}"
                }
                Link {
                    class: "btn btn-primary",
                    to: Route::Index {},
                    "返回工作台"
                }
            }
        }
    }
}
