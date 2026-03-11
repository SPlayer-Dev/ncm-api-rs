use super::Query;
use crate::error::Result;
/// 歌手信息
/// 对应 Node.js module/artists.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌手信息
    /// 对应 /artists
    pub async fn artists(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            &format!("/api/v1/artist/{}", query.get_or("id", "0")),
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
