use super::Query;
use crate::error::Result;
/// 用户创建的电台
/// 对应 Node.js module/user_audio.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户创建的电台
    /// 对应 /user/audio
    pub async fn user_audio(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "userId": query.get("uid").unwrap_or("")
        });
        self.request(
            "/api/djradio/get/byuser",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
