use super::Query;
use crate::error::Result;
/// 已购买单曲
/// 对应 Node.js module/song_singledownlist.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 已购买单曲
    /// 对应 /song/singledownlist
    pub async fn song_singledownlist(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "20"),
            "offset": query.get_or("offset", "0"),
            "total": "true",
        });
        self.request(
            "/api/member/song/singledownlist",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
