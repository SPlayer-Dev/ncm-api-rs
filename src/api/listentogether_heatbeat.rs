use super::Query;
use crate::error::Result;
/// 一起听 发送心跳
/// 对应 Node.js module/listentogether_heatbeat.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 一起听 发送心跳
    /// 对应 /listentogether/heatbeat
    pub async fn listentogether_heatbeat(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "roomId": query.get_or("roomId", ""),
            "songId": query.get_or("songId", ""),
            "playStatus": query.get_or("playStatus", ""),
            "progress": query.get_or("progress", "")
        });
        self.request(
            "/api/listen/together/heartbeat",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
