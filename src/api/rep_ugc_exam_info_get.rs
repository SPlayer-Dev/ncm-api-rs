use super::Query;
use crate::error::{NcmError, Result};
/// 云小编考试状态
/// 对应 Node.js module/rep_ugc_exam_info_get.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云小编考试状态
    /// 对应 /rep/ugc/exam/info/get
    ///
    /// examType: musicalStyleEnter / languageEnter / oriSingerEnter / emotionEnter
    pub async fn rep_ugc_exam_info_get(&self, query: &Query) -> Result<ApiResponse> {
        if query.get("examType").is_none_or(str::is_empty) {
            return Err(NcmError::InvalidParam("参数不足: examType".into()));
        }
        let data = json!({
            "examType": query.get_or("examType", "")
        });
        self.request(
            "/api/rep/ugc/exam/info/get",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
