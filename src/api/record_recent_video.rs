use super::Query;
use crate::error::Result;
/// 最近播放-视频
/// 对应 Node.js module/record_recent_video.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 最近播放-视频
    /// 对应 /record/recent/video
    pub async fn record_recent_video(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "100").parse::<i64>().unwrap_or(100)
        });
        self.request(
            "/api/play-record/newvideo/list",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
