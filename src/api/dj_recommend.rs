use super::Query;
use crate::error::Result;
/// 电台推荐
/// 对应 Node.js module/dj_recommend.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 电台推荐
    /// 对应 /dj/recommend
    pub async fn dj_recommend(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/djradio/recommend/v1",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
