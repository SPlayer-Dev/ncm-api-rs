use super::Query;
use crate::error::Result;
/// 云盘上传
/// 对应 Node.js module/cloud.js
///
/// 注意：Node.js 版本的 cloud.js 是一个复杂的多步骤上传流程
/// （检查 -> 解析元数据 -> 获取 token -> 上传 -> 提交信息 -> 发布）。
/// Rust SDK 将其拆分为独立的步骤接口（cloud_upload_token、cloud_upload_complete 等），
/// 此方法仅实现第一步：上传检查。
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云盘上传检查
    /// 对应 /cloud（上传检查步骤）
    pub async fn cloud_upload_check(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "bitrate": query.get_or("bitrate", "999000"),
            "ext": query.get_or("ext", ""),
            "length": query.get_or("length", "0").parse::<i64>().unwrap_or(0),
            "md5": query.get_or("md5", ""),
            "songId": query.get_or("songId", "0"),
            "version": 1
        });
        self.request(
            "/api/cloud/upload/check",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }

    /// 云盘上传信息提交
    /// 对应 /cloud（信息提交步骤）
    pub async fn cloud_upload_info(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "md5": query.get_or("md5", ""),
            "songid": query.get_or("songId", ""),
            "filename": query.get_or("filename", ""),
            "song": query.get_or("song", ""),
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

    /// 云盘歌曲发布
    /// 对应 /cloud（发布步骤）
    pub async fn cloud_publish(&self, query: &Query) -> Result<ApiResponse> {
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
