use super::Query;
use crate::error::Result;
/// 推荐视频
/// 对应 Node.js module/video_timeline_recommend.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 推荐视频
    /// 对应 /video/timeline/recommend
    pub async fn video_timeline_recommend(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "filterLives": "[]",
            "withProgramInfo": "true",
            "needUrl": "1",
            "resolution": "480"
        });
        self.request(
            "/api/videotimeline/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
