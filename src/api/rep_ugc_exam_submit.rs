use super::Query;
use crate::error::{NcmError, Result};
/// 云小编考试提交
/// 对应 Node.js module/rep_ugc_exam_submit.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云小编考试提交
    /// 对应 /rep/ugc/exam/submit
    ///
    /// answer: A 对, B 错
    pub async fn rep_ugc_exam_submit(&self, query: &Query) -> Result<ApiResponse> {
        if query.get("examType").is_none_or(str::is_empty) {
            return Err(NcmError::InvalidParam("参数不足: examType".into()));
        }
        if query.get("taskId").is_none_or(str::is_empty) {
            return Err(NcmError::InvalidParam("参数不足: taskId".into()));
        }
        if query.get("questionId").is_none_or(str::is_empty) {
            return Err(NcmError::InvalidParam("参数不足: questionId".into()));
        }
        if query.get("answer").is_none_or(str::is_empty) {
            return Err(NcmError::InvalidParam("参数不足: answer".into()));
        }
        let data = json!({
            "examType": query.get_or("examType", ""),
            "taskId": query.get_or("taskId", ""),
            "questionId": query.get_or("questionId", ""),
            "answer": query.get_or("answer", "")
        });
        self.request(
            "/api/rep/ugc/exam/submit",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
