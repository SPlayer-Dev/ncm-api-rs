use super::Query;
use crate::error::Result;
/// 广播电台 - 我的收藏
/// 对应 Node.js module/broadcast_channel_collect_list.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 广播电台 - 我的收藏
    /// 对应 /broadcast/channel/collect/list
    pub async fn broadcast_channel_collect_list(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "contentType": "BROADCAST",
            "limit": query.get_or("limit", "99999"),
            "timeReverseOrder": "true",
            "startDate": "4762584922000"
        });
        self.request(
            "/api/content/channel/collect/list",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
