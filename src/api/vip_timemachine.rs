use super::Query;
use crate::error::Result;
/// 黑胶时光机
/// 对应 Node.js module/vip_timemachine.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 黑胶时光机
    /// 对应 /vip/timemachine
    pub async fn vip_timemachine(&self, query: &Query) -> Result<ApiResponse> {
        let mut data = json!({});
        if let (Some(start_time), Some(end_time)) = (query.get("startTime"), query.get("endTime")) {
            data = json!({
                "startTime": start_time,
                "endTime": end_time,
                "type": 1,
                "limit": query.get_or("limit", "60").parse::<i64>().unwrap_or(60)
            });
        }
        self.request(
            "/api/vipmusic/newrecord/weekflow",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
