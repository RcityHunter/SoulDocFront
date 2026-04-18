use std::sync::LazyLock;

use dioxus::{document, prelude::*};

pub const GLOBAL_CSS: &str = include_str!("../prototype/style.css");
const NAV_JS: &str = include_str!("../prototype/nav.js");
const API_CONSOLE_JS: &str = include_str!("../prototype/api-console.js");

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PrototypeId {
    Index,
    Workspace,
    Spaces,
    Space,
    Docs,
    Templates,
    Editor,
    Versions,
    ChangeRequest,
    Members,
    Tags,
    Files,
    Notifications,
    Profile,
    Search,
    Language,
    AiTasks,
    AiTools,
    Seo,
    PublicDoc,
    GitSync,
    Developer,
    Organization,
    Settings,
    Install,
    Login,
    AiCenter,
}

pub struct PrototypePage {
    pub title: String,
    pub styles: String,
    pub body: String,
    pub scripts: String,
}

macro_rules! define_prototype_page {
    ($static_name:ident, $file:literal) => {
        static $static_name: LazyLock<PrototypePage> =
            LazyLock::new(|| parse_page(include_str!($file)));
    };
}

define_prototype_page!(INDEX_PAGE, "../prototype/index.html");
define_prototype_page!(WORKSPACE_PAGE, "../prototype/workspace.html");
define_prototype_page!(SPACES_PAGE, "../prototype/spaces.html");
define_prototype_page!(SPACE_PAGE, "../prototype/space.html");
define_prototype_page!(DOCS_PAGE, "../prototype/docs.html");
define_prototype_page!(TEMPLATES_PAGE, "../prototype/templates.html");
define_prototype_page!(EDITOR_PAGE, "../prototype/editor.html");
define_prototype_page!(VERSIONS_PAGE, "../prototype/versions.html");
define_prototype_page!(CHANGE_REQUEST_PAGE, "../prototype/change-request.html");
define_prototype_page!(MEMBERS_PAGE, "../prototype/members.html");
define_prototype_page!(TAGS_PAGE, "../prototype/tags.html");
define_prototype_page!(FILES_PAGE, "../prototype/files.html");
define_prototype_page!(NOTIFICATIONS_PAGE, "../prototype/notifications.html");
define_prototype_page!(PROFILE_PAGE, "../prototype/profile.html");
define_prototype_page!(SEARCH_PAGE, "../prototype/search.html");
define_prototype_page!(LANGUAGE_PAGE, "../prototype/language.html");
define_prototype_page!(AI_TASKS_PAGE, "../prototype/ai-tasks.html");
define_prototype_page!(AI_TOOLS_PAGE, "../prototype/ai-tools.html");
define_prototype_page!(SEO_PAGE, "../prototype/seo.html");
define_prototype_page!(PUBLIC_DOC_PAGE, "../prototype/public-doc.html");
define_prototype_page!(GIT_SYNC_PAGE, "../prototype/git-sync.html");
define_prototype_page!(DEVELOPER_PAGE, "../prototype/developer.html");
define_prototype_page!(ORGANIZATION_PAGE, "../prototype/organization.html");
define_prototype_page!(SETTINGS_PAGE, "../prototype/settings.html");
define_prototype_page!(INSTALL_PAGE, "../prototype/install.html");
define_prototype_page!(LOGIN_PAGE, "../prototype/login.html");
define_prototype_page!(AI_CENTER_PAGE, "../prototype/ai-center.html");

pub fn prototype_page(page: PrototypeId) -> &'static PrototypePage {
    match page {
        PrototypeId::Index => &INDEX_PAGE,
        PrototypeId::Workspace => &WORKSPACE_PAGE,
        PrototypeId::Spaces => &SPACES_PAGE,
        PrototypeId::Space => &SPACE_PAGE,
        PrototypeId::Docs => &DOCS_PAGE,
        PrototypeId::Templates => &TEMPLATES_PAGE,
        PrototypeId::Editor => &EDITOR_PAGE,
        PrototypeId::Versions => &VERSIONS_PAGE,
        PrototypeId::ChangeRequest => &CHANGE_REQUEST_PAGE,
        PrototypeId::Members => &MEMBERS_PAGE,
        PrototypeId::Tags => &TAGS_PAGE,
        PrototypeId::Files => &FILES_PAGE,
        PrototypeId::Notifications => &NOTIFICATIONS_PAGE,
        PrototypeId::Profile => &PROFILE_PAGE,
        PrototypeId::Search => &SEARCH_PAGE,
        PrototypeId::Language => &LANGUAGE_PAGE,
        PrototypeId::AiTasks => &AI_TASKS_PAGE,
        PrototypeId::AiTools => &AI_TOOLS_PAGE,
        PrototypeId::Seo => &SEO_PAGE,
        PrototypeId::PublicDoc => &PUBLIC_DOC_PAGE,
        PrototypeId::GitSync => &GIT_SYNC_PAGE,
        PrototypeId::Developer => &DEVELOPER_PAGE,
        PrototypeId::Organization => &ORGANIZATION_PAGE,
        PrototypeId::Settings => &SETTINGS_PAGE,
        PrototypeId::Install => &INSTALL_PAGE,
        PrototypeId::Login => &LOGIN_PAGE,
        PrototypeId::AiCenter => &AI_CENTER_PAGE,
    }
}

#[component]
pub fn PrototypeView(page: PrototypeId) -> Element {
    let definition = prototype_page(page);

    use_effect(use_reactive((&page,), move |_| {
        let _ = document::eval(NAV_JS);
        if matches!(page, PrototypeId::Developer) {
            let _ = document::eval(API_CONSOLE_JS);
        }
        if !definition.scripts.trim().is_empty() {
            let _ = document::eval(definition.scripts.as_str());
        }
    }));

    rsx! {
        document::Title { "{definition.title}" }
        if !definition.styles.trim().is_empty() {
            style { "{definition.styles}" }
        }
        div {
            class: "prototype-host",
            dangerous_inner_html: "{definition.body}"
        }
    }
}

fn parse_page(source: &str) -> PrototypePage {
    let head = extract_between(source, "<head>", "</head>").unwrap_or_default();
    let title = extract_between(&head, "<title>", "</title>")
        .map(str::trim)
        .unwrap_or("SoulDoc")
        .to_string();
    let styles = collect_tag_contents(&head, "style").join("\n\n");
    let body = extract_between(source, "<body>", "</body>")
        .map(str::to_string)
        .unwrap_or_default();
    let (body, scripts) = strip_scripts(&body);

    PrototypePage {
        title,
        styles,
        body: body.trim().to_string(),
        scripts: scripts.trim().to_string(),
    }
}

fn extract_between<'a>(source: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let start_index = source.find(start)? + start.len();
    let end_index = source[start_index..].find(end)? + start_index;
    Some(&source[start_index..end_index])
}

fn collect_tag_contents(source: &str, tag: &str) -> Vec<String> {
    let open = format!("<{tag}");
    let close = format!("</{tag}>");
    let mut results = Vec::new();
    let mut cursor = source;

    while let Some(open_index) = cursor.find(&open) {
        let after_open = &cursor[open_index..];
        let Some(tag_close_index) = after_open.find('>') else {
            break;
        };
        let content_start = open_index + tag_close_index + 1;
        let Some(close_index) = cursor[content_start..].find(&close) else {
            break;
        };
        let content_end = content_start + close_index;
        results.push(cursor[content_start..content_end].trim().to_string());
        cursor = &cursor[content_end + close.len()..];
    }

    results
}

fn strip_scripts(source: &str) -> (String, String) {
    let mut html = String::new();
    let mut scripts = String::new();
    let mut cursor = source;

    loop {
        let Some(script_start) = cursor.find("<script") else {
            html.push_str(cursor);
            break;
        };

        html.push_str(&cursor[..script_start]);
        let script_block = &cursor[script_start..];
        let Some(tag_end) = script_block.find('>') else {
            break;
        };
        let tag = &script_block[..=tag_end];
        let remaining = &script_block[tag_end + 1..];
        let Some(script_end) = remaining.find("</script>") else {
            break;
        };
        let content = remaining[..script_end].trim();

        if !tag.contains("src=") && !content.is_empty() {
            if !scripts.is_empty() {
                scripts.push_str("\n\n");
            }
            scripts.push_str(content);
        }

        cursor = &remaining[script_end + "</script>".len()..];
    }

    (html, scripts)
}
