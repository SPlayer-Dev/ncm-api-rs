use super::Query;
use crate::error::Result;
/// 注册 xeapi 公钥
/// 对应 Node.js module/register_xeapikey.js
use crate::request::{ApiClient, ApiResponse};

impl ApiClient {
    /// 重新向服务端申请 xeapi 公钥并刷新本地缓存
    /// 对应 /register/xeapikey
    pub async fn register_xeapikey(&self, _query: &Query) -> Result<ApiResponse> {
        let pk = self.refresh_xeapi_public_key().await?;
        let mut body = serde_json::to_value(&pk)?;
        body["deviceId"] = self.device_id().into();
        Ok(ApiResponse {
            status: 200,
            body,
            cookie: vec![],
        })
    }
}
