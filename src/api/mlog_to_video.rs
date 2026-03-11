use super::Query;
use crate::error::Result;
/// 将mlog id转为video id
/// 对应 Node.js module/mlog_to_video.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 将mlog id转为video id
    /// 对应 /mlog/to/video
    pub async fn mlog_to_video(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "mlogId": query.get_or("id", "")
        });
        self.request(
            "/api/mlog/video/convert/id",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
