use super::Query;
use crate::error::Result;
/// 默认搜索关键词
/// 对应 Node.js module/search_default.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 默认搜索关键词
    /// 对应 /search/default
    pub async fn search_default(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/search/defaultkeyword/get",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
