use super::Query;
use crate::error::Result;
/// 电台推荐类型
/// 对应 Node.js module/dj_category_recommend.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 电台推荐类型
    /// 对应 /dj/category/recommend
    pub async fn dj_category_recommend(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/djradio/home/category/recommend",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
