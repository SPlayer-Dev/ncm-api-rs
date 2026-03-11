use super::Query;
use crate::error::Result;
/// 云盘上传 Token
/// 对应 Node.js module/cloud_upload_token.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 获取云盘上传 Token 及上传地址
    /// 对应 /cloud/upload/token
    ///
    /// 此方法执行上传检查和 token 分配两步。
    /// 完整的上传流程（包括 LBS 查询）需要调用方自行处理。
    pub async fn cloud_upload_token_check(&self, query: &Query) -> Result<ApiResponse> {
        let bitrate = query.get_or("bitrate", "999000");
        let data = json!({
            "bitrate": &bitrate,
            "ext": "",
            "length": query.get_or("fileSize", "0").parse::<i64>().unwrap_or(0),
            "md5": query.get_or("md5", ""),
            "songId": "0",
            "version": 1
        });
        self.request(
            "/api/cloud/upload/check",
            data,
            query.to_option(CryptoType::default()),
        )
        .await
    }

    /// 分配 NOS 上传 Token
    /// 对应 /cloud/upload/token（token 分配步骤）
    pub async fn cloud_upload_token_alloc(&self, query: &Query) -> Result<ApiResponse> {
        let filename = query.get_or("filename", "");
        let ext = if filename.contains('.') {
            filename.rsplit('.').next().unwrap_or("mp3").to_string()
        } else {
            "mp3".to_string()
        };
        let clean_name = filename
            .rsplit_once('.')
            .map(|(name, _)| name)
            .unwrap_or(&filename)
            .replace(' ', "")
            .replace('.', "_");
        let data = json!({
            "bucket": "jd-musicrep-privatecloud-audio-public",
            "ext": ext,
            "filename": clean_name,
            "local": false,
            "nos_product": 3,
            "type": "audio",
            "md5": query.get_or("md5", "")
        });
        self.request(
            "/api/nos/token/alloc",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
