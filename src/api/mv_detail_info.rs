use super::Query;
use crate::error::Result;
/// MV 点赞转发评论数数据
/// 对应 Node.js module/mv_detail_info.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// MV 点赞转发评论数数据
    /// 对应 /mv/detail/info
    pub async fn mv_detail_info(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "threadid": format!("R_MV_5_{}", query.get_or("mvid", "0")),
            "composeliked": true
        });
        self.request(
            "/api/comment/commentthread/info",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
