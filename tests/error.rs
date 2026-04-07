/// Error 类型测试
use ncm_api_rs::NcmError;

#[test]
fn test_error_from_api_auth() {
    let err = NcmError::from_api(301, "".to_string());
    assert!(matches!(err, NcmError::AuthRequired(_)));
    assert!(err.to_string().contains("需要登录"));
}

#[test]
fn test_error_from_api_auth_with_msg() {
    let err = NcmError::from_api(301, "请先登录".to_string());
    assert!(matches!(err, NcmError::AuthRequired(_)));
    assert!(err.to_string().contains("请先登录"));
}

#[test]
fn test_error_from_api_invalid_param() {
    let err = NcmError::from_api(400, "missing id".to_string());
    assert!(matches!(err, NcmError::InvalidParam(_)));
}

#[test]
fn test_error_from_api_rate_limited() {
    let err = NcmError::from_api(503, "ip高频".to_string());
    assert!(matches!(err, NcmError::RateLimited(_)));
}

#[test]
fn test_error_from_api_generic() {
    let err = NcmError::from_api(405, "method not allowed".to_string());
    assert!(matches!(err, NcmError::Api { code: 405, .. }));
}

#[test]
fn test_error_display() {
    let err = NcmError::Api {
        code: 404,
        msg: "not found".to_string(),
    };
    assert_eq!(err.to_string(), "API error (code=404): not found");

    let err = NcmError::Timeout("10s elapsed".to_string());
    assert_eq!(err.to_string(), "Request timeout: 10s elapsed");

    let err = NcmError::Crypto("bad key".to_string());
    assert_eq!(err.to_string(), "Crypto error: bad key");
}
