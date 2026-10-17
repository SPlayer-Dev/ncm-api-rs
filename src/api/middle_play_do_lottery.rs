use super::Query;
use crate::error::Result;
/// 云小编每日抽奖
/// 对应 Node.js module/middle_play_do_lottery.js
use crate::request::{ApiClient, ApiResponse, CheckToken, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云小编每日抽奖
    /// 对应 /middle/play/do/lottery
    ///
    /// activityId 默认 6501202，drawCount 默认 1
    pub async fn middle_play_do_lottery(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "activityId": query.get_or("activityId", "6501202"),
            "drawCount": query.get_or("drawCount", "1")
        });
        self.request(
            "/api/middle/play/do/lottery",
            data,
            query.to_option_token(CryptoType::Eapi, CheckToken::Static),
        )
        .await
    }
}
