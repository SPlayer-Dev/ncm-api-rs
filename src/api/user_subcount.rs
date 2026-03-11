use super::Query;
use crate::error::Result;
/// 用户订阅数
/// 对应 Node.js module/user_subcount.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户订阅数
    /// 对应 /user/subcount
    pub async fn user_subcount(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request("/api/subcount", data, query.to_option(CryptoType::Weapi))
            .await
    }
}
