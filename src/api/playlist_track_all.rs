use super::Query;
use crate::error::Result;
/// 歌单所有歌曲
/// 对应 Node.js module/playlist_track_all.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌单所有歌曲（两步请求：先获取 trackIds，再获取歌曲详情）
    /// 对应 /playlist/track/all
    pub async fn playlist_track_all(&self, query: &Query) -> Result<ApiResponse> {
        let id = query.get_or("id", "0");
        let limit = query.get_or("limit", "1000").parse::<i64>().unwrap_or(1000);
        let offset = query.get_or("offset", "0").parse::<i64>().unwrap_or(0);

        // Step 1: 获取歌单详情（含 trackIds）
        let data1 = json!({
            "id": id,
            "n": 100000,
            "s": query.get_or("s", "8").parse::<i64>().unwrap_or(8)
        });
        let res = self
            .request(
                "/api/v6/playlist/detail",
                data1,
                query.to_option(CryptoType::default()),
            )
            .await?;

        // Step 2: 从 trackIds 中截取分页范围
        if let Some(track_ids) = res.body["playlist"]["trackIds"].as_array() {
            let start = offset as usize;
            let end = std::cmp::min(start + limit as usize, track_ids.len());
            let sliced = &track_ids[start..end];

            let c: Vec<String> = sliced
                .iter()
                .filter_map(|t| t["id"].as_i64())
                .map(|id| format!(r#"{{"id":{}}}"#, id))
                .collect();

            let data2 = json!({
                "c": format!("[{}]", c.join(","))
            });
            self.request(
                "/api/v3/song/detail",
                data2,
                query.to_option(CryptoType::default()),
            )
            .await
        } else {
            Ok(res)
        }
    }
}
