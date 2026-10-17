/// 错误类型定义
use crate::request::ApiResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NcmError {
    /// HTTP 请求失败（网络错误、DNS 解析失败等）
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// API 业务错误（网易云返回非 200 状态码）
    #[error("API error (code={code}): {msg}")]
    Api { code: i64, msg: String },

    /// 需要登录（API 返回 301）
    #[error("Authentication required: {0}")]
    AuthRequired(String),

    /// 参数错误（缺少必要参数或格式不正确）
    #[error("Invalid parameter: {0}")]
    InvalidParam(String),

    /// 加密/解密错误
    #[error("Crypto error: {0}")]
    Crypto(String),

    /// JSON 序列化/反序列化错误
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// 请求超时
    #[error("Request timeout: {0}")]
    Timeout(String),

    /// 触发风控/限流（API 返回 503 或 IP 高频）
    #[error("Rate limited: {0}")]
    RateLimited(String),

    /// 上游返回非 200（携带原始响应，HTTP 服务会原样返回 body，与 Node.js 版一致）
    #[error("API error (status={}): {msg}", .response.status)]
    Response {
        msg: String,
        response: Box<ApiResponse>,
    },

    /// 其他错误
    #[error("{0}")]
    Unknown(String),
}

impl NcmError {
    /// 从 API 状态码和消息构造合适的错误类型
    pub fn from_api(code: i64, msg: String) -> Self {
        match code {
            301 => NcmError::AuthRequired(if msg.is_empty() {
                "需要登录".to_string()
            } else {
                msg
            }),
            400 => NcmError::InvalidParam(msg),
            503 => NcmError::RateLimited(msg),
            _ => NcmError::Api { code, msg },
        }
    }
}

impl NcmError {
    /// 业务状态码（上游返回的 code，或按错误类型推断）
    pub fn status_code(&self) -> Option<i64> {
        match self {
            NcmError::Response { response, .. } => Some(response.status),
            NcmError::Api { code, .. } => Some(*code),
            NcmError::AuthRequired(_) => Some(301),
            NcmError::InvalidParam(_) => Some(400),
            NcmError::RateLimited(_) => Some(503),
            NcmError::Timeout(_) => Some(504),
            _ => None,
        }
    }

    /// 是否需要登录（code 301）
    pub fn is_auth_required(&self) -> bool {
        self.status_code() == Some(301)
    }

    /// 上游原始响应（仅 `Response` 变体）
    pub fn response(&self) -> Option<&ApiResponse> {
        match self {
            NcmError::Response { response, .. } => Some(response),
            _ => None,
        }
    }
}

pub type Result<T> = std::result::Result<T, NcmError>;
