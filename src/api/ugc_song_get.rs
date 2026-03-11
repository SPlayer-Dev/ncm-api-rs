use super::Query;
use crate::error::Result;
/// 歌曲简要百科信息
/// 对应 Node.js module/ugc_song_get.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌曲简要百科信息
    /// 对应 /ugc/song/get
    pub async fn ugc_song_get(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "songId": query.get_or("id", "")
        });
        self.request(
            "/api/rep/ugc/song/get",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
