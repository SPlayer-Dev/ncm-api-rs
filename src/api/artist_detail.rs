use super::Query;
use crate::error::Result;
/// 歌手详情
/// 对应 Node.js module/artist_detail.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌手详情
    /// 对应 /artist/detail
    pub async fn artist_detail(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", "0")
        });
        self.request(
            "/api/artist/head/info/get",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
