use super::Query;
use crate::error::Result;
/// 我创建的播客声音
/// 对应 Node.js module/voicelist_my_created.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 我创建的播客声音
    /// 对应 /voicelist/my/created
    pub async fn voicelist_my_created(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "20").parse::<i64>().unwrap_or(20)
        });
        self.request(
            "/api/social/my/created/voicelist/v1",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
