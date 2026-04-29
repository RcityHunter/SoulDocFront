use crate::api::members as members_api;
use crate::api::spaces as spaces_api;
use dioxus::prelude::*;

// ── Role badge color ────────────────────────────────────────────────────────

fn role_color(role: &str) -> &'static str {
    match role.to_lowercase().as_str() {
        "owner" => "#ef4444",
        "admin" => "#3b82f6",
        "editor" => "#10b981",
        "member" => "#8b5cf6",
        _ => "#6b7280",
    }
}

// ── Component ───────────────────────────────────────────────────────────────

#[component]
pub fn Members() -> Element {
    let spaces_res = use_resource(|| async move { spaces_api::list_spaces(1, 50).await });
    let mut selected_slug = use_signal(|| String::new());
    let mut selected_space_name = use_signal(|| String::new());
    let mut active_tab = use_signal(|| "members".to_string());
    let mut search_query = use_signal(|| String::new());

    use_effect(move || {
        if selected_slug.read().is_empty() {
            if let Some(Ok(data)) = &*spaces_res.read() {
                if let Some(first) = data
                    .spaces
                    .as_ref()
                    .or(data.items.as_ref())
                    .and_then(|s| s.first())
                {
                    selected_slug.set(first.slug.clone());
                    selected_space_name.set(first.name.clone());
                }
            }
        }
    });

    let members_res = use_resource(move || {
        let slug = selected_slug.read().clone();
        async move {
            if slug.is_empty() {
                return Err("请选择空间".to_string());
            }
            members_api::list_members(&slug).await
        }
    });

    let mut invite_email = use_signal(|| String::new());
    let mut invite_role = use_signal(|| "editor".to_string());
    let mut invite_err = use_signal(|| String::new());
    let mut inviting = use_signal(|| false);
    let mut show_invite = use_signal(|| false);

    let do_invite = move |_| {
        let slug = selected_slug.read().clone();
        let email = invite_email.read().trim().to_string();
        if slug.is_empty() || email.is_empty() {
            return;
        }
        inviting.set(true);
        invite_err.set(String::new());
        spawn(async move {
            match members_api::invite_member(
                &slug,
                members_api::InviteRequest {
                    email: Some(email),
                    role: invite_role.read().clone(),
                    message: None,
                },
            )
            .await
            {
                Ok(_) => {
                    show_invite.set(false);
                    invite_email.set(String::new());
                }
                Err(e) => invite_err.set(e),
            }
            inviting.set(false);
        });
    };

    rsx! {
        document::Title { "成员权限 — SoulBook" }
        div { class: "page-content",

            // ── Breadcrumb + actions header ─────────────────────────────
            div { style: "display:flex;align-items:center;justify-content:space-between;margin-bottom:20px;",
                div { style: "display:flex;align-items:center;gap:6px;font-size:13px;color:var(--muted);",
                    span { "SoulBook" }
                    span { "»" }
                    span { style: "color:var(--text);font-weight:500;", "成员权限" }
                }
                div { style: "display:flex;align-items:center;gap:10px;",
                    button {
                        style: "padding:7px 14px;border-radius:7px;border:1px solid var(--line);background:var(--panel2);font-size:13px;cursor:pointer;color:var(--text);",
                        "管理角色"
                    }
                    button {
                        style: "padding:7px 16px;border-radius:7px;border:none;background:#3b82f6;color:#fff;font-size:13px;font-weight:500;cursor:pointer;",
                        onclick: move |_| show_invite.set(true),
                        "+ 邀请成员"
                    }
                }
            }

            // ── Page title + subtitle ───────────────────────────────────
            div { style: "margin-bottom:24px;",
                h1 { style: "font-size:22px;font-weight:700;margin-bottom:6px;display:flex;align-items:center;gap:8px;",
                    span { "👥" }
                    span { "成员与权限管理" }
                }
                p { style: "font-size:13.5px;color:var(--muted);line-height:1.6;",
                    "管理组织成员与空间成员、角色分配与细粒度权限配置，用户分为人类与 AI 两种类型，地位平等"
                }
            }

            // ── Two info cards (org + space) ────────────────────────────
            {
                let space_name = selected_space_name.read().clone();
                let member_count = match &*members_res.read() {
                    Some(Ok(m)) => m.len(),
                    _ => 0,
                };
                rsx! {
                    div { style: "display:grid;grid-template-columns:1fr 1fr;gap:16px;margin-bottom:24px;",
                        // Org card
                        div { style: "padding:18px 20px;border-radius:10px;border:1px solid var(--line);background:var(--panel2);",
                            div { style: "display:flex;align-items:center;justify-content:space-between;margin-bottom:10px;",
                                span { style: "font-size:13px;font-weight:600;color:var(--muted);text-transform:uppercase;letter-spacing:.04em;", "所属组织" }
                            }
                            p { style: "font-size:13px;color:var(--text);line-height:1.65;",
                                "组织成员先进入统一目录，再分配到具体空间。"
                            }
                        }
                        // Space card
                        div { style: "padding:18px 20px;border-radius:10px;border:1px solid var(--line);background:var(--panel2);",
                            div { style: "display:flex;align-items:center;justify-content:space-between;margin-bottom:10px;",
                                span { style: "font-size:13px;font-weight:600;color:var(--muted);text-transform:uppercase;letter-spacing:.04em;", "当前空间" }
                            }
                            p { style: "font-size:13px;color:var(--text);line-height:1.65;margin-bottom:14px;",
                                if space_name.is_empty() {
                                    "请在下方选择一个空间以查看成员。"
                                } else {
                                    "{space_name} 的成员是组织成员的协作子集，角色可在空间级进一步收敛。"
                                }
                            }
                            div { style: "display:flex;flex-wrap:wrap;gap:8px;",
                                span { style: "padding:3px 10px;border-radius:20px;font-size:12px;background:#dbeafe;color:#1d4ed8;border:1px solid #bfdbfe;", "空间成员 {member_count}" }
                            }
                        }
                    }
                }
            }

            // ── Invite modal ────────────────────────────────────────────
            if show_invite() {
                div {
                    style: "position:fixed;inset:0;background:rgba(0,0,0,.45);z-index:200;display:flex;align-items:center;justify-content:center;",
                    onclick: move |_| show_invite.set(false),
                    div {
                        style: "width:440px;background:var(--panel2);border-radius:12px;padding:28px;box-shadow:0 20px 60px rgba(0,0,0,.25);",
                        onclick: move |e| e.stop_propagation(),
                        h3 { style: "font-size:16px;font-weight:700;margin-bottom:20px;", "邀请成员" }
                        div { style: "margin-bottom:14px;",
                            label { style: "display:block;font-size:13px;font-weight:500;margin-bottom:6px;", "邮箱地址" }
                            input {
                                style: "width:100%;padding:9px 12px;border-radius:7px;border:1px solid var(--line);background:var(--panel);font-size:13px;box-sizing:border-box;",
                                r#type: "email",
                                placeholder: "user@example.com",
                                value: "{invite_email}",
                                oninput: move |e| invite_email.set(e.value())
                            }
                        }
                        div { style: "margin-bottom:14px;",
                            label { style: "display:block;font-size:13px;font-weight:500;margin-bottom:6px;", "角色" }
                            select {
                                style: "width:100%;padding:9px 12px;border-radius:7px;border:1px solid var(--line);background:var(--panel);font-size:13px;",
                                value: "{invite_role}",
                                onchange: move |e| invite_role.set(e.value()),
                                option { value: "owner", "Owner — 所有者" }
                                option { value: "admin", "Admin — 管理员" }
                                option { value: "editor", "Editor — 编辑者" }
                                option { value: "member", "Member — 普通成员" }
                            }
                        }
                        if !invite_err().is_empty() {
                            p { style: "color:#dc2626;font-size:13px;margin-bottom:10px;", "{invite_err}" }
                        }
                        div { style: "display:flex;gap:10px;justify-content:flex-end;margin-top:8px;",
                            button {
                                style: "padding:8px 16px;border-radius:7px;border:1px solid var(--line);background:var(--panel2);font-size:13px;cursor:pointer;",
                                onclick: move |_| show_invite.set(false),
                                "取消"
                            }
                            button {
                                style: "padding:8px 16px;border-radius:7px;border:none;background:#3b82f6;color:#fff;font-size:13px;font-weight:500;cursor:pointer;",
                                disabled: inviting(),
                                onclick: do_invite,
                                if inviting() { "发送中…" } else { "发送邀请" }
                            }
                        }
                    }
                }
            }

            // ── Space selector ──────────────────────────────────────────
            match &*spaces_res.read() {
                None => rsx! { div { style: "color:var(--muted);padding:12px 0;font-size:13px;", "加载空间…" } },
                Some(Err(e)) => rsx! { div { style: "color:#dc2626;padding:12px 0;font-size:13px;", "加载空间失败：{e}" } },
                Some(Ok(data)) => {
                    let spaces = data.spaces.as_ref().or(data.items.as_ref()).cloned().unwrap_or_default();
                    rsx! {
                        div { style: "display:flex;flex-wrap:wrap;gap:8px;margin-bottom:20px;",
                            for space in spaces.iter() {
                                {
                                    let slug = space.slug.clone();
                                    let name = space.name.clone();
                                    let is_sel = *selected_slug.read() == slug;
                                    rsx! {
                                        button {
                                            style: if is_sel {
                                                "padding:6px 14px;border-radius:20px;font-size:13px;border:1.5px solid #3b82f6;background:#eff6ff;color:#1d4ed8;cursor:pointer;font-weight:500;"
                                            } else {
                                                "padding:6px 14px;border-radius:20px;font-size:13px;border:1px solid var(--line);background:var(--panel2);color:var(--text);cursor:pointer;"
                                            },
                                            onclick: move |_| {
                                                selected_slug.set(slug.clone());
                                                selected_space_name.set(name.clone());
                                            },
                                            "{name}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ── Tab card ────────────────────────────────────────────────
            div { style: "background:var(--panel2);border:1px solid var(--line);border-radius:12px;overflow:hidden;",

                // Tab bar
                div { style: "display:flex;border-bottom:1px solid var(--line);padding:0 4px;",
                    // Hidden dispatchers
                    div { style: "display:none;",
                        button { id: "mtab-members", onclick: move |_| active_tab.set("members".to_string()) }
                        button { id: "mtab-roles", onclick: move |_| active_tab.set("roles".to_string()) }
                        button { id: "mtab-permissions", onclick: move |_| active_tab.set("permissions".to_string()) }
                        button { id: "mtab-invitations", onclick: move |_| active_tab.set("invitations".to_string()) }
                    }
                    {
                        let count = match &*members_res.read() {
                            Some(Ok(m)) => m.len(),
                            _ => 0,
                        };
                        let label = format!("成员列表 ({})", count);
                        member_tab("members", &label, active_tab.read().as_str())
                    }
                    {member_tab("roles", "角色管理", active_tab.read().as_str())}
                    {member_tab("permissions", "权限矩阵", active_tab.read().as_str())}
                    {member_tab("invitations", "邀请记录", active_tab.read().as_str())}
                }

                // Tab content
                match active_tab.read().as_str() {
                    "roles" => rsx! {
                        div { style: "padding:24px;",
                            div { style: "display:grid;grid-template-columns:repeat(auto-fill,minmax(220px,1fr));gap:16px;",
                                {role_card("Owner", "所有者", "#ef4444", "完全控制空间，可转让所有权")}
                                {role_card("Admin", "管理员", "#3b82f6", "可管理空间所有内容和成员")}
                                {role_card("Editor", "编辑者", "#10b981", "可创建、编辑、发布文档")}
                                {role_card("Member", "普通成员", "#8b5cf6", "可查看和评论文档")}
                            }
                        }
                    },
                    "permissions" => rsx! {
                        div { style: "padding:24px;overflow-x:auto;",
                            table { style: "width:100%;border-collapse:collapse;min-width:520px;",
                                thead {
                                    tr { style: "background:var(--panel);",
                                        th { style: "padding:10px 14px;text-align:left;font-size:12px;color:var(--muted);font-weight:600;border-bottom:1px solid var(--line);", "权限" }
                                        th { style: "padding:10px 14px;text-align:center;font-size:12px;color:#ef4444;font-weight:600;border-bottom:1px solid var(--line);", "Owner" }
                                        th { style: "padding:10px 14px;text-align:center;font-size:12px;color:#3b82f6;font-weight:600;border-bottom:1px solid var(--line);", "Admin" }
                                        th { style: "padding:10px 14px;text-align:center;font-size:12px;color:#10b981;font-weight:600;border-bottom:1px solid var(--line);", "Editor" }
                                        th { style: "padding:10px 14px;text-align:center;font-size:12px;color:#8b5cf6;font-weight:600;border-bottom:1px solid var(--line);", "Member" }
                                    }
                                }
                                tbody {
                                    {perm_row("查看文档", true, true, true, true)}
                                    {perm_row("创建文档", true, true, true, false)}
                                    {perm_row("编辑文档", true, true, true, false)}
                                    {perm_row("删除文档", true, true, false, false)}
                                    {perm_row("发布文档", true, true, true, false)}
                                    {perm_row("管理成员", true, true, false, false)}
                                    {perm_row("修改设置", true, false, false, false)}
                                    {perm_row("转让空间", true, false, false, false)}
                                }
                            }
                        }
                    },
                    "invitations" => rsx! {
                        div { style: "padding:60px;text-align:center;color:var(--muted);",
                            div { style: "font-size:40px;margin-bottom:12px;", "📨" }
                            p { style: "font-size:13px;", "暂无待处理的邀请记录" }
                        }
                    },
                    _ => rsx! {
                        // Members list tab
                        div {
                            // Filter toolbar
                            div { style: "padding:16px 20px;border-bottom:1px solid var(--line);display:flex;align-items:center;gap:10px;flex-wrap:wrap;",
                                input {
                                    style: "flex:1;min-width:180px;padding:7px 12px;border-radius:7px;border:1px solid var(--line);background:var(--panel);font-size:13px;",
                                    placeholder: "搜索成员...",
                                    value: "{search_query}",
                                    oninput: move |e| search_query.set(e.value())
                                }
                                select {
                                    style: "padding:7px 10px;border-radius:7px;border:1px solid var(--line);background:var(--panel);font-size:13px;",
                                    option { "当前空间" }
                                    option { "全部空间" }
                                }
                                select {
                                    style: "padding:7px 10px;border-radius:7px;border:1px solid var(--line);background:var(--panel);font-size:13px;",
                                    option { "全部角色" }
                                    option { "Owner" }
                                    option { "Admin" }
                                    option { "Editor" }
                                    option { "Member" }
                                }
                                select {
                                    style: "padding:7px 10px;border-radius:7px;border:1px solid var(--line);background:var(--panel);font-size:13px;",
                                    option { "全部类型" }
                                    option { "人类" }
                                    option { "AI" }
                                }
                            }

                            // Hint text
                            div { style: "padding:10px 20px;background:var(--panel);border-bottom:1px solid var(--line);",
                                p { style: "font-size:12.5px;color:var(--muted);line-height:1.55;",
                                    "组织成员是候选池，空间成员是实际协作集。AI 与人类共享角色模板，只在审计标识与自动执行边界上区分。"
                                }
                            }

                            // Table header
                            div { style: "display:grid;grid-template-columns:1fr 140px 120px 130px 80px 160px;gap:0;padding:10px 20px;background:var(--panel);border-bottom:1px solid var(--line);",
                                span { style: "font-size:11.5px;font-weight:600;color:var(--muted);text-transform:uppercase;letter-spacing:.04em;", "成员" }
                                span { style: "font-size:11.5px;font-weight:600;color:var(--muted);text-transform:uppercase;letter-spacing:.04em;", "角色" }
                                span { style: "font-size:11.5px;font-weight:600;color:var(--muted);text-transform:uppercase;letter-spacing:.04em;", "加入时间" }
                                span { style: "font-size:11.5px;font-weight:600;color:var(--muted);text-transform:uppercase;letter-spacing:.04em;", "最近活动" }
                                span { style: "font-size:11.5px;font-weight:600;color:var(--muted);text-transform:uppercase;letter-spacing:.04em;", "状态" }
                                span { style: "font-size:11.5px;font-weight:600;color:var(--muted);text-transform:uppercase;letter-spacing:.04em;", "操作" }
                            }

                            // Member rows – real API data only
                            match &*members_res.read() {
                                None => rsx! {
                                    div { style: "padding:40px;text-align:center;color:var(--muted);font-size:13px;", "加载中…" }
                                },
                                Some(Err(e)) => rsx! {
                                    div { style: "padding:40px;text-align:center;color:#dc2626;font-size:13px;", "加载失败：{e}" }
                                },
                                Some(Ok(members)) if members.is_empty() => rsx! {
                                    div { style: "padding:40px;text-align:center;color:var(--muted);font-size:13px;", "该空间暂无成员，点击右上角「+ 邀请成员」添加" }
                                },
                                Some(Ok(members)) => rsx! {
                                    div {
                                        for m in members.iter() {
                                            {
                                                let display = m.username.as_deref().unwrap_or(m.email.as_deref().unwrap_or("?"));
                                                let email = m.email.as_deref().unwrap_or("-");
                                                let role = m.role.as_deref().unwrap_or("member");
                                                let joined = m.joined_at.as_deref().unwrap_or("-");
                                                let initial = display.chars().next().unwrap_or('?').to_uppercase().to_string();
                                                let rc = role_color(role);
                                                rsx! {
                                                    div {
                                                        style: "display:grid;grid-template-columns:1fr 140px 120px 130px 80px 160px;gap:0;padding:14px 20px;border-bottom:1px solid var(--line);align-items:center;",
                                                        div { style: "display:flex;align-items:center;gap:10px;",
                                                            div {
                                                                style: "width:34px;height:34px;border-radius:50%;background:#3b82f622;color:#3b82f6;display:flex;align-items:center;justify-content:center;font-size:13px;font-weight:600;flex-shrink:0;",
                                                                "{initial}"
                                                            }
                                                            div {
                                                                p { style: "font-size:13.5px;font-weight:500;line-height:1.25;margin-bottom:2px;", "{display}" }
                                                                p { style: "font-size:12px;color:var(--muted);", "{email}" }
                                                            }
                                                        }
                                                        span {
                                                            style: "padding:3px 10px;border-radius:20px;font-size:12px;font-weight:500;background:{rc}22;color:{rc};border:1px solid {rc}44;display:inline-block;",
                                                            "{role}"
                                                        }
                                                        span { style: "font-size:12.5px;color:var(--muted);", "{joined}" }
                                                        span { style: "font-size:12.5px;color:var(--muted);", "–" }
                                                        div {
                                                            span {
                                                                style: "padding:3px 10px;border-radius:20px;font-size:12px;font-weight:500;background:#d1fae522;color:#059669;border:1px solid #059669;",
                                                                "活跃"
                                                            }
                                                        }
                                                        div { style: "display:flex;align-items:center;gap:6px;",
                                                            button {
                                                                style: "padding:4px 10px;border-radius:6px;border:1px solid var(--line);background:var(--panel);font-size:12px;cursor:pointer;",
                                                                "调整角色"
                                                            }
                                                            button {
                                                                style: "padding:4px 10px;border-radius:6px;border:1px solid #fecaca;background:#fff1f2;color:#dc2626;font-size:12px;cursor:pointer;",
                                                                "移除"
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
                }
            }
        }
    }
}

// ── Tab helper ──────────────────────────────────────────────────────────────

fn member_tab(key: &str, label: &str, active: &str) -> Element {
    let is_active = key == active;
    rsx! {
        span {
            style: if is_active {
                "padding:12px 18px;font-size:13px;font-weight:600;color:#3b82f6;border-bottom:2px solid #3b82f6;cursor:pointer;white-space:nowrap;display:inline-block;user-select:none;"
            } else {
                "padding:12px 18px;font-size:13px;color:var(--muted);border-bottom:2px solid transparent;cursor:pointer;white-space:nowrap;display:inline-block;user-select:none;"
            },
            onclick: {
                let k = key.to_string();
                move |_| {
                    let js = format!("document.getElementById('mtab-{}').click();", k);
                    let _ = document::eval(&js);
                }
            },
            "{label}"
        }
    }
}

// ── Role card ──────────────────────────────────────────────────────────────

fn role_card(key: &str, name: &str, color: &str, desc: &str) -> Element {
    rsx! {
        div { style: "padding:18px;border-radius:10px;border:1px solid var(--line);background:var(--panel);",
            div { style: "display:flex;align-items:center;gap:8px;margin-bottom:8px;",
                span { style: "width:10px;height:10px;border-radius:50%;background:{color};display:inline-block;flex-shrink:0;" }
                span { style: "font-size:14px;font-weight:600;", "{name}" }
                span { style: "font-size:11px;color:var(--muted);", "({key})" }
            }
            p { style: "font-size:12.5px;color:var(--muted);line-height:1.5;", "{desc}" }
        }
    }
}

// ── Permission matrix helpers ──────────────────────────────────────────────

fn perm_row(perm: &str, owner: bool, admin: bool, editor: bool, member: bool) -> Element {
    rsx! {
        tr { style: "border-bottom:1px solid var(--line);",
            td { style: "padding:10px 14px;font-size:13px;", "{perm}" }
            td { style: "padding:10px 14px;text-align:center;", {perm_check(owner)} }
            td { style: "padding:10px 14px;text-align:center;", {perm_check(admin)} }
            td { style: "padding:10px 14px;text-align:center;", {perm_check(editor)} }
            td { style: "padding:10px 14px;text-align:center;", {perm_check(member)} }
        }
    }
}

fn perm_check(has: bool) -> Element {
    if has {
        rsx! { span { style: "color:#10b981;font-size:16px;font-weight:700;", "✓" } }
    } else {
        rsx! { span { style: "color:var(--line);font-size:16px;", "—" } }
    }
}
