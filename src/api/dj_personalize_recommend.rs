use super::Query;
use crate::error::Result;
/// 电台个性推荐
/// 对应 Node.js module/dj_personalize_recommend.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 电台个性推荐
    /// 对应 /dj/personalize/recommend
    pub async fn dj_personalize_recommend(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "6").parse::<i64>().unwrap_or(6)
        });
        self.request(
            "/api/djradio/personalize/rcmd",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
