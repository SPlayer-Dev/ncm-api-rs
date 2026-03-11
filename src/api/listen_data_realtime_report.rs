use super::Query;
use crate::error::Result;
/// 听歌足迹 - 本周/本月收听时长
/// 对应 Node.js module/listen_data_realtime_report.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 听歌足迹 - 本周/本月收听时长
    /// 对应 /listen/data/realtime/report
    pub async fn listen_data_realtime_report(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "type": query.get_or("type", "week")
        });
        self.request(
            "/api/content/activity/listen/data/realtime/report",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
