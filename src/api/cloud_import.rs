use super::Query;
use crate::error::Result;
/// 云盘导入歌曲
/// 对应 Node.js module/cloud_import.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云盘导入歌曲 - 检查
    /// 对应 /cloud/import（检查步骤）
    pub async fn cloud_import_check(&self, query: &Query) -> Result<ApiResponse> {
        let songs = json!([{
            "md5": query.get_or("md5", ""),
            "songId": query.get_or("id", "-2").parse::<i64>().unwrap_or(-2),
            "bitrate": query.get_or("bitrate", ""),
            "fileSize": query.get_or("fileSize", "")
        }]);
        let data = json!({
            "uploadType": 0,
            "songs": songs.to_string()
        });
        self.request(
            "/api/cloud/upload/check/v2",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }

    /// 云盘导入歌曲（秒传：先检查拿到 songId，再导入）
    /// 对应 /cloud/import
    ///
    /// 参数：id（默认 -2）, md5, fileSize, bitrate, song, artist, album, fileType
    pub async fn cloud_import(&self, query: &Query) -> Result<ApiResponse> {
        // data[0].upload：0 可导入，1 已在云盘，2 不能导入
        let check = self.cloud_import_check(query).await?;
        let song_id = match check.body.pointer("/data/0/songId") {
            Some(serde_json::Value::String(s)) => s.clone(),
            Some(v) if !v.is_null() => v.to_string(),
            _ => String::new(),
        };
        let song_name = query.get_or("song", "");
        let file_type = query.get_or("fileType", "mp3");
        let songs = json!([{
            "songId": song_id,
            "bitrate": query.get_or("bitrate", ""),
            "song": &song_name,
            "artist": query.get_or("artist", "未知"),
            "album": query.get_or("album", "未知"),
            "fileName": format!("{}.{}", song_name, file_type)
        }]);
        let data = json!({
            "uploadType": 0,
            "songs": songs.to_string()
        });
        self.request(
            "/api/cloud/user/song/import",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
