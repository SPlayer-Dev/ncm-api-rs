use super::Query;
use crate::error::Result;
/// 视频播放链接
/// 对应 Node.js module/video_url.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 视频播放链接
    /// 对应 /video/url
    pub async fn video_url(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "ids": format!(r#"["{}"]"#, query.get_or("id", "0")),
            "resolution": query.get_or("res", "1080").parse::<i64>().unwrap_or(1080)
        });
        self.request(
            "/api/cloudvideo/playurl",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
