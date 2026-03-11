use super::Query;
use crate::error::Result;
/// 通用 API 代理接口
/// 对应 Node.js module/api.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 通用 API 代理接口，允许调用任意 API 路径
    /// 对应 /api
    pub async fn api(&self, query: &Query) -> Result<ApiResponse> {
        let uri = query.get_or("uri", "");
        let data_str = query.get_or("data", "{}");
        let data: serde_json::Value = serde_json::from_str(&data_str).unwrap_or_else(|_| json!({}));
        let crypto_str = query.get_or("crypto", "");
        let crypto = CryptoType::from(crypto_str.as_str());
        self.request(&uri, data, query.to_option(crypto)).await
    }
}
