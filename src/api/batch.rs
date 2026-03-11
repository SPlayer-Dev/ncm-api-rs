use super::Query;
use crate::error::Result;
/// 批量请求接口
/// 对应 Node.js module/batch.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 批量请求接口
    /// 对应 /batch
    pub async fn batch(&self, query: &Query) -> Result<ApiResponse> {
        let mut data = json!({});
        let map = data.as_object_mut().unwrap();
        for (key, value) in &query.params {
            if key.starts_with("/api/") {
                map.insert(
                    key.to_string(),
                    serde_json::Value::String(value.to_string()),
                );
            }
        }
        self.request("/api/batch", data, query.to_option(CryptoType::default()))
            .await
    }
}
