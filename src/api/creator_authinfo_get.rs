use super::Query;
use crate::error::Result;
/// 获取达人用户信息
/// 对应 Node.js module/creator_authinfo_get.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 获取达人用户信息
    /// 对应 /creator/authinfo/get
    pub async fn creator_authinfo_get(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/user/creator/authinfo/get",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
