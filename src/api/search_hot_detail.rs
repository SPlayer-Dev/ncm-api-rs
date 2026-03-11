use super::Query;
use crate::error::Result;
/// 热搜详情
/// 对应 Node.js module/search_hot_detail.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 热搜列表
    /// 对应 /search/hot/detail
    pub async fn search_hot_detail(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/hotsearchlist/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
