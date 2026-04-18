#![allow(dead_code, unused_imports)]

mod api;
mod app;
mod prototype;
mod routes;

#[cfg(target_arch = "wasm32")]
use app::App;

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(App);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("SoulDoc 前端请通过 `dx serve` 在 Web 平台运行。");
}
