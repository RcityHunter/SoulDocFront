use crate::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn Install() -> Element {
    let mut step = use_signal(|| 1u32);
    let navigator = use_navigator();

    rsx! {
        document::Title { "安装向导 — SoulBook" }
        div { style: "min-height:100vh;background:var(--bg);display:flex;flex-direction:column;align-items:center;justify-content:center;padding:40px;",
            // 品牌标识
            div { style: "display:flex;align-items:center;gap:12px;margin-bottom:40px;",
                div { style: "width:48px;height:48px;border-radius:14px;background:var(--gradient);display:flex;align-items:center;justify-content:center;color:#fff;font-size:18px;font-weight:700;box-shadow:var(--shadow-primary);",
                    "SD"
                }
                div {
                    h1 { style: "font-size:22px;font-weight:700;letter-spacing:-.3px;", "SoulBook" }
                    p { style: "font-size:12.5px;color:var(--muted);", "安装向导" }
                }
            }

            div { style: "width:100%;max-width:640px;",
                // 步骤指示器
                div { class: "steps", style: "margin-bottom:32px;",
                    StepIndicator { num: 1, label: "系统检查", current: step(), done: step() > 1 }
                    div { class: "step-line" }
                    StepIndicator { num: 2, label: "数据库配置", current: step(), done: step() > 2 }
                    div { class: "step-line" }
                    StepIndicator { num: 3, label: "管理员账户", current: step(), done: step() > 3 }
                    div { class: "step-line" }
                    StepIndicator { num: 4, label: "完成", current: step(), done: false }
                }

                div { class: "card",
                    if step() == 1 {
                        div {
                            div { class: "card-header", h2 { style: "font-size:18px;", "系统环境检查" } }
                            div { style: "display:flex;flex-direction:column;gap:10px;margin-bottom:24px;",
                                CheckItem { label: "Rust 1.75+", status: "pass", detail: "1.78.0" }
                                CheckItem { label: "SurrealDB 可连接", status: "pass", detail: "127.0.0.1:8000" }
                                CheckItem { label: "Redis 可连接", status: "pass", detail: "127.0.0.1:6379" }
                                CheckItem { label: "对象存储 (S3/MinIO)", status: "warn", detail: "未配置，可后续设置" }
                                CheckItem { label: "端口 8080 可用", status: "pass", detail: "已可用" }
                            }
                            button { class: "btn btn-primary", onclick: move |_| step.set(2), "继续 →" }
                        }
                    }
                    if step() == 2 {
                        div {
                            div { class: "card-header", h2 { style: "font-size:18px;", "数据库配置" } }
                            div { class: "form-group",
                                label { class: "form-label", "SurrealDB 地址" }
                                input { class: "input", value: "ws://127.0.0.1:8000" }
                            }
                            div { class: "form-group",
                                label { class: "form-label", "命名空间" }
                                input { class: "input", value: "soulbook" }
                            }
                            div { class: "form-group",
                                label { class: "form-label", "数据库名" }
                                input { class: "input", value: "soulbook_prod" }
                            }
                            div { class: "form-group",
                                label { class: "form-label", "用户名" }
                                input { class: "input", value: "root" }
                            }
                            div { class: "form-group",
                                label { class: "form-label", "密码" }
                                input { class: "input", r#type: "password" }
                            }
                            div { style: "display:flex;gap:8px;",
                                button { class: "btn btn-sm", "测试连接" }
                                button { class: "btn btn-primary", onclick: move |_| step.set(3), "继续 →" }
                            }
                        }
                    }
                    if step() == 3 {
                        div {
                            div { class: "card-header", h2 { style: "font-size:18px;", "创建管理员账户" } }
                            div { class: "form-group",
                                label { class: "form-label", "显示名称" }
                                input { class: "input", placeholder: "Admin" }
                            }
                            div { class: "form-group",
                                label { class: "form-label", "邮箱" }
                                input { class: "input", r#type: "email", placeholder: "admin@example.com" }
                            }
                            div { class: "form-group",
                                label { class: "form-label", "密码" }
                                input { class: "input", r#type: "password", placeholder: "最少 12 位" }
                            }
                            div { class: "form-group",
                                label { class: "form-label", "组织名称" }
                                input { class: "input", placeholder: "我的团队" }
                            }
                            button { class: "btn btn-primary", onclick: move |_| step.set(4), "完成安装 →" }
                        }
                    }
                    if step() == 4 {
                        div { style: "text-align:center;padding:24px;",
                            div { style: "font-size:56px;margin-bottom:16px;", "🎉" }
                            h2 { style: "font-size:22px;font-weight:700;margin-bottom:8px;", "安装完成！" }
                            p { style: "color:var(--muted);margin-bottom:24px;", "SoulBook 已成功安装并准备就绪。" }
                            button {
                                class: "btn btn-primary btn-lg",
                                onclick: move |_| { navigator.push(Route::Login {}); },
                                "进入 SoulBook →"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StepIndicator(num: u32, label: &'static str, current: u32, done: bool) -> Element {
    rsx! {
        div { class: if current == num { "step active" } else if done { "step done" } else { "step" },
            div { class: "step-num",
                if done { "✓" } else { "{num}" }
            }
            span { class: "step-label", "{label}" }
        }
    }
}

#[component]
fn CheckItem(label: &'static str, status: &'static str, detail: &'static str) -> Element {
    let (icon, cls) = match status {
        "pass" => ("✅", "color:var(--success)"),
        "warn" => ("⚠️", "color:var(--warning)"),
        _ => ("❌", "color:var(--danger)"),
    };
    rsx! {
        div { style: "display:flex;align-items:center;justify-content:space-between;padding:10px 14px;border:1px solid var(--line);border-radius:9px;background:var(--panel2);",
            div { style: "display:flex;align-items:center;gap:10px;",
                span { style: "{cls}", "{icon}" }
                span { style: "font-size:13.5px;font-weight:500;", "{label}" }
            }
            span { style: "font-size:12.5px;color:var(--muted);", "{detail}" }
        }
    }
}
