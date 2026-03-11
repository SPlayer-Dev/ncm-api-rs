use super::Query;
use crate::error::Result;
/// 听歌足迹 - 总收听时长
/// 对应 Node.js module/listen_data_total.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 听歌足迹 - 总收听时长
    /// 对应 /listen/data/total
    pub async fn listen_data_total(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/content/activity/listen/data/total",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
