use super::Query;
use crate::error::Result;
/// 年度听歌报告2017-2023
/// 对应 Node.js module/summary_annual.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 年度听歌报告2017-2023
    /// 对应 /summary/annual
    pub async fn summary_annual(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        let year = query.get_or("year", "2023");
        let key = match year.as_str() {
            "2017" | "2018" | "2019" => "userdata",
            _ => "data",
        };
        let url = format!("/api/activity/summary/annual/{}/{}", year, key);
        self.request(&url, data, query.to_option(CryptoType::default()))
            .await
    }
}
