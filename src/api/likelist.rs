use super::Query;
use crate::error::Result;
/// 喜欢音乐列表
/// 对应 Node.js module/likelist.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 喜欢音乐列表
    /// 对应 /likelist
    pub async fn likelist(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "uid": query.get_or("uid", "0")
        });
        self.request(
            "/api/song/like/get",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
