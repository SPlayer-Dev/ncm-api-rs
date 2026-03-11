use super::Query;
use crate::error::Result;
/// 收藏/取消收藏视频
/// 对应 Node.js module/video_sub.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 收藏/取消收藏视频
    /// 对应 /video/sub
    pub async fn video_sub(&self, query: &Query) -> Result<ApiResponse> {
        let t = query.get_or("t", "1");
        let path = if t == "1" { "sub" } else { "unsub" };
        let data = json!({
            "id": query.get_or("id", "0")
        });
        self.request(
            &format!("/api/cloudvideo/video/{}", path),
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
