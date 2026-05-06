use super::{api_delete, api_get};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct FileItem {
    pub id: String,
    pub filename: String,
    pub original_name: String,
    pub file_size: i64,
    pub file_type: String,
    pub mime_type: String,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub space_id: Option<String>,
    pub document_id: Option<String>,
    pub uploaded_by: String,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct FileListResponse {
    pub files: Vec<FileItem>,
    pub total_count: i64,
}

pub async fn list_files(space_id: &str) -> Result<Vec<FileItem>, String> {
    let path = format!("/api/docs/files?space_id={}&per_page=50", space_id);
    let resp: FileListResponse = api_get(&path).await?;
    Ok(resp
        .files
        .into_iter()
        .map(normalize_file_item_urls)
        .collect())
}

pub async fn delete_file(file_id: &str) -> Result<(), String> {
    let path = format!("/api/docs/files/{}", file_id);
    api_delete(&path).await
}

fn normalize_file_item_urls(mut file: FileItem) -> FileItem {
    file.url = normalize_file_url(&file.url);
    file.thumbnail_url = file.thumbnail_url.map(|url| normalize_file_url(&url));
    file
}

fn normalize_file_url(url: &str) -> String {
    if let Some(rest) = url.strip_prefix("/api/files/") {
        format!("/api/docs/files/{}", rest)
    } else {
        url.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_file_url_uses_docs_api_mount() {
        assert_eq!(
            normalize_file_url("/api/files/file_upload:abc/download"),
            "/api/docs/files/file_upload:abc/download"
        );
        assert_eq!(
            normalize_file_url("/api/docs/files/file_upload:abc/download"),
            "/api/docs/files/file_upload:abc/download"
        );
    }
}
