use super::Query;
use crate::error::Result;
/// 广播电台 - 全部电台
/// 对应 Node.js module/broadcast_channel_list.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 广播电台 - 全部电台
    /// 对应 /broadcast/channel/list
    pub async fn broadcast_channel_list(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "categoryId": query.get_or("categoryId", "0"),
            "regionId": query.get_or("regionId", "0"),
            "limit": query.get_or("limit", "20"),
            "lastId": query.get_or("lastId", "0"),
            "score": query.get_or("score", "-1")
        });
        self.request(
            "/api/voice/broadcast/channel/list",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
