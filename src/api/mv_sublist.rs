use super::Query;
use crate::error::Result;
/// MV 收藏列表
/// 对应 Node.js module/mv_sublist.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// MV 收藏列表
    /// 对应 /mv/sublist
    pub async fn mv_sublist(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "limit": query.get_or("limit", "25").parse::<i64>().unwrap_or(25),
            "offset": query.get_or("offset", "0").parse::<i64>().unwrap_or(0),
            "total": true
        });
        self.request(
            "/api/cloudvideo/allvideo/sublist",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
