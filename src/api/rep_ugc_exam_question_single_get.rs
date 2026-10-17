use super::Query;
use crate::error::{NcmError, Result};
/// 云小编考试取题
/// 对应 Node.js module/rep_ugc_exam_question_single_get.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云小编考试取题
    /// 对应 /rep/ugc/exam/question/single/get
    pub async fn rep_ugc_exam_question_single_get(&self, query: &Query) -> Result<ApiResponse> {
        if query.get("examType").is_none_or(str::is_empty) {
            return Err(NcmError::InvalidParam("参数不足: examType".into()));
        }
        if query.get("taskId").is_none_or(str::is_empty) {
            return Err(NcmError::InvalidParam("参数不足: taskId".into()));
        }
        let data = json!({
            "examType": query.get_or("examType", ""),
            "taskId": query.get_or("taskId", "")
        });
        self.request(
            "/api/rep/ugc/exam/question/single/get",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
