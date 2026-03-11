use super::Query;
use crate::error::Result;
/// 一起听 房间情况
/// 对应 Node.js module/listentogether_room_check.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 一起听 房间情况
    /// 对应 /listentogether/room/check
    pub async fn listentogether_room_check(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "roomId": query.get_or("roomId", "")
        });
        self.request(
            "/api/listen/together/room/check",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
