use super::Query;
use crate::error::Result;
/// 歌单导入 - 元数据/文字/链接导入
/// 对应 Node.js module/playlist_import_name_task_create.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 歌单导入 - 元数据/文字/链接导入
    /// 对应 /playlist/import/name/task/create
    pub async fn playlist_import_name_task_create(&self, query: &Query) -> Result<ApiResponse> {
        let import_star_playlist = query.get_or("importStarPlaylist", "false");

        let data = if let Some(local) = query.get("local") {
            // 元数据导入
            let local_parsed: Vec<serde_json::Value> =
                serde_json::from_str(local).unwrap_or_default();
            let multi_songs: Vec<serde_json::Value> = local_parsed
                .iter()
                .map(|e| {
                    json!({
                        "songName": e.get("name").and_then(|v| v.as_str()).unwrap_or(""),
                        "artistName": e.get("artist").and_then(|v| v.as_str()).unwrap_or(""),
                        "albumName": e.get("album").and_then(|v| v.as_str()).unwrap_or("")
                    })
                })
                .collect();
            json!({
                "importStarPlaylist": import_star_playlist,
                "multiSongs": serde_json::to_string(&multi_songs).unwrap_or_default()
            })
        } else {
            let playlist_name = query.get_or("playlistName", "");

            let songs = if let Some(text) = query.get("text") {
                // 文字导入
                let url = format!("rpc://playlist/import?text={}", text);
                serde_json::to_string(&json!([{
                    "name": playlist_name,
                    "type": "",
                    "url": url
                }]))
                .unwrap_or_default()
            } else if let Some(link) = query.get("link") {
                // 链接导入
                let links: Vec<String> = serde_json::from_str(link).unwrap_or_default();
                let entries: Vec<serde_json::Value> = links
                    .iter()
                    .map(|e| {
                        json!({
                            "name": playlist_name,
                            "type": "",
                            "url": e
                        })
                    })
                    .collect();
                serde_json::to_string(&entries).unwrap_or_default()
            } else {
                String::new()
            };

            json!({
                "importStarPlaylist": import_star_playlist,
                "playlistName": playlist_name,
                "taskIdForLog": "",
                "songs": songs
            })
        };

        self.request(
            "/api/playlist/import/name/task/create",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
