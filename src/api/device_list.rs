use super::Query;
use crate::error::Result;
/// 登录设备列表
/// 对应 Node.js module/device_list.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 登录设备列表
    /// 对应 /device/list
    pub async fn device_list(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "excStatus": "9"
        });
        self.request(
            "/api/middle/user/device/list",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
