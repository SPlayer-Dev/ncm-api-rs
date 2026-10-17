use super::Query;
use crate::error::Result;
/// 从云盘获取歌曲下载链接
/// 对应 Node.js module/song_cloud_download.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 从云盘获取歌曲下载链接
    /// 对应 /song/cloud/download
    pub async fn song_cloud_download(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "songId": query.get_or("id", "")
        });
        self.request(
            "/api/cloud/dowonload",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
