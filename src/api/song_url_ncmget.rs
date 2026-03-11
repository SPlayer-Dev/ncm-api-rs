use super::Query;
use crate::error::Result;
/// 歌曲播放链接 (ncmget 占位)
/// 对应 Node.js module/song_url_ncmget.js
use crate::request::{ApiClient, ApiResponse};
use serde_json::json;

impl ApiClient {
    /// 歌曲播放链接 (ncmget 占位接口，始终返回空数据)
    /// 对应 /song/url/ncmget
    pub async fn song_url_ncmget(&self, _query: &Query) -> Result<ApiResponse> {
        Ok(ApiResponse {
            status: 200,
            body: json!({ "code": 200, "data": [] }),
            cookie: vec![],
        })
    }
}
