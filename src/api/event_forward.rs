use super::Query;
use crate::error::Result;
/// 转发动态
/// 对应 Node.js module/event_forward.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 转发动态
    /// 对应 /event/forward
    pub async fn event_forward(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "forwards": query.get_or("forwards", ""),
            "id": query.get_or("evId", "0"),
            "eventUserId": query.get_or("uid", "0")
        });
        self.request(
            "/api/event/forward",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
