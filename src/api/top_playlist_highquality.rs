use super::Query;
use crate::error::Result;
/// 精品歌单
/// 对应 Node.js module/top_playlist_highquality.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 精品歌单
    /// 对应 /top/playlist/highquality
    pub async fn top_playlist_highquality(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "cat": query.get_or("cat", "全部"),
            "limit": query.get_or("limit", "50").parse::<i64>().unwrap_or(50),
            "lasttime": query.get_or("before", "0").parse::<i64>().unwrap_or(0),
            "total": true
        });
        self.request(
            "/api/playlist/highquality/list",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
