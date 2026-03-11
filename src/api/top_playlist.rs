use super::Query;
use crate::error::Result;
/// 歌单列表
/// 对应 Node.js module/top_playlist.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌单列表（网友精选碟）
    /// 对应 /top/playlist
    pub async fn top_playlist(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "cat": query.get_or("cat", "全部"),
            "order": query.get_or("order", "hot"),
            "limit": query.get_or("limit", "50").parse::<i64>().unwrap_or(50),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "total": true
        });
        self.request(
            "/api/playlist/list",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
