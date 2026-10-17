use super::Query;
use crate::error::Result;
/// 刷新 neapi 密钥配置
/// 对应 Node.js module/register_neapikey.js
use crate::request::{ApiClient, ApiResponse};
use serde_json::json;

impl ApiClient {
    /// 向服务端拉取 neapi 配置并刷新本地缓存（密钥本身不返回）
    /// 对应 /register/neapikey
    pub async fn register_neapikey(&self, _query: &Query) -> Result<ApiResponse> {
        let (updated, circle_time, version) = self.refresh_neapi_key().await?;
        Ok(ApiResponse {
            status: 200,
            body: json!({
                "version": version,
                "updated": updated,
                "circleTime": circle_time
            }),
            cookie: vec![],
        })
    }
}
