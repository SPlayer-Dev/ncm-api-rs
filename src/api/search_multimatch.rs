use super::Query;
use crate::error::Result;
/// 搜索多重匹配
/// 对应 Node.js module/search_multimatch.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 搜索多重匹配
    /// 对应 /search/multimatch
    pub async fn search_multimatch(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "type": query.get_or("type", "1").parse::<i64>().unwrap_or(1),
            "s": query.get_or("keywords", "")
        });
        self.request(
            "/api/search/suggest/multimatch",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
