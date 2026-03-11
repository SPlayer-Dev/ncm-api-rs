use super::Query;
use crate::error::Result;
/// 刷新登录
/// 对应 Node.js module/login_refresh.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 刷新登录
    /// 对应 /login/refresh
    pub async fn login_refresh(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/login/token/refresh",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
