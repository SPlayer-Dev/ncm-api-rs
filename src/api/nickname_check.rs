use super::Query;
use crate::error::Result;
/// 昵称是否重复
/// 对应 Node.js module/nickname_check.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 昵称是否重复
    /// 对应 /nickname/check
    pub async fn nickname_check(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "nickname": query.get("nickname").unwrap_or("")
        });
        self.request(
            "/api/nickname/duplicated",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
