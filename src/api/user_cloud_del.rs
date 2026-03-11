use super::Query;
use crate::error::Result;
/// 用户云盘歌曲删除
/// 对应 Node.js module/user_cloud_del.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 用户云盘歌曲删除
    /// 对应 /user/cloud/del
    pub async fn user_cloud_del(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "songIds": [query.get_or("id", "0")]
        });
        self.request("/api/cloud/del", data, query.to_option(CryptoType::Weapi))
            .await
    }
}
