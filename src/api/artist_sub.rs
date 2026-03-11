use super::Query;
use crate::error::Result;
/// 收藏/取消收藏歌手
/// 对应 Node.js module/artist_sub.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 收藏/取消收藏歌手
    /// 对应 /artist/sub
    pub async fn artist_sub(&self, query: &Query) -> Result<ApiResponse> {
        let t = query.get_or("t", "1");
        let path = if t == "1" { "sub" } else { "unsub" };
        let id = query.get_or("id", "0");
        let data = json!({
            "artistId": id,
            "artistIds": format!("[{}]", id)
        });
        self.request(
            &format!("/api/artist/{}", path),
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
