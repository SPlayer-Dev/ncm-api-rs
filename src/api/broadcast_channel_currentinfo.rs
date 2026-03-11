use super::Query;
use crate::error::Result;
/// 广播电台 - 电台信息
/// 对应 Node.js module/broadcast_channel_currentinfo.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 广播电台 - 电台信息
    /// 对应 /broadcast/channel/currentinfo
    pub async fn broadcast_channel_currentinfo(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "channelId": query.get_or("id", "")
        });
        self.request(
            "/api/voice/broadcast/channel/currentinfo",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
