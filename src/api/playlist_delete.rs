use super::Query;
use crate::error::Result;
/// 删除歌单
/// 对应 Node.js module/playlist_delete.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 删除歌单
    /// 对应 /playlist/delete
    pub async fn playlist_delete(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "ids": format!("[{}]", query.get_or("id", "0"))
        });
        self.request(
            "/api/playlist/remove",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
