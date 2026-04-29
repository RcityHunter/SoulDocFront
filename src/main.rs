#![allow(non_snake_case)]

mod api;
mod app;
mod components;
mod models;
mod pages;
mod routes;
mod state;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    dioxus::launch(app::App);
}
