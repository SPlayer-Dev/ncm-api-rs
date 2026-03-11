use super::Query;
use crate::error::Result;
/// 最近播放-电台
/// 对应 Node.js module/record_recent_dj.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 最近播放-电台
    /// 对应 /record/recent/dj
    pub async fn record_recent_dj(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "100").parse::<i64>().unwrap_or(100)
        });
        self.request(
            "/api/play-record/djradio/list",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
