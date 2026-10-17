use super::Query;
use crate::error::Result;
/// 获取广告
/// 对应 Node.js module/ad_get.js
use crate::request::{ApiClient, ApiResponse, CheckToken, CryptoType};
use serde_json::{json, Value};

/// 字段可能是 JSON 字符串也可能是对象
fn as_object(v: Option<&Value>) -> Option<Value> {
    match v? {
        Value::String(s) => serde_json::from_str(s).ok(),
        other => Some(other.clone()),
    }
}

impl ApiClient {
    /// 获取广告，并在 extra.reqId 中提取广告 req_id
    /// 对应 /ad/get
    pub async fn ad_get(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "type_ids": query.get_or("type_ids", r#"["400002_0"]"#)
        });
        let res = self
            .request(
                "/api/ad/get",
                data,
                query.to_option_token(CryptoType::Xeapi, CheckToken::V3),
            )
            .await?;
        let raw = &res.body;
        let ad = raw
            .get("ads")
            .and_then(|a| a.as_object())
            .and_then(|m| m.values().next());
        let str_at = |v: Option<Value>, path: &[&str]| -> Option<String> {
            let mut cur = v?;
            for k in path {
                cur = cur.get(k)?.clone();
            }
            cur.as_str().filter(|s| !s.is_empty()).map(str::to_string)
        };
        // 逆向 v9.5.61：优先 adExtMap.req_id，兜底 adLogId.requestId / reqId / extJson.contextInfo.req_id
        let req_id = ad
            .and_then(|ad| {
                str_at(as_object(ad.get("adExtMap")), &["req_id"])
                    .or_else(|| str_at(Some(ad.clone()), &["adLogId", "requestId"]))
                    .or_else(|| str_at(Some(ad.clone()), &["reqId"]))
                    .or_else(|| str_at(as_object(ad.get("extJson")), &["contextInfo", "req_id"]))
            })
            .unwrap_or_default();

        Ok(ApiResponse {
            status: 200,
            body: json!({
                "code": 200,
                "ads": raw.get("ads").cloned().unwrap_or(Value::Null),
                "message": raw.get("message").cloned().unwrap_or(Value::Null),
                "extra": { "reqId": req_id }
            }),
            cookie: res.cookie,
        })
    }
}
