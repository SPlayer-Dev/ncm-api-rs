use super::Query;
use crate::error::Result;
/// MV 播放链接
/// 对应 Node.js module/mv_url.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// MV 播放链接
    /// 对应 /mv/url
    pub async fn mv_url(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", "0"),
            "r": query.get_or("r", "1080").parse::<i64>().unwrap_or(1080)
        });
        self.request(
            "/api/song/enhance/play/mv/url",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
