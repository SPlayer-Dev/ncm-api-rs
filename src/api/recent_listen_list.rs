use super::Query;
use crate::error::Result;
/// 最近听歌列表
/// 对应 Node.js module/recent_listen_list.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 最近听歌列表
    /// 对应 /recent/listen/list
    pub async fn recent_listen_list(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/pc/recent/listen/list",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
