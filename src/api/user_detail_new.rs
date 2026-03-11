use super::Query;
use crate::error::Result;
/// 用户详情（新版）
/// 对应 Node.js module/user_detail_new.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户详情（新版）
    /// 对应 /user/detail/new
    pub async fn user_detail_new(&self, query: &Query) -> Result<ApiResponse> {
        let uid = query.get_or("uid", "0");
        let data = json!({
            "all": "true",
            "userId": uid
        });
        self.request(
            &format!("/api/w/v1/user/detail/{}", uid),
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
