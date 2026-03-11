use super::Query;
use crate::error::Result;
/// 电台24小时节目榜
/// 对应 Node.js module/dj_program_toplist_hours.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 电台24小时节目榜
    /// 对应 /dj/program/toplist/hours
    pub async fn dj_program_toplist_hours(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "100").parse::<i64>().unwrap_or(100)
        });
        self.request(
            "/api/djprogram/toplist/hours",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
