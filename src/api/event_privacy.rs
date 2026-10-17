use super::Query;
use crate::error::Result;
/// 修改本人动态的可见权限
/// 对应 Node.js module/event_privacy.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 修改本人动态的可见权限
    /// 对应 /event/privacy
    ///
    /// 参数：evId 动态 id, privacy 取值 0 / 1 / 2 / 6
    pub async fn event_privacy(&self, query: &Query) -> Result<ApiResponse> {
        let event_id = query.get_or("evId", "").trim().to_string();
        let privacy = query.get_or("privacy", "").trim().parse::<i64>().ok();
        let privacy = match privacy {
            Some(p @ (0 | 1 | 2 | 6)) if !event_id.is_empty() => p,
            _ => {
                return Ok(ApiResponse {
                    status: 400,
                    body: json!({
                        "code": 400,
                        "message": "evId is required and privacy must be one of 0, 1, 2, 6"
                    }),
                    cookie: vec![],
                })
            }
        };
        let data = json!({
            "eventId": event_id,
            "privacy": privacy
        });
        self.request(
            "/api/event/privacy/op",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
