use super::Query;
use crate::error::{NcmError, Result};
/// 提交歌曲播放状态
/// 对应 Node.js module/relay_play_state_submit.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use rand::RngExt;
use serde_json::json;

impl ApiClient {
    /// 提交歌曲播放状态
    /// 对应 /relay/play/state/submit
    ///
    /// 参数：id, sessionId（缺省随机）, progress（默认 0）, playMode（默认 list_loop）, type（默认 song）
    pub async fn relay_play_state_submit(&self, query: &Query) -> Result<ApiResponse> {
        let id = query.get_or("id", "");
        if id.is_empty() {
            return Err(NcmError::InvalidParam("缺少必要参数：id".into()));
        }
        let session_id = query
            .get("sessionId")
            .map(str::to_string)
            .unwrap_or_else(|| {
                const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
                let mut rng = rand::rng();
                (0..12)
                    .map(|_| CHARS[rng.random_range(0..36)] as char)
                    .collect()
            });
        let req = json!({
            "resource": { "id": id, "type": query.get_or("type", "song") },
            "progress": query.get_or("progress", "0").parse::<f64>().unwrap_or(0.0),
            "sessionId": session_id,
            "playMode": query.get_or("playMode", "list_loop")
        });
        self.request(
            "/api/relay/play/state/submit",
            json!({ "playStateSubmitReq": req.to_string() }),
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
