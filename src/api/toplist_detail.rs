use super::Query;
use crate::error::Result;
/// 排行榜详情
/// 对应 Node.js module/toplist_detail.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 排行榜详情
    /// 对应 /toplist/detail
    pub async fn toplist_detail(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/toplist/detail",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
