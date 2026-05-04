use crate::api::{members as members_api, notifications as notif_api};
use crate::models::Notification;
use dioxus::prelude::*;

fn can_accept_invitation(notification: &Notification) -> bool {
    notification.notification_type.as_deref() == Some("space_invitation")
        && notification
            .invite_token
            .as_deref()
            .map(str::trim)
            .is_some_and(|token| !token.is_empty())
}

#[component]
pub fn Notifications() -> Element {
    let mut refresh_epoch = use_signal(|| 0u32);
    let mut accepting_token = use_signal(String::new);
    let mut action_message = use_signal(String::new);
    let mut action_error = use_signal(String::new);

    let notifs_res = use_resource(move || async move {
        let _ = refresh_epoch();
        notif_api::list_notifications(1, 50).await
    });

    let mark_all = move |_| {
        action_message.set(String::new());
        action_error.set(String::new());
        spawn(async move {
            match notif_api::mark_all_read().await {
                Ok(_) => {
                    action_message.set("已全部标为已读".to_string());
                    refresh_epoch.set(refresh_epoch() + 1);
                }
                Err(e) => action_error.set(e),
            }
        });
    };

    let accept_invitation = move |invite_token: String| {
        action_message.set(String::new());
        action_error.set(String::new());
        accepting_token.set(invite_token.clone());
        spawn(async move {
            match members_api::accept_invitation(invite_token).await {
                Ok(_) => {
                    action_message.set("已接受邀请，空间权限已生效".to_string());
                    accepting_token.set(String::new());
                    refresh_epoch.set(refresh_epoch() + 1);
                }
                Err(e) => {
                    accepting_token.set(String::new());
                    action_error.set(e);
                }
            }
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
            if !action_message().is_empty() {
                div { class: "card", style: "margin-bottom:16px;border-color:#bbf7d0;background:#f0fdf4;color:#166534;padding:12px 16px;font-size:13px;",
                    "{action_message}"
                }
            }
            if !action_error().is_empty() {
                div { class: "card", style: "margin-bottom:16px;border-color:#fecaca;background:#fef2f2;color:#991b1b;padding:12px 16px;font-size:13px;",
                    "操作失败：{action_error}"
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
                                        notification: n.clone(),
                                        unread: true,
                                        accepting_token: accepting_token(),
                                        on_accept: accept_invitation,
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
                                            notification: n.clone(),
                                            unread: false,
                                            accepting_token: accepting_token(),
                                            on_accept: accept_invitation,
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
fn NotifItem(
    notification: Notification,
    unread: bool,
    accepting_token: String,
    on_accept: EventHandler<String>,
) -> Element {
    let icon_bg = if unread { "#eef2ff" } else { "#f1f5fb" };
    let icon = if unread { "🔔" } else { "📭" };
    let title = notification.title.clone();
    let sub = notification.message.clone().unwrap_or_default();
    let time = notification.created_at.clone().unwrap_or_default();
    let can_accept = can_accept_invitation(&notification);
    let invite_token = notification.invite_token.clone().unwrap_or_default();
    let is_accepting = can_accept && accepting_token == invite_token;
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
                if can_accept {
                    div { style: "margin-top:10px;display:flex;gap:8px;align-items:center;",
                        button {
                            class: "btn btn-sm btn-primary",
                            disabled: is_accepting,
                            onclick: move |_| on_accept.call(invite_token.clone()),
                            if is_accepting { "接受中…" } else { "接受邀请" }
                        }
                        if let Some(space_name) = notification.space_name.as_ref() {
                            span { class: "text-muted", style: "font-size:12px;", "空间：{space_name}" }
                        }
                        if let Some(role) = notification.role.as_ref() {
                            span { class: "badge", "{role}" }
                        }
                    }
                }
            }
            span { class: "notif-time", "{time}" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::can_accept_invitation;
    use crate::models::Notification;

    fn notification(notification_type: Option<&str>, invite_token: Option<&str>) -> Notification {
        Notification {
            id: Some("notification:one".to_string()),
            title: "邀请".to_string(),
            message: Some("加入空间".to_string()),
            notification_type: notification_type.map(str::to_string),
            is_read: false,
            created_at: None,
            link: None,
            invite_token: invite_token.map(str::to_string),
            space_name: Some("测试空间".to_string()),
            role: Some("editor".to_string()),
        }
    }

    #[test]
    fn space_invitation_with_token_can_be_accepted() {
        let notif = notification(Some("space_invitation"), Some("abc-token"));

        assert!(can_accept_invitation(&notif));
    }

    #[test]
    fn non_invitation_or_missing_token_cannot_be_accepted() {
        assert!(!can_accept_invitation(&notification(
            Some("system"),
            Some("abc-token")
        )));
        assert!(!can_accept_invitation(&notification(
            Some("space_invitation"),
            None
        )));
    }
}
