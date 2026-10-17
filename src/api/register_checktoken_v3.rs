use super::Query;
use crate::error::Result;
/// 易盾反作弊 token（v3）
/// 对应 Node.js module/register_checktoken_v3.js
use crate::request::{ApiClient, ApiResponse};
use serde_json::json;

impl ApiClient {
    /// 实时获取易盾 v3 反作弊 token（不缓存）
    /// 对应 /register/checktoken/v3
    pub async fn register_checktoken_v3(&self, _query: &Query) -> Result<ApiResponse> {
        let token = self.fetch_check_token_v3().await.unwrap_or_default();
        Ok(ApiResponse {
            status: 200,
            body: json!({ "code": 200, "token": token, "registered": !token.is_empty() }),
            cookie: vec![],
        })
    }
}
