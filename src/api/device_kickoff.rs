use super::Query;
use crate::error::Result;
/// 强制下线设备
/// 对应 Node.js module/device_kickoff.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 强制下线设备
    /// 对应 /device/kickoff
    pub async fn device_kickoff(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "key": query.get_or("deviceKey", ""),
            "captcha": query.get_or("captcha", "")
        });
        self.request(
            "/api/middle/user/security/device/kickoff",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
