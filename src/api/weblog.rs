use super::Query;
use crate::error::Result;
/// Weblog
/// 对应 Node.js module/weblog.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// Weblog 上报
    /// 对应 /weblog
    pub async fn weblog(&self, query: &Query) -> Result<ApiResponse> {
        // 通用 weblog 接口，data 由调用方传入
        let data = if let Some(data_str) = query.get("data") {
            serde_json::from_str(data_str).unwrap_or(json!({}))
        } else {
            json!({})
        };
        self.request(
            "/api/feedback/weblog",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
