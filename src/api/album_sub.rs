use super::Query;
use crate::error::Result;
/// 收藏/取消收藏专辑
/// 对应 Node.js module/album_sub.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 收藏/取消收藏专辑
    /// 对应 /album/sub
    pub async fn album_sub(&self, query: &Query) -> Result<ApiResponse> {
        let t = query.get_or("t", "1");
        let path = if t == "1" { "sub" } else { "unsub" };
        let data = json!({
            "id": query.get_or("id", "0")
        });
        self.request(
            &format!("/api/album/{}", path),
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
