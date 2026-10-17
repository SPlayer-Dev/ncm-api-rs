use super::Query;
use crate::error::Result;
/// 红心与取消红心歌曲 v1
/// 对应 Node.js module/like_v1.js
use crate::request::{ApiClient, ApiResponse, CheckToken, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 红心与取消红心歌曲 v1
    /// 对应 /like/v1
    pub async fn like_v1(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "alg": "itembased",
            "trackId": query.get_or("id", ""),
            "like": query.get("like") != Some("false"),
            "time": "3"
        });
        self.request(
            "/api/v1/radio/like",
            data,
            query.to_option_token(CryptoType::Xeapi, CheckToken::V3),
        )
        .await
    }
}
