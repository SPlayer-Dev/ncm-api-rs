/// 使用示例：搜索歌曲并获取播放链接
///
/// 运行: cargo run --example basic
use ncm_api_rs::{create_client, Query};

#[tokio::main]
async fn main() {
    let client = create_client(None);

    // 搜索歌曲
    println!("=== 搜索歌曲 ===");
    let query = Query::new()
        .param("keywords", "晴天 周杰伦")
        .param("type", "1")
        .param("limit", "5");
    match client.cloudsearch(&query).await {
        Ok(resp) => {
            println!("状态: {}", resp.status);
            if let Some(songs) = resp.body["result"]["songs"].as_array() {
                for song in songs {
                    println!(
                        "  {} - {} (ID: {})",
                        song["name"].as_str().unwrap_or(""),
                        song["ar"][0]["name"].as_str().unwrap_or(""),
                        song["id"].as_i64().unwrap_or(0)
                    );
                }
            }
        }
        Err(e) => eprintln!("搜索失败: {}", e),
    }

    // 获取歌曲详情
    println!("\n=== 歌曲详情 ===");
    let query = Query::new().param("ids", "186016");
    match client.song_detail(&query).await {
        Ok(resp) => {
            if let Some(songs) = resp.body["songs"].as_array() {
                for song in songs {
                    println!(
                        "  {} - {} (时长: {}ms)",
                        song["name"].as_str().unwrap_or(""),
                        song["ar"][0]["name"].as_str().unwrap_or(""),
                        song["dt"].as_i64().unwrap_or(0)
                    );
                }
            }
        }
        Err(e) => eprintln!("获取详情失败: {}", e),
    }

    // 获取歌词
    println!("\n=== 歌词 ===");
    let query = Query::new().param("id", "186016");
    match client.lyric(&query).await {
        Ok(resp) => {
            if let Some(lyric) = resp.body["lrc"]["lyric"].as_str() {
                // 只显示前 5 行
                for line in lyric.lines().take(5) {
                    println!("  {}", line);
                }
                println!("  ...");
            }
        }
        Err(e) => eprintln!("获取歌词失败: {}", e),
    }

    // 获取播放链接
    println!("\n=== 播放链接 ===");
    let query = Query::new()
        .param("id", "186016")
        .param("level", "standard");
    match client.song_url_v1(&query).await {
        Ok(resp) => {
            if let Some(data) = resp.body["data"].as_array() {
                for item in data {
                    println!(
                        "  URL: {}",
                        item["url"].as_str().unwrap_or("需要登录或 VIP")
                    );
                    println!(
                        "  码率: {} kbps, 大小: {} bytes",
                        item["br"].as_i64().unwrap_or(0) / 1000,
                        item["size"].as_i64().unwrap_or(0)
                    );
                }
            }
        }
        Err(e) => eprintln!("获取链接失败: {}", e),
    }
}
