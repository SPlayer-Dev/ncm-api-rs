use super::Query;
use crate::error::Result;
/// 搜索建议
/// 对应 Node.js module/search_suggest.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 搜索建议
    /// 对应 /search/suggest
    pub async fn search_suggest(&self, query: &Query) -> Result<ApiResponse> {
        let suggest_type = if query.get_or("type", "") == "mobile" {
            "keyword"
        } else {
            "web"
        };
        let data = json!({
            "s": query.get_or("keywords", "")
        });
        self.request(
            &format!("/api/search/suggest/{}", suggest_type),
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
