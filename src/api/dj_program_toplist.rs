use super::Query;
use crate::error::Result;
/// 电台节目榜
/// 对应 Node.js module/dj_program_toplist.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 电台节目榜
    /// 对应 /dj/program/toplist
    pub async fn dj_program_toplist(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "100").parse::<i64>().unwrap_or(100),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0)
        });
        self.request(
            "/api/program/toplist/v1",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
