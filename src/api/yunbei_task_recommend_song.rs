use super::Query;
use crate::error::Result;
/// 云贝广告任务 - 获取推荐歌曲
/// 对应 Node.js module/yunbei_task_recommend_song.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云贝广告任务 - 获取推荐歌曲
    /// 对应 /yunbei/task/recommend/song
    pub async fn yunbei_task_recommend_song(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "offset": query.get_or("offset", "0"),
            "limit": query.get_or("limit", "10")
        });
        self.request(
            "/api/ad/power/yunbei/distribution/recommend/song",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
