
use apiclient_rs::ApiResult;
use serde::{Deserialize, Serialize};

use crate::schema::GameVersionsByType2Schema;

use super::V2Client;

const ENDPOINT: &str = "v2/games";

#[derive(Debug, Serialize, Deserialize)]
pub struct GetVersionsResponse {
    pub data: Vec<GameVersionsByType2Schema>,
}


impl V2Client {  
    pub async fn get_game_versions(&self, game_id: i32) -> ApiResult<GetVersionsResponse> {
        let endpoint = format!("{ENDPOINT}/{}/versions", game_id);
        self.get_no_params(endpoint.as_str()).await
    }
}