use super::Query;
use crate::error::Result;
/// 推荐节目
/// 对应 Node.js module/program_recommend.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 推荐节目
    /// 对应 /program/recommend
    pub async fn program_recommend(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "cateId": query.get_or("type", ""),
            "limit": query.get_or("limit", "10").parse::<i64>().unwrap_or(10),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0)
        });
        self.request(
            "/api/program/recommend/v1",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
