use super::Query;
use crate::error::Result;
/// 一起听 发送播放状态
/// 对应 Node.js module/listentogether_play_command.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 一起听 发送播放状态
    /// 对应 /listentogether/play/command
    pub async fn listentogether_play_command(&self, query: &Query) -> Result<ApiResponse> {
        let command_info = json!({
            "commandType": query.get_or("commandType", ""),
            "progress": query.get_or("progress", "0"),
            "playStatus": query.get_or("playStatus", ""),
            "formerSongId": query.get_or("formerSongId", ""),
            "targetSongId": query.get_or("targetSongId", ""),
            "clientSeq": query.get_or("clientSeq", "")
        });
        let data = json!({
            "roomId": query.get_or("roomId", ""),
            "commandInfo": command_info.to_string()
        });
        self.request(
            "/api/listen/together/play/command/report",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
