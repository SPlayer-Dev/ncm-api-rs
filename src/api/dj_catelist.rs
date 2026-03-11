use super::Query;
use crate::error::Result;
/// 电台分类列表
/// 对应 Node.js module/dj_catelist.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 电台分类列表
    /// 对应 /dj/catelist
    pub async fn dj_catelist(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/djradio/category/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
