use super::Query;
use crate::error::Result;
/// 搜索建议PC端
/// 对应 Node.js module/search_suggest_pc.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 搜索建议PC端
    /// 对应 /search/suggest/pc
    pub async fn search_suggest_pc(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "keyword": query.get_or("keyword", "")
        });
        self.request(
            "/api/search/pc/suggest/keyword/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
