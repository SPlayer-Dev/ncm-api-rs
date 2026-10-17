use super::Query;
use crate::error::Result;
/// 获取当前登录用户的全部动态
/// 对应 Node.js module/user_event_all.js
use crate::request::{ApiClient, ApiResponse};
use serde_json::{json, Value};
use std::collections::HashSet;

const PAGE_SIZE: &str = "100";
const MAX_PAGES: usize = 1000;

fn error_response(message: String, cookie: Vec<String>) -> ApiResponse {
    ApiResponse {
        status: 502,
        body: json!({ "code": 502, "message": message }),
        cookie,
    }
}

impl ApiClient {
    /// 自动翻页获取当前登录用户可枚举的全部动态（按 id 去重）
    /// 对应 /user/event/all
    pub async fn user_event_all(&self, query: &Query) -> Result<ApiResponse> {
        let account = self.user_account(query).await?;
        let uid = account
            .body
            .pointer("/account/id")
            .or_else(|| account.body.pointer("/profile/userId"))
            .filter(|v| !v.is_null())
            .map(|v| v.to_string());
        let Some(uid) = uid else {
            return Ok(ApiResponse {
                status: 401,
                body: json!({ "code": 401, "message": "A valid login cookie is required" }),
                cookie: account.cookie,
            });
        };

        let mut cookies = account.cookie;
        let mut events: Vec<Value> = Vec::new();
        let mut event_ids = HashSet::new();
        let mut cursors = HashSet::new();
        let mut lasttime = Value::from(-1);
        let mut size: Option<i64> = None;
        let mut page_count = 0;

        loop {
            page_count += 1;
            let mut q = query.clone();
            q.params.insert("uid".into(), uid.clone());
            q.params.insert("limit".into(), PAGE_SIZE.into());
            q.params.insert(
                "lasttime".into(),
                lasttime
                    .as_str()
                    .map(str::to_string)
                    .unwrap_or_else(|| lasttime.to_string()),
            );
            let page = self.user_event(&q).await?;
            cookies.extend(page.cookie);
            let body = page.body;

            if page_count == 1 {
                size = body
                    .get("size")
                    .and_then(|s| s.as_i64().or_else(|| s.as_str()?.parse().ok()));
            }
            for ev in body
                .get("events")
                .and_then(|e| e.as_array())
                .into_iter()
                .flatten()
            {
                match ev.get("id").filter(|v| !v.is_null()) {
                    Some(id) if !event_ids.insert(id.to_string()) => {}
                    _ => events.push(ev.clone()),
                }
            }

            let more = body.get("more").and_then(|m| m.as_bool()).unwrap_or(false);
            lasttime = body.get("lasttime").cloned().unwrap_or(Value::Null);
            if !more {
                break;
            }
            let cursor = match &lasttime {
                Value::Null => String::new(),
                Value::String(s) => s.clone(),
                v => v.to_string(),
            };
            if cursor.is_empty() || !cursors.insert(cursor) {
                return Ok(error_response(
                    "Upstream event pagination cursor stalled".into(),
                    cookies,
                ));
            }
            if page_count >= MAX_PAGES {
                return Ok(error_response(
                    format!("Upstream event pagination exceeded {} pages", MAX_PAGES),
                    cookies,
                ));
            }
        }

        let retrieved = events.len() as i64;
        Ok(ApiResponse {
            status: 200,
            body: json!({
                "code": 200,
                "events": events,
                "size": size,
                "retrievedCount": retrieved,
                "unavailableCount": size.map(|s| (s - retrieved).max(0)),
                "sizeMismatch": size.map(|s| s != retrieved),
                "pageCount": page_count,
                "more": false,
                "lasttime": lasttime
            }),
            cookie: cookies,
        })
    }
}
