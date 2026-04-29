use crate::routes::Route;
use crate::state::{AuthState, CreateDocTrigger};
use dioxus::prelude::*;
use dioxus::router::components::HistoryProvider;
use dioxus::web::WebHistory;
use std::rc::Rc;

const GLOBAL_CSS: &str = include_str!("../assets/style.css");
const DOCS_BASE_PATH: &str = "/docs";

#[component]
pub fn App() -> Element {
    let mut auth = use_context_provider(|| Signal::new(AuthState::init()));
    use_context_provider(|| Signal::new(CreateDocTrigger(false)));

    use_effect(move || {
        if auth.read().token.is_some() && auth.read().user.is_none() {
            spawn(async move {
                match crate::api::auth::me().await {
                    Ok(user) => auth.write().user = Some(user),
                    Err(e) => {
                        // 只有 token 真正失效（401/403）才退出登录；
                        // 后端未启动或网络异常时保留 token，让用户继续操作
                        if e.contains("401") || e.contains("403") {
                            auth.write().logout();
                        }
                    }
                }
            });
        }
    });

    rsx! {
        style { "{GLOBAL_CSS}" }
        style {
            r#"
            @import url('https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;800&display=swap');
            "#
        }
        HistoryProvider {
            history: |_| Rc::new(WebHistory::new(Some(DOCS_BASE_PATH.to_string()), true)) as Rc<dyn History>,
            Router::<Route> {}
        }
    }
}
