use super::Query;
use crate::error::Result;
/// 云小编抽奖剩余次数查询
/// 对应 Node.js module/middle_play_lottery_remain_chance.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 云小编抽奖剩余次数查询
    /// 对应 /middle/play/lottery/remain/chance
    pub async fn middle_play_lottery_remain_chance(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "activityId": query.get_or("activityId", "6501202")
        });
        self.request(
            "/api/middle/play/lottery/remain/chance",
            data,
            query.to_option(CryptoType::Eapi),
        )
        .await
    }
}
