use super::Query;
use crate::error::Result;
/// 黑胶乐签打卡
/// 对应 Node.js module/vip_sign.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 黑胶乐签打卡
    /// 对应 /vip/sign
    pub async fn vip_sign(&self, query: &Query) -> Result<ApiResponse> {
        let task_sign = self
            .request(
                "/api/vip-center-bff/task/sign",
                json!({}),
                query.to_option(CryptoType::Weapi),
            )
            .await?;
        let checkin_detail = self
            .request(
                "/api/vipnewcenter/app/level/user/checkin/history/detail",
                json!({
                    "signDayTime": chrono::Utc::now().timestamp_millis(),
                    "type": 1
                }),
                query.to_option(CryptoType::Eapi),
            )
            .await?;

        // 两个接口都返回 code=200 即视为打卡成功
        let ok = |v: &serde_json::Value| {
            v.get("code")
                .and_then(|c| c.as_i64().or_else(|| c.as_str()?.parse().ok()))
                == Some(200)
        };
        let signed = ok(&task_sign.body) && ok(&checkin_detail.body);

        Ok(ApiResponse {
            status: 200,
            body: json!({
                "code": 200,
                "taskSign": task_sign.body,
                "checkinDetail": checkin_detail.body,
                "signed": signed,
                "message": if signed { "黑胶乐签打卡成功" } else { "黑胶乐签打卡失败" }
            }),
            cookie: task_sign.cookie,
        })
    }
}
