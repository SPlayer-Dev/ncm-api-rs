use super::Query;
use crate::error::Result;
/// 签到进度
/// 对应 Node.js module/signin_progress.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 签到进度
    /// 对应 /signin/progress
    pub async fn signin_progress(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "moduleId": query.get_or("moduleId", "1207signin-1207signin")
        });
        self.request(
            "/api/act/modules/signin/v2/progress",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
