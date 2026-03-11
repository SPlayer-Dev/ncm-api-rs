use super::Query;
use crate::error::Result;
/// 音乐人签到
/// 对应 Node.js module/musician_sign.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 音乐人签到
    /// 对应 /musician/sign
    pub async fn musician_sign(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/creator/user/access",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
