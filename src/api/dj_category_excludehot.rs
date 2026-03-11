use super::Query;
use crate::error::Result;
/// 电台非热门类型
/// 对应 Node.js module/dj_category_excludehot.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 电台非热门类型
    /// 对应 /dj/category/excludehot
    pub async fn dj_category_excludehot(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/djradio/category/excludehot",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
