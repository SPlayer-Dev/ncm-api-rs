use super::Query;
use crate::error::Result;
/// 相似用户
/// 对应 Node.js module/simi_user.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 相似用户
    /// 对应 /simi/user
    pub async fn simi_user(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "songid": query.get_or("id", "0"),
            "limit": query.get_or("limit", "50"),
            "offset": query.get_or("offset", "0")
        });
        self.request(
            "/api/discovery/simiUser",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
