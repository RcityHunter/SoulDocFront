use crate::api::git_sync as git_api;
use crate::api::spaces as spaces_api;
use dioxus::prelude::*;
use serde_json::Value;

#[component]
pub fn GitSync() -> Element {
    let repos_res = use_resource(|| async move { git_api::list_repos().await });
    let spaces_res = use_resource(|| async move { spaces_api::list_spaces(1, 50).await });
    let mut show_bind = use_signal(|| false);
    let mut selected_space = use_signal(|| String::new());
    let mut github_url = use_signal(|| String::new());
    let mut binding = use_signal(|| false);
    let mut bind_err = use_signal(|| String::new());
    let mut sync_msg = use_signal(|| String::new());

    let do_bind = move |_| {
        let space = selected_space.read().trim().to_string();
        let url = github_url.read().trim().to_string();
        if space.is_empty() || url.is_empty() {
            return;
        }
        binding.set(true);
        bind_err.set(String::new());
        spawn(async move {
            match git_api::create_repo(&serde_json::json!({
                "space_slug": space,
                "github_url": url,
                "branch": "main",
                "direction": "bidirectional",
                "auto_sync": false
            }))
            .await
            {
                Ok(_) => {
                    show_bind.set(false);
                    github_url.set(String::new());
                }
                Err(e) => bind_err.set(e),
            }
            binding.set(false);
        });
    };

    rsx! {
        document::Title { "GitHub 同步 — SoulBook" }
        div { class: "page-content",
            div { class: "page-header",
                div { class: "page-header-left",
                    h1 { "🔀 GitHub 同步" }
                    p { "将 Space 内容双向同步到 GitHub 仓库，支持分支策略和冲突处理" }
                }
                div { class: "page-header-actions",
                    button { class: "btn btn-primary", onclick: move |_| show_bind.set(true), "＋ 绑定仓库" }
                }
            }

            // Bind modal
            if show_bind() {
                div { style: "position:fixed;inset:0;background:rgba(0,0,0,.4);z-index:300;display:flex;align-items:center;justify-content:center;",
                    onclick: move |_| show_bind.set(false),
                    div { class: "card", style: "width:440px;padding:24px;", onclick: move |e| e.stop_propagation(),
                        h3 { style: "font-size:15px;font-weight:700;margin-bottom:16px;", "绑定 GitHub 仓库" }
                        div { class: "form-group",
                            label { class: "form-label", "关联空间" }
                            match &*spaces_res.read() {
                                Some(Ok(data)) => {
                                    let spaces = data.spaces.as_ref().or(data.items.as_ref()).cloned().unwrap_or_default();
                                    rsx! {
                                        select { class: "input", onchange: move |e| selected_space.set(e.value()),
                                            option { value: "", "— 选择空间 —" }
                                            for s in spaces.iter() {
                                                option { value: "{s.slug}", "{s.name}" }
                                            }
                                        }
                                    }
                                },
                                _ => rsx! { input { class: "input", placeholder: "space-slug", oninput: move |e| selected_space.set(e.value()) } }
                            }
                        }
                        div { class: "form-group",
                            label { class: "form-label", "GitHub 仓库地址" }
                            input { class: "input", placeholder: "https://github.com/org/repo", value: "{github_url}",
                                oninput: move |e| github_url.set(e.value()) }
                        }
                        if !bind_err().is_empty() {
                            p { style: "color:#dc2626;font-size:13px;margin-bottom:10px;", "{bind_err}" }
                        }
                        div { style: "display:flex;gap:10px;justify-content:flex-end;",
                            button { class: "btn", onclick: move |_| show_bind.set(false), "取消" }
                            button { class: "btn btn-primary", disabled: binding(), onclick: do_bind,
                                if binding() { "绑定中…" } else { "绑定" }
                            }
                        }
                    }
                }
            }

            // Workflow intro
            div { class: "card", style: "margin-bottom:24px;",
                div { class: "card-header", h3 { "GitHub 同步工作流" } }
                div { class: "grid-2", style: "gap:16px;",
                    WorkflowStep { num: "1", title: "SoulBook → GitHub", desc: "文档发布时，自动创建 PR 到目标分支，包含 Markdown 和资源文件。" }
                    WorkflowStep { num: "2", title: "GitHub → SoulBook", desc: "监听仓库 Push 事件，自动同步变更到对应 Space 文档。" }
                    WorkflowStep { num: "3", title: "冲突处理", desc: "检测到冲突时，暂停自动同步，通知 Owner 手动 merge。" }
                    WorkflowStep { num: "4", title: "审计日志", desc: "每次同步操作均记录到审计日志，支持回溯和恢复。" }
                }
            }

            if !sync_msg().is_empty() {
                div { style: "padding:10px 14px;background:var(--panel2);border:1px solid var(--line);border-radius:8px;font-size:13px;margin-bottom:16px;",
                    "{sync_msg}"
                }
            }

            match &*repos_res.read() {
                None => rsx! { p { style: "color:var(--muted);", "加载中…" } },
                Some(Err(e)) => rsx! { p { style: "color:#dc2626;", "加载失败：{e}" } },
                Some(Ok(repos)) if repos.is_empty() => rsx! {
                    div { style: "text-align:center;padding:60px;color:var(--muted);",
                        div { style: "font-size:48px;margin-bottom:12px;", "🔀" }
                        h3 { "暂无绑定仓库" }
                        p { style: "font-size:13px;margin-bottom:20px;", "点击「绑定仓库」将 Space 内容与 GitHub 仓库关联" }
                        button { class: "btn btn-primary", onclick: move |_| show_bind.set(true), "＋ 绑定第一个仓库" }
                    }
                },
                Some(Ok(repos)) => rsx! {
                    div { style: "display:flex;flex-direction:column;gap:14px;",
                        for repo in repos.iter() {
                            RepoCard {
                                repo: repo.clone(),
                                on_sync: move |id: String| {
                                    sync_msg.set("同步中…".to_string());
                                    spawn(async move {
                                        match git_api::trigger_sync(&id).await {
                                            Ok(_) => sync_msg.set("✅ 同步任务已提交".to_string()),
                                            Err(e) => sync_msg.set(format!("❌ 同步失败：{}", e)),
                                        }
                                    });
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}

#[component]
fn RepoCard(repo: Value, on_sync: EventHandler<String>) -> Element {
    let id = repo
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim_start_matches("git_repo:")
        .to_string();
    let space = repo
        .get("space_slug")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let url = repo
        .get("github_url")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let branch = repo
        .get("branch")
        .and_then(|v| v.as_str())
        .unwrap_or("main")
        .to_string();
    let status = repo
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("connected")
        .to_string();
    rsx! {
        div { class: "card",
            div { style: "display:flex;align-items:center;justify-content:space-between;",
                div { style: "display:flex;align-items:center;gap:12px;",
                    div { style: "width:40px;height:40px;border-radius:10px;background:var(--primary-light);display:flex;align-items:center;justify-content:center;font-size:20px;", "🔀" }
                    div {
                        p { style: "font-weight:600;", "{space}" }
                        p { style: "font-size:12px;color:var(--muted);", "{url} · {branch}" }
                    }
                }
                div { style: "display:flex;align-items:center;gap:8px;",
                    span { class: "badge badge-success", "{status}" }
                    button {
                        class: "btn btn-sm btn-primary",
                        onclick: move |_| on_sync.call(id.clone()),
                        "立即同步"
                    }
                }
            }
        }
    }
}

#[component]
fn WorkflowStep(num: &'static str, title: &'static str, desc: &'static str) -> Element {
    rsx! {
        div { style: "display:flex;gap:12px;",
            div { style: "width:28px;height:28px;border-radius:10px;background:var(--primary-light);color:var(--primary);display:flex;align-items:center;justify-content:center;font-size:12px;font-weight:700;flex-shrink:0;",
                "{num}"
            }
            div {
                p { style: "font-weight:600;font-size:13.5px;margin-bottom:3px;", "{title}" }
                p { style: "font-size:13px;color:var(--muted);", "{desc}" }
            }
        }
    }
}
