use super::Query;
use crate::error::Result;
/// 推荐电台
/// 对应 Node.js module/personalized_djprogram.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 推荐电台
    /// 对应 /personalized/djprogram
    pub async fn personalized_djprogram(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/personalized/djprogram",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
