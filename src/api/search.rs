use super::api_get;
use crate::models::SearchResult;
use serde::Deserialize;

/// Backend returns SearchResponse directly (no `data` wrapper)
#[derive(Deserialize)]
struct BackendSearchResponse {
    results: Vec<BackendSearchResult>,
    total_count: i64,
    page: i64,
    per_page: i64,
}

#[derive(Deserialize)]
struct BackendSearchResult {
    document_id: Option<String>,
    title: String,
    excerpt: Option<String>,
    tags: Option<Vec<String>>,
    score: Option<f64>,
}

#[derive(Deserialize)]
pub struct SearchData {
    pub results: Option<Vec<SearchResult>>,
    pub total: Option<u64>,
}

pub async fn search(query: &str, page: u32, per_page: u32) -> Result<SearchData, String> {
    let path = format!(
        "/api/docs/search?q={}&page={}&per_page={}",
        urlencoding(query),
        page,
        per_page
    );
    let resp: BackendSearchResponse = api_get(&path).await?;
    Ok(SearchData {
        total: Some(resp.total_count as u64),
        results: Some(
            resp.results
                .into_iter()
                .map(|r| SearchResult {
                    id: r.document_id.unwrap_or_default(),
                    title: r.title,
                    snippet: r.excerpt,
                    space_name: None,
                    space_slug: None,
                    doc_slug: None,
                    result_type: Some("document".to_string()),
                    tags: r.tags,
                    score: r.score,
                })
                .collect(),
        ),
    })
}

pub async fn suggest(query: &str) -> Result<Vec<String>, String> {
    let path = format!("/api/docs/search/suggest?q={}", urlencoding(query));
    // suggest endpoint returns suggestions array wrapped in data
    #[derive(Deserialize)]
    struct SuggestResp {
        suggestions: Vec<String>,
    }
    let resp: SuggestResp = api_get(&path).await?;
    Ok(resp.suggestions)
}

fn urlencoding(s: &str) -> String {
    js_sys::encode_uri_component(s).into()
}
