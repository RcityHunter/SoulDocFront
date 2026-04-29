use crate::api::auth as auth_api;
use crate::routes::Route;
use crate::state::AuthState;
use dioxus::prelude::*;

#[component]
pub fn Profile() -> Element {
    let auth = use_context::<Signal<AuthState>>();

    let display_name = {
        let state = auth.read();
        state
            .user
            .as_ref()
            .and_then(|u| u.username.as_deref())
            .unwrap_or("用户")
            .to_string()
    };
    let email = {
        let state = auth.read();
        state
            .user
            .as_ref()
            .map(|u| u.email.clone())
            .unwrap_or_default()
    };
    let initial = display_name
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "U".to_string());

    let mut username_val = use_signal(|| display_name.clone());
    let mut save_msg = use_signal(|| String::new());
    let mut saving = use_signal(|| false);

    let mut old_pw = use_signal(|| String::new());
    let mut new_pw = use_signal(|| String::new());
    let mut pw_msg = use_signal(|| String::new());
    let mut pw_saving = use_signal(|| false);

    let navigator = use_navigator();
    let mut auth_w = auth;

    let do_save = move |_| {
        saving.set(true);
        save_msg.set(String::new());
        let uname = username_val.read().trim().to_string();
        spawn(async move {
            match auth_api::update_profile(auth_api::UpdateProfileRequest {
                username: Some(uname),
                avatar_url: None,
            })
            .await
            {
                Ok(_) => save_msg.set("保存成功".to_string()),
                Err(e) => save_msg.set(format!("保存失败：{}", e)),
            }
            saving.set(false);
        });
    };

    let do_change_pw = move |_| {
        pw_saving.set(true);
        pw_msg.set(String::new());
        let old = old_pw.read().clone();
        let new = new_pw.read().clone();
        if new.len() < 6 {
            pw_msg.set("新密码至少6位".to_string());
            pw_saving.set(false);
            return;
        }
        spawn(async move {
            match auth_api::change_password(old, new).await {
                Ok(_) => {
                    pw_msg.set("密码已修改".to_string());
                    old_pw.set(String::new());
                    new_pw.set(String::new());
                }
                Err(e) => pw_msg.set(format!("修改失败：{}", e)),
            }
            pw_saving.set(false);
        });
    };

    let do_logout = move |_| {
        auth_w.write().logout();
        navigator.push(Route::Login {});
    };

    rsx! {
        document::Title { "个人中心 — SoulBook" }
        div { class: "page-content",
            div { class: "grid-2", style: "gap:20px;align-items:start;",
                div { style: "display:flex;flex-direction:column;gap:16px;",
                    div { class: "card", style: "text-align:center;padding:32px;",
                        div { class: "avatar avatar-lg", style: "background:var(--gradient);margin:0 auto 16px;font-size:22px;", "{initial}" }
                        h2 { style: "font-size:20px;font-weight:700;margin-bottom:4px;", "{display_name}" }
                        p { style: "color:var(--muted);margin-bottom:10px;", "{email}" }
                        div { style: "display:flex;justify-content:center;gap:8px;",
                            span { class: "identity-badge human", "👤 人类" }
                        }
                    }
                    div { class: "card",
                        div { class: "card-header", h3 { "退出登录" } }
                        button { class: "btn btn-sm", style: "width:100%;justify-content:center;color:#dc2626;",
                            onclick: do_logout, "退出登录"
                        }
                    }
                }

                div { style: "display:flex;flex-direction:column;gap:16px;",
                    div { class: "card",
                        div { class: "card-header", h3 { "基本信息" } }
                        div { class: "form-group",
                            label { class: "form-label", "用户名" }
                            input { class: "input", value: "{username_val}",
                                oninput: move |e| username_val.set(e.value()) }
                        }
                        div { class: "form-group",
                            label { class: "form-label", "邮箱" }
                            input { class: "input", r#type: "email", value: "{email}", disabled: true }
                        }
                        if !save_msg().is_empty() {
                            p { style: "font-size:13px;color:var(--primary);margin-bottom:10px;", "{save_msg}" }
                        }
                        button { class: "btn btn-primary", disabled: saving(), onclick: do_save,
                            if saving() { "保存中…" } else { "保存更改" }
                        }
                    }

                    div { class: "card",
                        div { class: "card-header", h3 { "修改密码" } }
                        div { class: "form-group",
                            label { class: "form-label", "当前密码" }
                            input { class: "input", r#type: "password", value: "{old_pw}",
                                oninput: move |e| old_pw.set(e.value()) }
                        }
                        div { class: "form-group",
                            label { class: "form-label", "新密码（至少6位）" }
                            input { class: "input", r#type: "password", value: "{new_pw}",
                                oninput: move |e| new_pw.set(e.value()) }
                        }
                        if !pw_msg().is_empty() {
                            p { style: "font-size:13px;color:var(--primary);margin-bottom:10px;", "{pw_msg}" }
                        }
                        button { class: "btn btn-sm", style: "width:100%;justify-content:center;",
                            disabled: pw_saving(), onclick: do_change_pw,
                            if pw_saving() { "修改中…" } else { "修改密码" }
                        }
                    }
                }
            }
        }
    }
}
