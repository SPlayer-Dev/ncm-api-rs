use super::Query;
use crate::error::Result;
/// 歌单排序更新
/// 对应 Node.js module/playlist_order_update.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌单排序更新
    /// 对应 /playlist/order/update
    pub async fn playlist_order_update(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "ids": query.get_or("ids", "")
        });
        self.request(
            "/api/playlist/order/update",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
