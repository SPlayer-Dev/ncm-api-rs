use super::Query;
use crate::error::Result;
/// 听歌打卡
/// 对应 Node.js module/scrobble.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use crate::util::config::CL_DOMAIN;
use serde_json::json;

impl ApiClient {
    /// 听歌打卡
    /// 对应 /scrobble
    ///
    /// 依次上报 startplay（进「最近播放」）和 play（涨「听歌排行」计数）
    pub async fn scrobble(&self, query: &Query) -> Result<ApiResponse> {
        let query = query.with_cookie(&[("os", "osx")]);
        let id = query.get_or("id", "0");
        let content = format!("id={}", query.get_or("sourceid", ""));

        let startplay = json!({
            "logs": json!([{
                "action": "startplay",
                "json": {
                    "id": id,
                    "type": "song",
                    "mainsite": "1",
                    "mainsiteWeb": "1",
                    "content": content
                }
            }]).to_string()
        });
        let play = json!({
            "logs": json!([{
                "action": "play",
                "json": {
                    "download": 0,
                    "end": "playend",
                    "id": id,
                    "sourceId": query.get_or("sourceid", ""),
                    "time": query.get_i64("time", 0),
                    "type": "song",
                    "wifi": 0,
                    "source": "list",
                    "mainsite": "1",
                    "mainsiteWeb": "1",
                    "content": content
                }
            }]).to_string()
        });

        let mut option = query.to_option(CryptoType::Eapi);
        option.domain = Some(CL_DOMAIN.to_string());
        let res1 = self
            .request("/api/feedback/weblog", startplay, option.clone())
            .await?;
        let res2 = self.request("/api/feedback/weblog", play, option).await?;

        Ok(ApiResponse {
            status: 200,
            body: json!({
                "code": 200,
                "data": "success",
                "details": { "startplay": res1.body, "play": res2.body }
            }),
            cookie: vec![],
        })
    }
}
