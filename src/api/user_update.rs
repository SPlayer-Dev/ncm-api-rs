use super::Query;
use crate::error::Result;
/// 更新用户资料
/// 对应 Node.js module/user_update.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 更新用户资料
    /// 对应 /user/update
    pub async fn user_update(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "birthday": query.get_or("birthday", ""),
            "city": query.get_or("city", ""),
            "gender": query.get_or("gender", ""),
            "nickname": query.get_or("nickname", ""),
            "province": query.get_or("province", ""),
            "signature": query.get_or("signature", "")
        });
        self.request(
            "/api/user/profile/update",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
