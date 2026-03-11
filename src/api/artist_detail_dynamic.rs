use super::Query;
use crate::error::Result;
/// 歌手动态信息
/// 对应 Node.js module/artist_detail_dynamic.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌手动态信息
    /// 对应 /artist/detail/dynamic
    pub async fn artist_detail_dynamic(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", "0")
        });
        self.request(
            "/api/artist/detail/dynamic",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
