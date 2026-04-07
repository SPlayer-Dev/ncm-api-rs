/// 集成测试 - 验证关键接口能正常请求网易云 API
///
/// 注意：这些测试会发起真实网络请求，CI 环境中可能需要跳过
use ncm_api_rs::{create_client, Query};

#[tokio::test]
async fn test_banner() {
    let client = create_client(None);
    let query = Query::new();
    let result = client.banner(&query).await;
    assert!(result.is_ok());
    let resp = result.unwrap();
    assert_eq!(resp.status, 200);
    assert!(resp.body.get("banners").is_some());
}

#[tokio::test]
async fn test_search() {
    let client = create_client(None);
    let query = Query::new()
        .param("keywords", "海阔天空")
        .param("limit", "1");
    let result = client.cloudsearch(&query).await;
    assert!(result.is_ok());
    let resp = result.unwrap();
    assert_eq!(resp.status, 200);
}

#[tokio::test]
async fn test_song_detail() {
    let client = create_client(None);
    // 347230 = 海阔天空 - Beyond
    let query = Query::new().param("ids", "347230");
    let result = client.song_detail(&query).await;
    assert!(result.is_ok());
    let resp = result.unwrap();
    assert_eq!(resp.status, 200);
    assert!(resp.body.get("songs").is_some());
}

#[tokio::test]
async fn test_song_detail_multiple_ids() {
    let client = create_client(None);
    let query = Query::new().param("ids", "347230,186016");
    let result = client.song_detail(&query).await;
    assert!(result.is_ok());
    let resp = result.unwrap();
    let songs = resp.body.get("songs").and_then(|s| s.as_array());
    assert!(songs.is_some());
    assert_eq!(songs.unwrap().len(), 2);
}

#[tokio::test]
async fn test_toplist() {
    let client = create_client(None);
    let query = Query::new();
    let result = client.toplist(&query).await;
    assert!(result.is_ok());
    let resp = result.unwrap();
    assert_eq!(resp.status, 200);
    assert!(resp.body.get("list").is_some());
}

#[tokio::test]
async fn test_search_hot_detail() {
    let client = create_client(None);
    let query = Query::new();
    let result = client.search_hot_detail(&query).await;
    assert!(result.is_ok());
    let resp = result.unwrap();
    assert_eq!(resp.status, 200);
}

#[tokio::test]
async fn test_login_status_without_cookie() {
    let client = create_client(None);
    let query = Query::new();
    // 未登录应该正常返回（不会报错）
    let result = client.login_status(&query).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_countries_code_list() {
    let client = create_client(None);
    let query = Query::new();
    let result = client.countries_code_list(&query).await;
    assert!(result.is_ok());
    let resp = result.unwrap();
    assert_eq!(resp.status, 200);
}
