use super::Query;
use crate::error::Result;
/// 歌单分类列表
/// 对应 Node.js module/playlist_catlist.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌单分类列表
    /// 对应 /playlist/catlist
    pub async fn playlist_catlist(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/playlist/catalogue",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
