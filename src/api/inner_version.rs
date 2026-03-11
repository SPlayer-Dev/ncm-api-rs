use crate::error::Result;
/// 内部版本号
/// 对应 Node.js module/inner_version.js
use crate::request::{ApiClient, ApiResponse};

impl ApiClient {
    /// 内部版本号（本地返回，不发起网络请求）
    /// 对应 /inner/version
    pub async fn inner_version(&self) -> Result<ApiResponse> {
        let body = serde_json::json!({
            "code": 200,
            "data": {
                "version": env!("CARGO_PKG_VERSION")
            }
        });
        Ok(ApiResponse {
            status: 200,
            body,
            cookie: vec![],
        })
    }
}
