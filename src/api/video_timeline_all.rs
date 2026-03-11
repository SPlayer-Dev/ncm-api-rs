use super::Query;
use crate::error::Result;
/// 全部视频列表
/// 对应 Node.js module/video_timeline_all.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 全部视频列表
    /// 对应 /video/timeline/all
    pub async fn video_timeline_all(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "groupId": 0,
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "need_preview_url": "true",
            "total": true
        });
        self.request(
            "/api/videotimeline/otherclient/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
