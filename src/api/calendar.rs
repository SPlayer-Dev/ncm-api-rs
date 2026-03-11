use super::Query;
use crate::error::Result;
/// 音乐日历
/// 对应 Node.js module/calendar.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 音乐日历
    /// 对应 /calendar
    pub async fn calendar(&self, query: &Query) -> Result<ApiResponse> {
        let now = chrono::Utc::now().timestamp_millis().to_string();
        let data = json!({
            "startTime": query.get_or("startTime", &now).parse::<i64>().unwrap_or(0),
            "endTime": query.get_or("endTime", &now).parse::<i64>().unwrap_or(0)
        });
        self.request(
            "/api/mcalendar/detail",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
