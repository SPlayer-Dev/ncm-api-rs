use super::Query;
use crate::error::Result;
/// 检查音乐是否可用
/// 对应 Node.js module/check_music.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 检查音乐是否可用
    /// 对应 /check/music
    pub async fn check_music(&self, query: &Query) -> Result<ApiResponse> {
        let id = query.get_or("id", "0").parse::<i64>().unwrap_or(0);
        let br = query
            .get_or("br", "999000")
            .parse::<i64>()
            .unwrap_or(999000);
        let data = json!({
            "ids": format!("[{}]", id),
            "br": br
        });
        self.request(
            "/api/song/enhance/player/url",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
