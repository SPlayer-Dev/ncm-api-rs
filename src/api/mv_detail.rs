use super::Query;
use crate::error::Result;
/// MV 详情
/// 对应 Node.js module/mv_detail.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// MV 详情
    /// 对应 /mv/detail
    pub async fn mv_detail(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("mvid", "0")
        });
        self.request(
            "/api/v1/mv/detail",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
