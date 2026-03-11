use super::Query;
use crate::error::Result;
/// 网易云歌曲解灰 (适配 SPlayer 的 UNM-Server)
/// 对应 Node.js module/song_url_match.js
///
/// 注意: 此功能依赖 Node.js 的 @neteasecloudmusicapienhanced/unblockmusic-utils，
/// Rust 版本仅提供占位实现，返回不支持的错误。
use crate::request::{ApiClient, ApiResponse};
use serde_json::json;

impl ApiClient {
    /// 网易云歌曲解灰
    /// 对应 /song/url/match
    ///
    /// 此功能依赖外部 unblockmusic-utils，Rust SDK 暂不支持。
    pub async fn song_url_match(&self, _query: &Query) -> Result<ApiResponse> {
        Ok(ApiResponse {
            status: 500,
            body: json!({
                "code": 500,
                "msg": "song_url_match is not supported in Rust SDK",
                "data": [],
            }),
            cookie: vec![],
        })
    }
}
