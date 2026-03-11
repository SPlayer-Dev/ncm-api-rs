use super::Query;
use crate::error::Result;
/// 视频点赞转发评论数数据
/// 对应 Node.js module/video_detail_info.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 视频点赞转发评论数数据
    /// 对应 /video/detail/info
    pub async fn video_detail_info(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "threadid": format!("R_VI_62_{}", query.get_or("vid", "")),
            "composeliked": true
        });
        self.request(
            "/api/comment/commentthread/info",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
