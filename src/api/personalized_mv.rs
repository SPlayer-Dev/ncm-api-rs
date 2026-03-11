use super::Query;
use crate::error::Result;
/// 推荐 MV
/// 对应 Node.js module/personalized_mv.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 推荐 MV
    /// 对应 /personalized/mv
    pub async fn personalized_mv(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/personalized/mv",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
