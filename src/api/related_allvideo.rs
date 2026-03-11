use super::Query;
use crate::error::Result;
/// 相关视频
/// 对应 Node.js module/related_allvideo.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 相关视频
    /// 对应 /related/allvideo
    pub async fn related_allvideo(&self, query: &Query) -> Result<ApiResponse> {
        let id = query.get_or("id", "0");
        let id_type = if id.chars().all(|c| c.is_ascii_digit()) {
            0
        } else {
            1
        };
        let data = json!({
            "id": id,
            "type": id_type
        });
        self.request(
            "/api/cloudvideo/v1/allvideo/rcmd",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
