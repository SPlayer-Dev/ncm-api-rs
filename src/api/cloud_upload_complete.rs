use super::Query;
use crate::error::Result;
/// 云盘上传完成
/// 对应 Node.js module/cloud_upload_complete.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云盘上传完成 - 提交云盘信息
    /// 对应 /cloud/upload/complete（信息提交步骤）
    pub async fn cloud_upload_complete_info(&self, query: &Query) -> Result<ApiResponse> {
        let filename = query.get_or("filename", "");
        let song = if let Some(s) = query.get("song") {
            s.to_string()
        } else {
            filename
                .rsplit_once('.')
                .map(|(name, _)| name)
                .unwrap_or(&filename)
                .to_string()
        };
        let data = json!({
            "md5": query.get_or("md5", ""),
            "songid": query.get_or("songId", ""),
            "filename": &filename,
            "song": song,
            "album": query.get_or("album", "未知专辑"),
            "artist": query.get_or("artist", "未知艺术家"),
            "bitrate": query.get_or("bitrate", "999000"),
            "resourceId": query.get_or("resourceId", "")
        });
        self.request(
            "/api/upload/cloud/info/v2",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }

    /// 云盘上传完成 - 发布
    /// 对应 /cloud/upload/complete（发布步骤）
    pub async fn cloud_upload_complete_pub(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "songid": query.get_or("songId", "")
        });
        self.request(
            "/api/cloud/pub/v2",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }
}
