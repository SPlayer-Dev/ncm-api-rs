use super::Query;
use crate::error::Result;
/// 插播相似歌曲
/// 对应 Node.js module/song_simi_get.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 插播相似歌曲
    /// 对应 /song/simi/get
    pub async fn song_simi_get(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "positionCode": "toolBarRcmdSong",
            "resourceId": query.get_or("id", ""),
            "resourceType": "song"
        });
        self.request(
            "/api/link/position/show/resource",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
