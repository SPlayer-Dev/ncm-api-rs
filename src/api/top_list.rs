use super::Query;
use crate::error::Result;
/// 排行榜
/// 对应 Node.js module/top_list.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 排行榜
    /// 对应 /top/list
    pub async fn top_list(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", ""),
            "n": "500",
            "s": "0"
        });
        self.request(
            "/api/playlist/v4/detail",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
