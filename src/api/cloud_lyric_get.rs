use super::Query;
use crate::error::Result;
/// 获取云盘歌词
/// 对应 Node.js module/cloud_lyric_get.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 获取云盘歌词
    /// 对应 /cloud/lyric/get
    pub async fn cloud_lyric_get(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "userId": query.get_or("uid", ""),
            "songId": query.get_or("sid", ""),
            "lv": -1,
            "kv": -1
        });
        self.request(
            "/api/cloud/lyric/get",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
