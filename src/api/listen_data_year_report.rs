use super::Query;
use crate::error::Result;
/// 听歌足迹 - 年度听歌足迹
/// 对应 Node.js module/listen_data_year_report.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 听歌足迹 - 年度听歌足迹
    /// 对应 /listen/data/year/report
    pub async fn listen_data_year_report(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/content/activity/listen/data/year/report",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
