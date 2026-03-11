use super::Query;
use crate::error::Result;
/// 黑胶乐签打卡
/// 对应 Node.js module/vip_sign.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 黑胶乐签打卡
    /// 对应 /vip/sign
    pub async fn vip_sign(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/vip-center-bff/task/sign",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
