use super::Query;
use crate::error::Result;
/// 广播电台 - 收藏/取消收藏电台
/// 对应 Node.js module/broadcast_sub.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 广播电台 - 收藏/取消收藏电台
    /// 对应 /broadcast/sub
    pub async fn broadcast_sub(&self, query: &Query) -> Result<ApiResponse> {
        let cancel_collect = if query.get_or("t", "0") == "1" {
            "false"
        } else {
            "true"
        };
        let data = json!({
            "contentType": "BROADCAST",
            "contentId": query.get_or("id", ""),
            "cancelCollect": cancel_collect
        });
        self.request(
            "/api/content/interact/collect",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
