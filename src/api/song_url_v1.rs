use super::Query;
use crate::error::Result;
/// 歌曲播放链接 v1
/// 对应 Node.js module/song_url_v1.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌曲播放链接
    /// 对应 /song/url/v1
    ///
    /// level: standard, exhigh, lossless, hires, jyeffect(高清臻音), vivid(臻音全景声), jymaster(超清母带), sky(沉浸环绕声)
    /// level 为 sky 时可通过 immerseType 选择沉浸声类型：c512, ste2, aac2, c51, ste, aac（默认 c51）
    pub async fn song_url_v1(&self, query: &Query) -> Result<ApiResponse> {
        let id = query.get_or("id", "0");
        let level = query.get_or("level", "standard");
        let mut data = json!({
            "ids": format!("[{}]", id),
            "level": level,
            "encodeType": "flac"
        });
        let query = match level.as_str() {
            "sky" => {
                data["immerseType"] = json!(query.get_or("immerseType", "c51"));
                query.clone()
            }
            "vivid" => {
                data["encodeType"] = json!("mp3");
                query.with_cookie(&[("os", "android"), ("appver", "9.5.61")])
            }
            _ => query.clone(),
        };
        self.request(
            "/api/song/enhance/player/url/v1",
            data,
            query.to_option(CryptoType::Xeapi),
        )
        .await
    }
}
