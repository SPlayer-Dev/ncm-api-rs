use super::Query;
use crate::error::Result;
/// 视频分组
/// 对应 Node.js module/video_group.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 视频标签/分组下的视频
    /// 对应 /video/group
    pub async fn video_group(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "groupId": query.get_or("id", "0"),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "need_preview_url": "true",
            "total": true
        });
        self.request(
            "/api/videotimeline/videogroup/otherclient/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
