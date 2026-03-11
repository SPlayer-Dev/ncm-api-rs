use super::Query;
use crate::error::Result;
/// 创建歌单
/// 对应 Node.js module/playlist_create.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 创建歌单
    /// 对应 /playlist/create
    pub async fn playlist_create(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "name": query.get_or("name", ""),
            "privacy": query.get_or("privacy", "0"),
            "type": query.get_or("type", "NORMAL")
        });
        self.request(
            "/api/playlist/create",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
