use super::Query;
use crate::error::Result;
/// 会员本月下载歌曲记录
/// 对应 Node.js module/song_monthdownlist.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 会员本月下载歌曲记录
    /// 对应 /song/monthdownlist
    pub async fn song_monthdownlist(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "20"),
            "offset": query.get_or("offset", "0"),
            "total": "true",
        });
        self.request(
            "/api/member/song/monthdownlist",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
