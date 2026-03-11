use super::Query;
use crate::error::Result;
/// 电台节目列表
/// 对应 Node.js module/dj_program.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 电台节目列表
    /// 对应 /dj/program
    pub async fn dj_program(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "radioId": query.get_or("rid", "0"),
            "limit": query.get_or("limit", "30").parse::<i64>().unwrap_or(30),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "asc": query.get_or("asc", "false") == "true"
        });
        self.request(
            "/api/dj/program/byradio",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
