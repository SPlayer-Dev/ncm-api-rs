use super::Query;
use crate::error::Result;
/// 获取客户端歌曲下载链接 - v1 (302 重定向)
/// 对应 Node.js module/song_url_v1_302.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 获取客户端歌曲下载链接 (302 重定向)
    /// 对应 /song/url/v1/302
    ///
    /// 先尝试 download/url/v1，若无 url 则 fallback 到 player/url/v1，
    /// 最终返回带有 redirectUrl 的 302 响应。
    pub async fn song_url_v1_302(&self, query: &Query) -> Result<ApiResponse> {
        let id = query.get_or("id", "0");
        let level = query.get_or("level", "standard");

        // 第一次请求: download url
        let data = json!({
            "id": &id,
            "immerseType": "c51",
            "level": &level,
        });
        let response = self
            .request(
                "/api/song/enhance/download/url/v1",
                data,
                query.to_option(CryptoType::default()),
            )
            .await?;

        // 检查第一次请求是否有 url
        let url = response
            .body
            .get("data")
            .and_then(|d| d.get(0))
            .and_then(|item| item.get("url"))
            .and_then(|u| u.as_str())
            .map(|s| s.to_string());

        if let Some(url) = url {
            return Ok(ApiResponse {
                status: 302,
                body: json!({ "redirectUrl": url }),
                cookie: response.cookie,
            });
        }

        // Fallback: player url v1
        let mut fallback_data = json!({
            "ids": format!("[{}]", id),
            "level": &level,
            "encodeType": "flac",
        });
        if level == "sky" {
            fallback_data["immerseType"] = json!("c51");
        }
        let fallback = self
            .request(
                "/api/song/enhance/player/url/v1",
                fallback_data,
                query.to_option(CryptoType::default()),
            )
            .await?;

        let fallback_url = fallback
            .body
            .get("data")
            .and_then(|d| d.get(0))
            .and_then(|item| item.get("url"))
            .and_then(|u| u.as_str())
            .map(|s| s.to_string());

        match fallback_url {
            Some(url) => Ok(ApiResponse {
                status: 302,
                body: json!({ "redirectUrl": url }),
                cookie: fallback.cookie,
            }),
            None => Ok(fallback),
        }
    }
}
