use crate::api::auth as auth_api;
use crate::state::AuthState;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use std::collections::HashMap;

const TOKEN_KEY: &str = "soulbook_token";
const LEGACY_TOKEN_KEY: &str = "souldoc_token";

#[component]
pub fn Sso() -> Element {
    let mut auth = use_context::<Signal<AuthState>>();
    let mut started = use_signal(|| false);
    let mut message = use_signal(|| "正在完成登录...".to_string());

    use_effect(move || {
        if started() {
            return;
        }
        started.set(true);

        spawn(async move {
            let params = query_params();
            let Some(token) = params
                .get("token")
                .map(String::as_str)
                .map(str::trim)
                .filter(|token| !token.is_empty())
                .map(str::to_string)
            else {
                message.set("登录回调缺少 token，请重新登录。".to_string());
                return;
            };

            save_token(&token);
            auth.write().token = Some(token);

            if let Ok(user) = auth_api::me().await {
                auth.write().user = Some(user);
            }

            let next = safe_next(params.get("next").map(String::as_str));
            redirect_to(&next);
        });
    });

    rsx! {
        div {
            style: "min-height:100vh;display:flex;align-items:center;justify-content:center;background:#f8fafc;color:#0f172a;",
            div {
                style: "width:min(420px,calc(100vw - 32px));padding:32px;border-radius:20px;background:white;box-shadow:0 20px 60px rgba(15,23,42,.12);text-align:center;",
                div { style: "font-size:32px;margin-bottom:12px;", "SoulBook" }
                h1 { style: "font-size:20px;margin:0 0 8px;", "正在登录" }
                p { style: "margin:0;color:#64748b;line-height:1.6;", "{message}" }
                a { href: "/docs/login", style: "display:inline-block;margin-top:20px;color:#2563eb;text-decoration:none;", "返回登录页" }
            }
        }
    }
}

fn save_token(token: &str) {
    let _ = LocalStorage::set(TOKEN_KEY, token);
    let _ = LocalStorage::set(LEGACY_TOKEN_KEY, token);
}

fn query_params() -> HashMap<String, String> {
    let search = web_sys::window()
        .and_then(|window| window.location().search().ok())
        .unwrap_or_default();

    search
        .trim_start_matches('?')
        .split('&')
        .filter(|pair| !pair.is_empty())
        .filter_map(|pair| {
            let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
            Some((percent_decode(key), percent_decode(value)))
        })
        .collect()
}

fn safe_next(raw: Option<&str>) -> String {
    let default = "/docs/".to_string();
    let Some(raw) = raw else {
        return default;
    };

    let decoded = percent_decode(raw);
    let target = decoded.trim();
    if target.is_empty() {
        return default;
    }

    if let Some(path) = normalize_frontend_path(target) {
        return path;
    }

    let origin = web_sys::window()
        .and_then(|window| window.location().origin().ok())
        .unwrap_or_default();
    if !origin.is_empty() && target.starts_with(&origin) {
        let path = &target[origin.len()..];
        if let Some(path) = normalize_frontend_path(path) {
            return path;
        }
    }

    default
}

fn normalize_frontend_path(path: &str) -> Option<String> {
    if path.starts_with("//") {
        return None;
    }

    if path == "/" {
        return Some("/docs/".to_string());
    }

    if path == "/docs" || path.starts_with("/docs/") {
        return Some(path.to_string());
    }

    if path.starts_with('/')
        && !path.starts_with("/api/")
        && !path.starts_with("/auth/")
        && !path.starts_with("/agent/")
    {
        return Some(format!("/docs{path}"));
    }

    None
}

fn redirect_to(target: &str) {
    if let Some(window) = web_sys::window() {
        let _ = window.location().set_href(target);
    }
}

fn percent_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;

    while index < bytes.len() {
        match bytes[index] {
            b'%' if index + 2 < bytes.len() => {
                if let (Some(high), Some(low)) =
                    (hex_value(bytes[index + 1]), hex_value(bytes[index + 2]))
                {
                    decoded.push((high << 4) | low);
                    index += 3;
                } else {
                    decoded.push(bytes[index]);
                    index += 1;
                }
            }
            b'+' => {
                decoded.push(b' ');
                index += 1;
            }
            byte => {
                decoded.push(byte);
                index += 1;
            }
        }
    }

    String::from_utf8_lossy(&decoded).into_owned()
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
