use super::Query;
use crate::error::Result;
/// 云贝推歌
/// 对应 Node.js module/yunbei_rcmd_song.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云贝推歌
    /// 对应 /yunbei/rcmd/song
    pub async fn yunbei_rcmd_song(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "songId": query.get_or("id", ""),
            "reason": query.get_or("reason", "好歌献给你"),
            "scene": "",
            "fromUserId": -1,
            "yunbeiNum": query.get_or("yunbeiNum", "10").parse::<i64>().unwrap_or(10)
        });
        self.request(
            "/api/yunbei/rcmd/song/submit",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
