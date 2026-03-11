use super::Query;
use crate::error::Result;
/// 电台 Banner
/// 对应 Node.js module/dj_banner.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 电台 Banner
    /// 对应 /dj/banner
    pub async fn dj_banner(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/djradio/banner/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
