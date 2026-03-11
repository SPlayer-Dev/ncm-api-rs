use super::Query;
use crate::error::Result;
/// 一起听 结束房间
/// 对应 Node.js module/listentogether_end.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 一起听 结束房间
    /// 对应 /listentogether/end
    pub async fn listentogether_end(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "roomId": query.get_or("roomId", "")
        });
        self.request(
            "/api/listen/together/end/v2",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
