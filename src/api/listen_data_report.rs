use super::Query;
use crate::error::Result;
/// 听歌足迹 - 周/月/年收听报告
/// 对应 Node.js module/listen_data_report.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 听歌足迹 - 周/月/年收听报告
    /// 对应 /listen/data/report
    pub async fn listen_data_report(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "type": query.get_or("type", "week"),
            "endTime": query.get_or("endTime", "")
        });
        self.request(
            "/api/content/activity/listen/data/report",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
