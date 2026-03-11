use super::Query;
use crate::error::Result;
/// 听歌排行
/// 对应 Node.js module/user_record.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 听歌排行
    /// 对应 /user/record
    pub async fn user_record(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "uid": query.get_or("uid", "0"),
            "type": query.get_or("type", "0").parse::<i64>().unwrap_or(0)
        });
        self.request(
            "/api/v1/play/record",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
