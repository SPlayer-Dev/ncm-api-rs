use super::Query;
use crate::error::Result;
/// 收藏/取消收藏歌曲到歌单
/// 对应 Node.js module/playlist_tracks.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 收藏/删除歌曲到歌单
    /// 对应 /playlist/tracks
    ///
    /// 走 weapi（与 issue #2 报告者验证可用的方式一致）。
    /// 该 issue 偶发「发送失败」的根因是 eapi 域名 5 秒关闭空闲连接、连接池复用了已关闭的连接，已在 request.rs 中修复
    pub async fn playlist_tracks(&self, query: &Query) -> Result<ApiResponse> {
        let tracks: Vec<String> = query
            .get_or("tracks", "")
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let build = |ids: &[String]| {
            json!({
                "op": query.get_or("op", "add"),
                "pid": query.get_or("pid", "0"),
                "trackIds": serde_json::to_string(ids).unwrap_or_default(),
                "imme": "true"
            })
        };
        let res = self
            .request(
                "/api/playlist/manipulate/tracks",
                build(&tracks),
                query.to_option(CryptoType::Weapi),
            )
            .await;
        match res {
            // 512 时与 Node 版一致：重复一遍 trackIds 后重试
            Err(e) if e.status_code() == Some(512) => {
                let doubled = [tracks.clone(), tracks].concat();
                self.request(
                    "/api/playlist/manipulate/tracks",
                    build(&doubled),
                    query.to_option(CryptoType::Weapi),
                )
                .await
            }
            other => other,
        }
    }
}
