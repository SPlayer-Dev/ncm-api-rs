use super::Query;
use crate::error::Result;
/// 歌曲播放链接
/// 对应 Node.js module/song_url.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌曲播放链接
    /// 对应 /song/url
    pub async fn song_url(&self, query: &Query) -> Result<ApiResponse> {
        let ids: Vec<String> = query
            .get_or("id", "")
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let br = query
            .get_or("br", "999000")
            .parse::<i64>()
            .unwrap_or(999000);
        let data = json!({
            "ids": serde_json::to_string(&ids).unwrap_or_default(),
            "br": br
        });
        self.request(
            "/api/song/enhance/player/url",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
