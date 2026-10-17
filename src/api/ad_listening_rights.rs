use super::Query;
use crate::error::Result;
/// 获取免费听时长状态
/// 对应 Node.js module/ad_listening_rights.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 获取免费听时长状态
    /// 对应 /ad/listening/rights
    pub async fn ad_listening_rights(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "entrance": "FREE_LISTEN_RN"
        });
        self.request(
            "/api/ad/homepage/free/tab/extend/v2",
            data,
            query.to_option(CryptoType::Xeapi),
        )
        .await
    }
}
