use super::Query;
use crate::error::Result;
/// 电台详情
/// 对应 Node.js module/dj_detail.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 电台详情
    /// 对应 /dj/detail
    pub async fn dj_detail(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("rid", "0")
        });
        self.request(
            "/api/djradio/v2/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
