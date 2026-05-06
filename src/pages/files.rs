use crate::api::files as files_api;
use crate::api::spaces as spaces_api;
use crate::state::AuthState;
use dioxus::prelude::*;

// ── Component ────────────────────────────────────────────────────────────────

#[component]
pub fn Files() -> Element {
    let auth = use_context::<Signal<AuthState>>();
    let spaces_res = use_resource(|| async move { spaces_api::list_spaces(1, 50).await });
    let mut selected_id = use_signal(|| String::new());
    let mut selected_name = use_signal(|| String::new());
    let mut refresh = use_signal(|| 0u32);
    let mut uploading = use_signal(|| false);
    let mut upload_msg = use_signal(|| String::new());
    let mut active_tab = use_signal(|| "all".to_string());
    let mut active_nav = use_signal(|| "all".to_string()); // left sidebar nav
    let mut grid_view = use_signal(|| true);
    let mut selected_files: Signal<Vec<String>> = use_signal(|| vec![]);

    use_effect(move || {
        if selected_id.read().is_empty() {
            if let Some(Ok(data)) = &*spaces_res.read() {
                if let Some(first) = data
                    .spaces
                    .as_ref()
                    .or(data.items.as_ref())
                    .and_then(|s| s.first())
                {
                    selected_id.set(first.id.clone().unwrap_or_else(|| first.slug.clone()));
                    selected_name.set(first.name.clone());
                }
            }
        }
    });

    let files_res = use_resource(move || {
        let id = selected_id.read().clone();
        let _r = *refresh.read();
        async move {
            if id.is_empty() {
                return Ok(vec![]);
            }
            files_api::list_files(&id).await
        }
    });

    use_effect(move || {
        let Some(Ok(files)) = &*files_res.read() else {
            return;
        };

        let token = crate::api::get_token().unwrap_or_default();
        if token.is_empty() {
            return;
        }

        let current_user_id = auth.read().user.as_ref().map(|u| u.id.clone());
        let active_filter = if active_nav.read().as_str() == "all" {
            active_tab.read().clone()
        } else {
            active_nav.read().clone()
        };
        let previews: Vec<_> = filter_files_for_view(files, &active_filter, current_user_id.as_deref())
            .into_iter()
            .filter(|file| file.file_type == "image")
            .map(|file| {
                serde_json::json!({
                    "element_id": file_preview_element_id(&file.id),
                    "url": file.url,
                })
            })
            .collect();

        if previews.is_empty() {
            return;
        }

        let Ok(token_json) = serde_json::to_string(&token) else {
            return;
        };
        let Ok(previews_json) = serde_json::to_string(&previews) else {
            return;
        };
        let js = format!(
            r#"(async function(){{
  const token = {token_json};
  const previews = {previews_json};
  for (const item of previews) {{
    const el = document.getElementById(item.element_id);
    if (!el || el.dataset.loadedSrc === item.url) continue;
    try {{
      const resp = await fetch(item.url, {{ headers: {{ Authorization: 'Bearer ' + token }} }});
      if (!resp.ok) continue;
      const blob = await resp.blob();
      const oldUrl = el.dataset.blobUrl;
      const blobUrl = URL.createObjectURL(blob);
      el.src = blobUrl;
      el.dataset.loadedSrc = item.url;
      el.dataset.blobUrl = blobUrl;
      if (oldUrl) URL.revokeObjectURL(oldUrl);
    }} catch (_) {{}}
  }}
}})();"#
        );
        let _ = document::eval(&js);
    });

    let mut deleting = use_signal(|| String::new());

    let do_upload = move |_| {
        let space_id = selected_id.read().clone();
        if space_id.is_empty() {
            return;
        }
        let token = crate::api::get_token().unwrap_or_default();
        uploading.set(true);
        upload_msg.set(String::new());
        let js = format!(
            r#"(function(){{
  var inp = document.createElement('input');
  inp.type = 'file';
  inp.multiple = true;
  inp.style.display = 'none';
  document.body.appendChild(inp);
  inp.onchange = async function() {{
    var files = inp.files;
    if (!files || !files.length) {{ dioxus.send('cancel'); document.body.removeChild(inp); return; }}
    var ok = 0, fail = 0;
    for (var i = 0; i < files.length; i++) {{
      var form = new FormData();
      form.append('space_id', '{space_id}');
      form.append('file', files[i]);
      try {{
        var r = await fetch('/api/docs/files', {{
          method: 'POST',
          headers: {{'Authorization': 'Bearer {token}'}},
          body: form
        }});
        if (r.ok) ok++; else fail++;
      }} catch(e) {{ fail++; }}
    }}
    document.body.removeChild(inp);
    dioxus.send('done:'+ok+':'+fail);
  }};
  inp.addEventListener('cancel', function() {{ dioxus.send('cancel'); document.body.removeChild(inp); }});
  inp.click();
}})();"#
        );
        spawn(async move {
            let mut eval = document::eval(&js);
            match eval.recv::<String>().await {
                Ok(msg) if msg.starts_with("done:") => {
                    let parts: Vec<&str> = msg.splitn(3, ':').collect();
                    let ok = parts
                        .get(1)
                        .and_then(|s| s.parse::<u32>().ok())
                        .unwrap_or(0);
                    let fail = parts
                        .get(2)
                        .and_then(|s| s.parse::<u32>().ok())
                        .unwrap_or(0);
                    upload_msg.set(if fail == 0 {
                        format!("成功上传 {} 个文件", ok)
                    } else {
                        format!("上传完成：{} 成功，{} 失败", ok, fail)
                    });
                    let cur = *refresh.read();
                    refresh.set(cur + 1);
                }
                _ => {}
            }
            uploading.set(false);
        });
    };

    rsx! {
        document::Title { "文件管理 — SoulBook" }

        // ── Outer shell: left sidebar + main ────────────────────────────────
        div {
            style: "display:flex;height:100vh;overflow:hidden;background:var(--bg);",

            // ── Left sidebar (240 px fixed) ──────────────────────────────────
            div {
                style: "width:240px;flex-shrink:0;background:var(--panel);border-right:1px solid var(--line);display:flex;flex-direction:column;overflow-y:auto;",

                // Quick Access
                div { style: "padding:20px 12px 8px;",
                    p { style: "font-size:10.5px;font-weight:700;color:var(--muted2);text-transform:uppercase;letter-spacing:.1em;margin-bottom:8px;padding:0 8px;", "快速访问" }
                    {sidebar_nav_item("all",      "📁", "全部文件",  active_nav.read().as_str(), active_nav, active_tab)}
                    {sidebar_nav_item("recent",   "🕐", "最近上传",  active_nav.read().as_str(), active_nav, active_tab)}
                    {sidebar_nav_item("mine",     "👤", "我上传的",  active_nav.read().as_str(), active_nav, active_tab)}
                    {sidebar_nav_item("ref",      "🔗", "被引用的",  active_nav.read().as_str(), active_nav, active_tab)}
                }

                // File types
                div { style: "padding:8px 12px;",
                    p { style: "font-size:10.5px;font-weight:700;color:var(--muted2);text-transform:uppercase;letter-spacing:.1em;margin-bottom:8px;padding:0 8px;", "文件类型" }
                    {sidebar_nav_item("image",    "🖼️", "图片",     active_nav.read().as_str(), active_nav, active_tab)}
                    {sidebar_nav_item("doc",      "📄", "文档",     active_nav.read().as_str(), active_nav, active_tab)}
                    {sidebar_nav_item("video",    "🎬", "视频",     active_nav.read().as_str(), active_nav, active_tab)}
                    {sidebar_nav_item("code",     "💻", "代码",     active_nav.read().as_str(), active_nav, active_tab)}
                    {sidebar_nav_item("archive",  "📦", "压缩包",   active_nav.read().as_str(), active_nav, active_tab)}
                }

                // Spaces list
                div { style: "padding:8px 12px;",
                    p { style: "font-size:10.5px;font-weight:700;color:var(--muted2);text-transform:uppercase;letter-spacing:.1em;margin-bottom:8px;padding:0 8px;", "空间" }
                    match &*spaces_res.read() {
                        None => rsx! { p { style: "padding:8px;font-size:12px;color:var(--muted);", "加载中…" } },
                        Some(Err(_)) => rsx! { p { style: "padding:8px;font-size:12px;color:#dc2626;", "加载失败" } },
                        Some(Ok(data)) => {
                            let spaces = data.spaces.as_ref().or(data.items.as_ref()).cloned().unwrap_or_default();
                            rsx! {
                                for space in spaces.iter() {
                                    {
                                        let id   = space.id.clone().unwrap_or_else(|| space.slug.clone());
                                        let name = space.name.clone();
                                        let is_sel = *selected_id.read() == id;
                                        rsx! {
                                            button {
                                                style: if is_sel {
                                                    "display:flex;align-items:center;gap:8px;width:100%;padding:7px 10px;border-radius:8px;background:var(--primary-light);color:var(--primary);border:none;cursor:pointer;text-align:left;font-size:13px;font-weight:500;margin-bottom:2px;"
                                                } else {
                                                    "display:flex;align-items:center;gap:8px;width:100%;padding:7px 10px;border-radius:8px;background:transparent;color:var(--text3);border:none;cursor:pointer;text-align:left;font-size:13px;margin-bottom:2px;"
                                                },
                                                onclick: move |_| {
                                                    selected_id.set(id.clone());
                                                    selected_name.set(name.clone());
                                                    upload_msg.set(String::new());
                                                    active_nav.set("all".to_string());
                                                    active_tab.set("all".to_string());
                                                },
                                                span { style: "font-size:14px;opacity:.7;", "□" }
                                                span { style: "overflow:hidden;text-overflow:ellipsis;white-space:nowrap;", "{name}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

            }

            // ── Main content area ────────────────────────────────────────────
            div { style: "flex:1;display:flex;flex-direction:column;overflow:hidden;",

                // Top bar row
                div { style: "display:flex;align-items:center;justify-content:flex-end;gap:10px;padding:12px 24px;border-bottom:1px solid var(--line);background:var(--panel);",
                    button {
                        style: "display:flex;align-items:center;gap:6px;padding:7px 14px;border-radius:8px;border:1px solid var(--line);background:var(--panel2);font-size:13px;color:var(--text2);cursor:pointer;",
                        onclick: move |_| { let v = *grid_view.read(); grid_view.set(!v); },
                        span { "□" }
                        span { if *grid_view.read() { "网格视图" } else { "列表视图" } }
                    }
                    if !upload_msg().is_empty() {
                        span { style: "font-size:12px;color:var(--muted);", "{upload_msg}" }
                    }
                    button {
                        class: "btn btn-primary",
                        style: "padding:7px 16px;font-size:13px;",
                        disabled: uploading() || selected_id.read().is_empty(),
                        onclick: do_upload,
                        span { "↑" }
                        if uploading() { "上传中…" } else { "上传文件" }
                    }
                    // User avatar
                    div { style: "width:34px;height:34px;border-radius:50%;background:var(--primary);color:#fff;display:flex;align-items:center;justify-content:center;font-size:13px;font-weight:700;flex-shrink:0;", "U" }
                }

                // Breadcrumb
                div { style: "padding:12px 24px 0;",
                    div { style: "display:flex;align-items:center;gap:6px;font-size:13px;color:var(--muted);",
                        span { "文件" }
                        span { "›" }
                        span { style: "color:var(--text2);font-weight:500;", "{selected_name.read()}" }
                        span { "›" }
                        span { style: "color:var(--text2);font-weight:500;", "{active_nav_label(active_nav.read().as_str())}" }
                    }
                }

                // Tabs + search + sort row
                div { style: "display:flex;align-items:center;gap:8px;padding:10px 24px;border-bottom:1px solid var(--line);flex-wrap:wrap;",
                    // Pill tabs
                    div { style: "display:flex;align-items:center;gap:4px;",
                        {pill_tab("all",   "全部",   active_tab.read().as_str())}
                        {pill_tab_icon("image", "■ 图片", active_tab.read().as_str())}
                        {pill_tab_icon("doc",   "■ 文档", active_tab.read().as_str())}
                        {pill_tab_icon("video", "■ 视频", active_tab.read().as_str())}
                        {pill_tab("other", "其他",   active_tab.read().as_str())}
                    }
                    // Hidden clickable buttons wired to signals
                    div { style: "display:none;",
                        button { onclick: move |_| active_tab.set("all".to_string()),   id: "ftab-all" }
                        button { onclick: move |_| active_tab.set("image".to_string()), id: "ftab-image" }
                        button { onclick: move |_| active_tab.set("doc".to_string()),   id: "ftab-doc" }
                        button { onclick: move |_| active_tab.set("video".to_string()), id: "ftab-video" }
                        button { onclick: move |_| active_tab.set("other".to_string()), id: "ftab-other" }
                    }
                    div { style: "margin-left:auto;display:flex;align-items:center;gap:8px;",
                        button { style: "display:flex;align-items:center;gap:5px;padding:6px 12px;border-radius:8px;border:1px solid var(--line);background:var(--panel2);font-size:12.5px;color:var(--text3);cursor:pointer;",
                            "最近上传 ▼"
                        }
                        div { style: "position:relative;",
                            span { style: "position:absolute;left:9px;top:50%;transform:translateY(-50%);color:var(--muted);font-size:13px;pointer-events:none;", "🔍" }
                            input {
                                style: "padding:6px 12px 6px 30px;border:1px solid var(--line);border-radius:8px;background:var(--panel2);font-size:12.5px;color:var(--text);width:180px;outline:none;",
                                placeholder: "搜索文件…",
                            }
                        }
                    }
                }

                // Batch operation bar
                div { style: "display:flex;align-items:center;gap:10px;padding:8px 24px;background:var(--panel3);border-bottom:1px solid var(--line);font-size:12.5px;",
                    span { style: "color:var(--muted);", "0 个文件已选" }
                    div { style: "margin-left:auto;display:flex;gap:6px;",
                        button { style: "display:flex;align-items:center;gap:4px;padding:5px 12px;border-radius:6px;border:1px solid var(--line);background:var(--panel);font-size:12px;color:var(--text2);cursor:pointer;",
                            "↓ 下载"
                        }
                        button { style: "display:flex;align-items:center;gap:4px;padding:5px 12px;border-radius:6px;border:1px solid var(--line);background:var(--panel);font-size:12px;color:#dc2626;cursor:pointer;",
                            "× 删除"
                        }
                        button { style: "padding:5px 12px;border-radius:6px;border:1px solid var(--line);background:var(--panel);font-size:12px;color:var(--muted);cursor:pointer;",
                            "取消选择"
                        }
                    }
                }

                // File grid area (scrollable)
                div { style: "flex:1;overflow-y:auto;padding:20px 24px;",
                    match &*files_res.read() {
                        None => rsx! {
                            div { style: "display:flex;align-items:center;justify-content:center;height:200px;color:var(--muted);font-size:14px;",
                                "加载中…"
                            }
                        },
                        Some(Err(e)) => rsx! {
                            div { style: "text-align:center;padding:60px;color:#dc2626;font-size:14px;",
                                "加载失败：{e}"
                            }
                        },
                        Some(Ok(files)) => {
                            let current_user_id = auth.read().user.as_ref().map(|u| u.id.clone());
                            let active_filter = if active_nav.read().as_str() == "all" {
                                active_tab.read().clone()
                            } else {
                                active_nav.read().clone()
                            };
                            let filtered = filter_files_for_view(files, &active_filter, current_user_id.as_deref());

                            if filtered.is_empty() {
                                rsx! {
                                    // Empty state / drop zone
                                    div {
                                        style: "display:flex;flex-direction:column;align-items:center;justify-content:center;border:2px dashed var(--line);border-radius:16px;padding:60px 20px;text-align:center;cursor:pointer;background:var(--panel2);",
                                        onclick: do_upload,
                                        div { style: "font-size:56px;margin-bottom:16px;line-height:1;", "☁" }
                                        p { style: "font-size:15px;font-weight:600;color:var(--text2);margin-bottom:8px;", "拖拽文件到这里，或点击上传" }
                                        p { style: "font-size:13px;color:var(--muted);margin-bottom:20px;", "支持 PNG、JPG、PDF、MP4、ZIP 等，单文件最大 100MB" }
                                        button {
                                            class: "btn btn-primary",
                                            style: "pointer-events:none;",
                                            "选择文件"
                                        }
                                    }
                                }
                            } else {
                                rsx! {
                                    div { style: "display:grid;grid-template-columns:repeat(4,1fr);gap:14px;",
                                        for f in filtered.iter() {
                                            {
                                                let fid   = f.id.clone();
                                                let furl  = f.url.clone();
                                                let fname = f.original_name.clone();
                                                let ftype = f.file_type.clone();
                                                let fsize = f.file_size;
                                                let is_image = ftype == "image";
                                                let preview_id = file_preview_element_id(&fid);
                                                rsx! {
                                                    div {
                                                        style: "position:relative;border:1px solid var(--line);border-radius:12px;overflow:hidden;background:var(--panel);display:flex;flex-direction:column;",
                                                        // Checkbox
                                                        div { style: "position:absolute;top:8px;right:8px;z-index:2;",
                                                            input {
                                                                r#type: "checkbox",
                                                                style: "width:16px;height:16px;cursor:pointer;accent-color:var(--primary);",
                                                                onchange: {
                                                                    let fid2 = fid.clone();
                                                                    move |_| {
                                                                        let mut sel = selected_files.write();
                                                                        if sel.contains(&fid2) { sel.retain(|x| x != &fid2); }
                                                                        else { sel.push(fid2.clone()); }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        // Preview / icon
                                                        div { style: "height:110px;background:var(--panel3);display:flex;align-items:center;justify-content:center;overflow:hidden;",
                                                            if is_image {
                                                                img { id: "{preview_id}", style: "width:100%;height:100%;object-fit:cover;" }
                                                            } else {
                                                                span { style: "font-size:42px;", "{file_icon(&ftype)}" }
                                                            }
                                                        }
                                                        // Info
                                                        div { style: "padding:10px;",
                                                            p { style: "font-size:12.5px;font-weight:500;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;margin-bottom:4px;color:var(--text2);", title: "{fname}", "{fname}" }
                                                            p { style: "font-size:11.5px;color:var(--muted);", "{format_size(fsize)}" }
                                                        }
                                                        // Actions
                                                        div { style: "display:flex;border-top:1px solid var(--line);",
                                                            button {
                                                                style: "flex:1;padding:7px;text-align:center;font-size:12px;color:var(--primary);text-decoration:none;border-right:1px solid var(--line);",
                                                                onclick: move |_| {
                                                                    let js = download_file_js(&furl, &fname);
                                                                    let _ = document::eval(&js);
                                                                },
                                                                "下载"
                                                            }
                                                            button {
                                                                style: "flex:1;padding:7px;font-size:12px;color:#dc2626;background:transparent;border:none;cursor:pointer;",
                                                                disabled: *deleting.read() == fid,
                                                                onclick: move |_| {
                                                                    let fid2 = fid.clone();
                                                                    deleting.set(fid2.clone());
                                                                    spawn(async move {
                                                                        let _ = files_api::delete_file(&fid2).await;
                                                                        deleting.set(String::new());
                                                                        let cur = *refresh.read();
                                                                        refresh.set(cur + 1);
                                                                    });
                                                                },
                                                                if *deleting.read() == fid { "删除中" } else { "删除" }
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

// ── Helper: left sidebar nav item ────────────────────────────────────────────

fn sidebar_nav_item(
    key: &'static str,
    icon: &'static str,
    label: &'static str,
    active: &str,
    mut active_nav: Signal<String>,
    mut active_tab: Signal<String>,
) -> Element {
    let is_active = key == active;
    let tab_key = match key {
        "image" | "doc" | "video" => key,
        _ => "all",
    };
    rsx! {
        button {
            style: if is_active {
                "display:flex;align-items:center;gap:8px;width:100%;padding:7px 10px;border-radius:8px;background:var(--primary-light);color:var(--primary);font-size:13px;font-weight:500;margin-bottom:2px;cursor:pointer;border:none;text-align:left;"
            } else {
                "display:flex;align-items:center;gap:8px;width:100%;padding:7px 10px;border-radius:8px;background:transparent;color:var(--text3);font-size:13px;margin-bottom:2px;cursor:pointer;border:none;text-align:left;"
            },
            onclick: move |_| {
                active_nav.set(key.to_string());
                active_tab.set(tab_key.to_string());
            },
            span { style: "font-size:15px;", "{icon}" }
            span { "{label}" }
        }
    }
}

// ── Helper: pill tab ──────────────────────────────────────────────────────────

fn pill_tab(key: &str, label: &str, active: &str) -> Element {
    let is_active = key == active;
    rsx! {
        span {
            style: if is_active {
                "padding:5px 14px;border-radius:20px;background:var(--primary);color:#fff;font-size:12.5px;font-weight:600;cursor:pointer;white-space:nowrap;"
            } else {
                "padding:5px 14px;border-radius:20px;background:var(--panel2);color:var(--muted);font-size:12.5px;cursor:pointer;white-space:nowrap;border:1px solid var(--line);"
            },
            onclick: {
                let k = key.to_string();
                move |_| {
                    let js = format!("document.getElementById('ftab-{}').click();", k);
                    let _ = document::eval(&js);
                }
            },
            "{label}"
        }
    }
}

fn pill_tab_icon(key: &str, label: &str, active: &str) -> Element {
    pill_tab(key, label, active)
}

// ── Pure helpers ──────────────────────────────────────────────────────────────

fn format_size(bytes: i64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

fn file_icon(file_type: &str) -> &'static str {
    match file_type {
        "pdf" => "📄",
        "video" => "🎬",
        "audio" => "🎵",
        "archive" => "📦",
        "spreadsheet" => "📊",
        "doc" | "docx" => "📝",
        "image" => "🖼️",
        _ => "📎",
    }
}

fn file_preview_element_id(file_id: &str) -> String {
    format!("file-preview-{}", dom_id_fragment(file_id))
}

fn dom_id_fragment(input: &str) -> String {
    input
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch
            } else {
                '-'
            }
        })
        .collect()
}

fn download_file_js(url: &str, filename: &str) -> String {
    let url_json = serde_json::to_string(url).unwrap_or_else(|_| "\"\"".to_string());
    let filename_json = serde_json::to_string(filename).unwrap_or_else(|_| "\"download\"".to_string());
    format!(
        r#"(async function(){{
  const token = localStorage.getItem('soulbook_token') || localStorage.getItem('souldoc_token') || localStorage.getItem('jwt_token') || localStorage.getItem('auth_token') || localStorage.getItem('token') || '';
  const resp = await fetch({url_json}, {{ headers: {{ Authorization: 'Bearer ' + token }} }});
  if (!resp.ok) {{
    alert('下载失败：HTTP ' + resp.status);
    return;
  }}
  const blob = await resp.blob();
  const blobUrl = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = blobUrl;
  a.download = {filename_json};
  document.body.appendChild(a);
  a.click();
  a.remove();
  setTimeout(function() {{ URL.revokeObjectURL(blobUrl); }}, 1000);
}})();"#
    )
}

fn active_nav_label(key: &str) -> &'static str {
    match key {
        "recent" => "最近上传",
        "mine" => "我上传的",
        "ref" => "被引用的",
        "image" => "图片",
        "doc" => "文档",
        "video" => "视频",
        "code" => "代码",
        "archive" => "压缩包",
        "other" => "其他",
        _ => "全部文件",
    }
}

fn is_document_file(file: &files_api::FileItem) -> bool {
    matches!(
        file.file_type.as_str(),
        "pdf" | "doc" | "docx" | "document" | "spreadsheet" | "presentation"
    ) || file.mime_type.contains("pdf")
        || file.mime_type.contains("word")
        || file.mime_type.contains("document")
        || file.mime_type.contains("spreadsheet")
        || file.mime_type.contains("presentation")
}

fn is_code_file(file: &files_api::FileItem) -> bool {
    matches!(
        file.mime_type.as_str(),
        "application/json" | "application/xml"
    ) || matches!(
        file.mime_type.as_str(),
        "text/html" | "text/css" | "text/javascript"
    )
}

fn filter_files_for_view(
    files: &[files_api::FileItem],
    filter: &str,
    current_user_id: Option<&str>,
) -> Vec<files_api::FileItem> {
    let mut filtered: Vec<_> = match filter {
        "image" => files
            .iter()
            .filter(|f| f.file_type == "image" || f.mime_type.starts_with("image/"))
            .cloned()
            .collect(),
        "doc" => files
            .iter()
            .filter(|f| is_document_file(f))
            .cloned()
            .collect(),
        "video" => files
            .iter()
            .filter(|f| f.file_type == "video" || f.mime_type.starts_with("video/"))
            .cloned()
            .collect(),
        "code" => files.iter().filter(|f| is_code_file(f)).cloned().collect(),
        "archive" => files
            .iter()
            .filter(|f| {
                f.file_type == "archive"
                    || f.mime_type.contains("zip")
                    || f.mime_type.contains("tar")
                    || f.mime_type.contains("gzip")
            })
            .cloned()
            .collect(),
        "mine" => files
            .iter()
            .filter(|f| current_user_id.is_some_and(|user_id| f.uploaded_by == user_id))
            .cloned()
            .collect(),
        "ref" => files
            .iter()
            .filter(|f| f.document_id.as_ref().is_some_and(|id| !id.is_empty()))
            .cloned()
            .collect(),
        "other" => files
            .iter()
            .filter(|f| {
                f.file_type != "image"
                    && f.file_type != "video"
                    && f.file_type != "archive"
                    && !is_document_file(f)
                    && !is_code_file(f)
            })
            .cloned()
            .collect(),
        _ => files.to_vec(),
    };

    if filter == "recent" {
        filtered.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    }

    filtered
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::files::FileItem;

    fn file(
        id: &str,
        file_type: &str,
        mime_type: &str,
        uploaded_by: &str,
        document_id: Option<&str>,
    ) -> FileItem {
        FileItem {
            id: id.to_string(),
            filename: format!("{id}.bin"),
            original_name: format!("{id}.bin"),
            file_size: 1,
            file_type: file_type.to_string(),
            mime_type: mime_type.to_string(),
            url: format!("/api/files/{id}/download"),
            thumbnail_url: None,
            space_id: Some("space:514".to_string()),
            document_id: document_id.map(ToString::to_string),
            uploaded_by: uploaded_by.to_string(),
            created_at: "2026-05-06T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn filter_files_for_view_matches_sidebar_categories() {
        let files = vec![
            file("img", "image", "image/png", "u1", None),
            file(
                "doc",
                "document",
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
                "u2",
                None,
            ),
            file("video", "video", "video/mp4", "u2", None),
            file("code", "other", "application/json", "u2", None),
            file(
                "zip",
                "archive",
                "application/zip",
                "u2",
                Some("document:d1"),
            ),
        ];

        assert_eq!(filter_files_for_view(&files, "image", Some("u1")).len(), 1);
        assert_eq!(filter_files_for_view(&files, "doc", Some("u1")).len(), 1);
        assert_eq!(filter_files_for_view(&files, "video", Some("u1")).len(), 1);
        assert_eq!(filter_files_for_view(&files, "code", Some("u1")).len(), 1);
        assert_eq!(
            filter_files_for_view(&files, "archive", Some("u1")).len(),
            1
        );
        assert_eq!(filter_files_for_view(&files, "mine", Some("u1")).len(), 1);
        assert_eq!(filter_files_for_view(&files, "ref", Some("u1")).len(), 1);
    }

    #[test]
    fn secure_file_helpers_generate_safe_dom_ids_and_fetch_downloads() {
        assert_eq!(
            file_preview_element_id("file_upload:spsx6o7y1n9g82vfsiuy"),
            "file-preview-file-upload-spsx6o7y1n9g82vfsiuy"
        );

        let js = download_file_js("/api/docs/files/file_upload:abc/download", "a\"b.jpg");
        assert!(js.contains("fetch(\"/api/docs/files/file_upload:abc/download\""));
        assert!(js.contains("Authorization: 'Bearer ' + token"));
        assert!(js.contains("a.download = \"a\\\"b.jpg\""));
    }
}
