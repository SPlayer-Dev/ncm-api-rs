use super::Query;
use crate::error::Result;
/// 电台节目详情
/// 对应 Node.js module/dj_program_detail.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 电台节目详情
    /// 对应 /dj/program/detail
    pub async fn dj_program_detail(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", "0")
        });
        self.request(
            "/api/dj/program/detail",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
