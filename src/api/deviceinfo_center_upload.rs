use super::Query;
use crate::error::{NcmError, Result};
/// 上报设备中心设备名称
/// 对应 Node.js module/deviceinfo_center_upload.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 上报设备中心设备名称（「登录设备管理」中显示的名称）
    /// 对应 /deviceinfo/center/upload
    ///
    /// 参数：deviceName（或 name）
    pub async fn deviceinfo_center_upload(&self, query: &Query) -> Result<ApiResponse> {
        let device_name = query
            .get("deviceName")
            .or_else(|| query.get("name"))
            .unwrap_or("")
            .trim()
            .to_string();
        if device_name.is_empty() {
            return Err(NcmError::InvalidParam("缺少必要参数: deviceName".into()));
        }
        self.request(
            "/api/deviceinfo/center/upload",
            json!({ "deviceName": device_name }),
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
