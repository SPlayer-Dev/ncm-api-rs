use super::Query;
use crate::error::Result;
/// 用户等级
/// 对应 Node.js module/user_level.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户等级信息
    /// 对应 /user/level
    pub async fn user_level(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request("/api/user/level", data, query.to_option(CryptoType::Weapi))
            .await
    }
}
