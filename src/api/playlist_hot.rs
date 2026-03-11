use super::Query;
use crate::error::Result;
/// 热门歌单标签
/// 对应 Node.js module/playlist_hot.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 热门歌单标签
    /// 对应 /playlist/hot
    pub async fn playlist_hot(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/playlist/hottags",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
