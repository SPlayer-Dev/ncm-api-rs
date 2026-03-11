use super::Query;
use crate::error::Result;
/// 私人 FM 模式
/// 对应 Node.js module/personal_fm_mode.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 私人 FM 模式
    /// 对应 /personal/fm/mode
    pub async fn personal_fm_mode(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "mode": query.get_or("mode", "DEFAULT"),
            "subMode": query.get_or("submode", ""),
            "limit": query.get_or("limit", "3").parse::<i64>().unwrap_or(3)
        });
        self.request(
            "/api/v1/radio/get",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
