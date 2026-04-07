/// Query 参数提取和类型安全辅助方法测试
use ncm_api_rs::Query;

#[test]
fn test_query_builder() {
    let q = Query::new().param("id", "123").param("name", "test");

    assert_eq!(q.get("id"), Some("123"));
    assert_eq!(q.get("name"), Some("test"));
    assert_eq!(q.get("missing"), None);
}

#[test]
fn test_query_get_or() {
    let q = Query::new().param("id", "123");

    assert_eq!(q.get_or("id", "0"), "123");
    assert_eq!(q.get_or("missing", "default"), "default");
}

#[test]
fn test_query_get_i64() {
    let q = Query::new()
        .param("limit", "30")
        .param("offset", "0")
        .param("bad", "not_a_number")
        .param("empty", "");

    assert_eq!(q.get_i64("limit", 10), 30);
    assert_eq!(q.get_i64("offset", 0), 0);
    assert_eq!(q.get_i64("missing", 50), 50);
    // 解析失败返回默认值
    assert_eq!(q.get_i64("bad", 99), 99);
    assert_eq!(q.get_i64("empty", 99), 99);
}

#[test]
fn test_query_get_i64_negative() {
    let q = Query::new().param("offset", "-10");
    assert_eq!(q.get_i64("offset", 0), -10);
}

#[test]
fn test_query_get_bool() {
    let q = Query::new()
        .param("like", "true")
        .param("follow", "false")
        .param("flag1", "1")
        .param("flag0", "0")
        .param("bad", "yes");

    assert!(q.get_bool("like", false));
    assert!(!q.get_bool("follow", true));
    assert!(q.get_bool("flag1", false));
    assert!(!q.get_bool("flag0", true));
    // 无法识别的值返回默认
    assert!(q.get_bool("bad", true));
    assert!(!q.get_bool("bad", false));
    // 缺失返回默认
    assert!(q.get_bool("missing", true));
    assert!(!q.get_bool("missing", false));
}

#[test]
fn test_query_cookie() {
    let q = Query::new().cookie("MUSIC_U=xxx; __csrf=yyy");
    assert_eq!(q.cookie, Some("MUSIC_U=xxx; __csrf=yyy".to_string()));
}

#[test]
fn test_query_default() {
    let q = Query::new();
    assert!(q.params.is_empty());
    assert!(q.cookie.is_none());
    assert!(q.proxy.is_none());
    assert!(q.real_ip.is_none());
    assert!(!q.random_cn_ip);
}
