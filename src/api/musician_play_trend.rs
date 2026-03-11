use super::Query;
use crate::error::Result;
/// 音乐人歌曲播放趋势
/// 对应 Node.js module/musician_play_trend.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 音乐人歌曲播放趋势
    /// 对应 /musician/play/trend
    pub async fn musician_play_trend(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "startTime": query.get_or("startTime", ""),
            "endTime": query.get_or("endTime", "")
        });
        self.request(
            "/api/creator/musician/play/count/statistic/data/trend/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
