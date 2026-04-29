use crate::api::notifications as notif_api;
use crate::models::Notification;
use dioxus::prelude::*;

#[component]
pub fn Notifications() -> Element {
    let notifs_res = use_resource(|| async move { notif_api::list_notifications(1, 50).await });

    let mark_all = move |_| {
        spawn(async move {
            let _ = notif_api::mark_all_read().await;
        });
    };

    rsx! {
        document::Title { "通知中心 — SoulBook" }
        div { class: "page-content",
            div { class: "page-header",
                div { class: "page-header-left",
                    h1 { "🔔 通知中心" }
                    p { "变更请求、AI 任务、成员邀请和系统通知" }
                }
                div { class: "page-header-actions",
                    button { class: "btn btn-sm", onclick: mark_all, "全部标为已读" }
                }
            }

            match &*notifs_res.read() {
                None => rsx! { div { class: "text-muted", style: "padding:40px;text-align:center;", "加载中…" } },
                Some(Err(e)) => rsx! { div { style: "color:#dc2626;padding:40px;text-align:center;", "加载失败：{e}" } },
                Some(Ok(data)) => {
                    let notifs: Vec<Notification> = data.notifications.as_ref().cloned().unwrap_or_default();
                    let unread: Vec<_> = notifs.iter().filter(|n| !n.is_read).collect();
                    let read: Vec<_> = notifs.iter().filter(|n| n.is_read).collect();
                    rsx! {
                        div { class: "card",
                            div { class: "card-header",
                                h3 { "未读通知 ({unread.len()})" }
                                div { class: "card-actions",
                                    span { class: "badge badge-primary", "{unread.len()}" }
                                }
                            }
                            div {
                                if unread.is_empty() {
                                    p { style: "padding:24px;color:var(--muted);text-align:center;font-size:13px;", "暂无未读通知" }
                                }
                                for n in unread {
                                    NotifItem {
                                        title: n.title.clone(),
                                        sub: n.message.clone().unwrap_or_default(),
                                        time: n.created_at.clone().unwrap_or_default(),
                                        unread: true,
                                    }
                                }
                            }
                        }
                        if !read.is_empty() {
                            div { class: "card", style: "margin-top:20px;",
                                div { class: "card-header", h3 { "已读通知 ({read.len()})" } }
                                div {
                                    for n in read {
                                        NotifItem {
                                            title: n.title.clone(),
                                            sub: n.message.clone().unwrap_or_default(),
                                            time: n.created_at.clone().unwrap_or_default(),
                                            unread: false,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NotifItem(title: String, sub: String, time: String, unread: bool) -> Element {
    let icon_bg = if unread { "#eef2ff" } else { "#f1f5fb" };
    let icon = if unread { "🔔" } else { "📭" };
    rsx! {
        div { class: if unread { "notif-item unread" } else { "notif-item" },
            div { style: "width:36px;height:36px;border-radius:9px;background:{icon_bg};display:flex;align-items:center;justify-content:center;font-size:16px;flex-shrink:0;",
                "{icon}"
            }
            div { class: "notif-body",
                p { class: "notif-title",
                    "{title}"
                    if unread {
                        span { style: "display:inline-block;width:7px;height:7px;border-radius:50%;background:var(--primary);margin-left:8px;vertical-align:middle;" }
                    }
                }
                p { class: "notif-sub", "{sub}" }
            }
            span { class: "notif-time", "{time}" }
        }
    }
}
