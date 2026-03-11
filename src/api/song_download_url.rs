use super::Query;
use crate::error::Result;
/// 歌曲下载链接
/// 对应 Node.js module/song_download_url.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌曲下载链接
    /// 对应 /song/download/url
    pub async fn song_download_url(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", "0"),
            "br": query.get_or("br", "999000").parse::<i64>().unwrap_or(999000)
        });
        self.request(
            "/api/song/enhance/download/url",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
