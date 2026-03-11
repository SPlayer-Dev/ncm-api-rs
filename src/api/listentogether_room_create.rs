use super::Query;
use crate::error::Result;
/// 一起听 创建房间
/// 对应 Node.js module/listentogether_room_create.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 一起听 创建房间
    /// 对应 /listentogether/room/create
    pub async fn listentogether_room_create(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "refer": "songplay_more"
        });
        self.request(
            "/api/listen/together/room/create",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
