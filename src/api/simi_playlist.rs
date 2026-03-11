use super::Query;
use crate::error::Result;
/// 相似歌单
/// 对应 Node.js module/simi_playlist.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 相似歌单
    /// 对应 /simi/playlist
    pub async fn simi_playlist(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "songid": query.get_or("id", "0"),
            "limit": query.get_or("limit", "50").parse::<i64>().unwrap_or(50),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0)
        });
        self.request(
            "/api/discovery/simiPlaylist",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
