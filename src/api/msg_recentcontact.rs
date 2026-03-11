use super::Query;
use crate::error::Result;
/// 最近联系
/// 对应 Node.js module/msg_recentcontact.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 最近联系
    /// 对应 /msg/recentcontact
    pub async fn msg_recentcontact(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/msg/recentcontact/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
