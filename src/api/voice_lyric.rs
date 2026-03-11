use super::Query;
use crate::error::Result;
/// 声音歌词
/// 对应 Node.js module/voice_lyric.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 声音歌词
    /// 对应 /voice/lyric
    pub async fn voice_lyric(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "programId": query.get_or("id", "")
        });
        self.request(
            "/api/voice/lyric/get",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
