use super::Query;
use crate::error::Result;
/// 热门搜索
/// 对应 Node.js module/search_hot.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 热门搜索
    /// 对应 /search/hot
    pub async fn search_hot(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "type": 1111,
        });
        self.request(
            "/api/search/hot",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
