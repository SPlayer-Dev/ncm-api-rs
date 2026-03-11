use super::Query;
use crate::error::Result;
/// 更新歌曲顺序
/// 对应 Node.js module/song_order_update.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 更新歌曲顺序
    /// 对应 /song/order/update
    pub async fn song_order_update(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "pid": query.get_or("pid", ""),
            "trackIds": query.get_or("ids", ""),
            "op": "update",
        });
        self.request(
            "/api/playlist/manipulate/tracks",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
