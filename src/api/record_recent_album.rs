use super::Query;
use crate::error::Result;
/// 最近播放-专辑
/// 对应 Node.js module/record_recent_album.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 最近播放-专辑
    /// 对应 /record/recent/album
    pub async fn record_recent_album(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "100").parse::<i64>().unwrap_or(100)
        });
        self.request(
            "/api/play-record/album/list",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
