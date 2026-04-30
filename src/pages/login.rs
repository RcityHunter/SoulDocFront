use crate::api::auth as auth_api;
use crate::routes::Route;
use crate::state::AuthState;
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error_msg = use_signal(|| String::new());
    let mut loading = use_signal(|| false);
    let navigator = use_navigator();
    let mut auth = use_context::<Signal<AuthState>>();

    // Redirect to dashboard if already authenticated
    use_effect(move || {
        if auth.read().is_authenticated() {
            navigator.replace(Route::Dashboard {});
        }
    });

    let handle_login = move |e: FormEvent| {
        e.prevent_default();
        e.stop_propagation();
        if loading() {
            return;
        }
        let em = email.read().trim().to_string();
        let pw = password.read().clone();
        if em.is_empty() || pw.is_empty() {
            error_msg.set("请输入邮箱和密码".to_string());
            return;
        }
        loading.set(true);
        error_msg.set(String::new());
        spawn(async move {
            match auth_api::login(em, pw).await {
                Ok(result) => {
                    auth.write().login(result.token, result.user);
                    // use_effect above will handle navigation when auth state changes
                }
                Err(e) => {
                    let msg =
                        if e.contains("401") || e.contains("Authentication") || e.contains("邮箱")
                        {
                            "邮箱或密码不正确".to_string()
                        } else {
                            format!("登录失败：{}", e)
                        };
                    error_msg.set(msg);
                    loading.set(false);
                }
            }
        });
    };

    rsx! {
        document::Title { "登录 — SoulBook" }
        div { style: "min-height:100vh;display:flex;background:var(--bg);",
            // Left brand panel
            div { style: "flex:0 0 420px;background:linear-gradient(160deg,#1e1b4b 0%,#312e81 50%,#1e3a5f 100%);display:flex;flex-direction:column;justify-content:center;padding:60px 52px;",
                div { style: "margin-bottom:40px;",
                    div { style: "width:52px;height:52px;border-radius:14px;background:rgba(255,255,255,.15);display:flex;align-items:center;justify-content:center;font-size:20px;font-weight:700;color:#fff;letter-spacing:.08em;margin-bottom:20px;",
                        "SD"
                    }
                    h1 { style: "font-size:28px;font-weight:800;color:#fff;letter-spacing:-.5px;margin-bottom:10px;", "SoulBook" }
                    p { style: "font-size:15px;color:rgba(255,255,255,.65);line-height:1.7;",
                        "AI 原生知识工作台。人类与 AI 共享同一套角色与权限体系，协同生产、治理和发布知识。"
                    }
                }
                div { style: "display:flex;flex-direction:column;gap:16px;",
                    FeaturePoint { icon: "🗂️", text: "Scope → Space → Document 内容层次" }
                    FeaturePoint { icon: "🤖", text: "AI 与人类平等的用户模型" }
                    FeaturePoint { icon: "🌍", text: "多语言独立版本治理" }
                    FeaturePoint { icon: "🔀", text: "发布链 + GitHub 同步" }
                }
            }

            // Right login form
            div { style: "flex:1;display:flex;align-items:center;justify-content:center;padding:48px;",
                div { style: "width:100%;max-width:420px;",
                    h2 { style: "font-size:26px;font-weight:700;letter-spacing:-.4px;margin-bottom:8px;", "欢迎回来" }
                    p { style: "color:var(--muted);margin-bottom:32px;", "登录到您的 SoulBook 账户" }

                    form { onsubmit: handle_login,
                        div { class: "form-group",
                            label { class: "form-label", r#for: "email", "邮箱地址" }
                            input {
                                id: "email",
                                class: "input",
                                r#type: "email",
                                placeholder: "admin@soulbook.io",
                                value: "{email}",
                                oninput: move |e| email.set(e.value())
                            }
                        }
                        div { class: "form-group",
                            label { class: "form-label", r#for: "password",
                                "密码"
                                a { style: "float:right;color:var(--primary);font-size:12.5px;", href: "#", "忘记密码？" }
                            }
                            input {
                                id: "password",
                                class: "input",
                                r#type: "password",
                                placeholder: "••••••••",
                                value: "{password}",
                                oninput: move |e| password.set(e.value())
                            }
                        }
                        div { style: "display:flex;align-items:center;gap:8px;margin-bottom:20px;",
                            input { r#type: "checkbox", id: "remember", style: "cursor:pointer;" }
                            label { r#for: "remember", style: "font-size:13px;color:var(--text3);cursor:pointer;", "保持登录状态" }
                        }
                        if !error_msg().is_empty() {
                            div {
                                style: "margin-bottom:14px;padding:10px 14px;background:#fef2f2;border:1px solid #fca5a5;border-radius:8px;color:#dc2626;font-size:13px;",
                                "{error_msg}"
                            }
                        }
                        button {
                            r#type: "submit",
                            class: "btn btn-primary w-full btn-lg",
                            style: "justify-content:center;width:100%;",
                            disabled: loading(),
                            if loading() { "登录中…" } else { "登录" }
                        }
                    }

                    div { style: "margin-top:24px;padding-top:24px;border-top:1px solid var(--line);text-align:center;",
                        p { style: "font-size:13px;color:var(--muted);margin-bottom:12px;", "或通过以下方式登录" }
                        a {
                            href: "/api/docs/auth/soulauth/start",
                            class: "btn w-full",
                            style: "width:100%;justify-content:center;display:flex;align-items:center;gap:8px;margin-bottom:10px;text-decoration:none;",
                            span {
                                style: "display:inline-flex;align-items:center;justify-content:center;width:18px;height:18px;border-radius:50%;background:linear-gradient(45deg,#4285f4,#ea4335,#fbbc05,#34a853);color:#fff;font-size:11px;font-weight:800;",
                                "G"
                            }
                            span { "使用 Google / SoulAuth 登录" }
                        }
                        div { style: "display:flex;gap:10px;",
                            button { class: "btn w-full", style: "flex:1;justify-content:center;",
                                "🐙 GitHub"
                            }
                            button { class: "btn w-full", style: "flex:1;justify-content:center;",
                                "🌐 SSO"
                            }
                        }
                    }

                    div { style: "margin-top:24px;text-align:center;",
                        p { style: "font-size:13px;color:var(--muted);",
                            "还没有账户？ "
                            Link { to: Route::Install {}, style: "color:var(--primary);font-weight:500;", "安装 SoulBook" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FeaturePoint(icon: &'static str, text: &'static str) -> Element {
    rsx! {
        div { style: "display:flex;align-items:center;gap:12px;",
            span { style: "font-size:18px;", "{icon}" }
            span { style: "font-size:13.5px;color:rgba(255,255,255,.75);", "{text}" }
        }
    }
}
