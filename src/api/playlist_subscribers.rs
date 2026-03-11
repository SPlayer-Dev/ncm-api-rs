use super::Query;
use crate::error::Result;
/// 歌单收藏者
/// 对应 Node.js module/playlist_subscribers.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌单收藏者
    /// 对应 /playlist/subscribers
    pub async fn playlist_subscribers(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", "0"),
            "limit": query.get_or("limit", "20").parse::<i64>().unwrap_or(20),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0)
        });
        self.request(
            "/api/playlist/subscribers",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
