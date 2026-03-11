use super::Query;
use crate::error::Result;
/// 所有榜单内容摘要v2
/// 对应 Node.js module/toplist_detail_v2.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 所有榜单内容摘要v2
    /// 对应 /toplist/detail/v2
    pub async fn toplist_detail_v2(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/toplist/detail/v2",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
