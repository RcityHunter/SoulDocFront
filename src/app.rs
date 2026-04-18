use dioxus::prelude::*;

use crate::{prototype::GLOBAL_CSS, routes::Route};

#[component]
pub fn App() -> Element {
    rsx! {
        style { "{GLOBAL_CSS}" }
        Router::<Route> {}
    }
}
