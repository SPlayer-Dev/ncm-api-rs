use super::Query;
use crate::error::Result;
/// 歌手简要百科信息
/// 对应 Node.js module/ugc_artist_get.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌手简要百科信息
    /// 对应 /ugc/artist/get
    pub async fn ugc_artist_get(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "artistId": query.get_or("id", "")
        });
        self.request(
            "/api/rep/ugc/artist/get",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
