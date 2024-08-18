use apiclient_rs::ApiResult;
use serde::{Deserialize, Serialize};

use crate::schema::{GameSchema, GameVersionTypeSchema, GameVersionsByTypeSchema, PaginationSchema};

use super::V1Client;

const ENDPOINT: &str = "v1/games";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]  
pub struct GetGamesParams {
    pub index: Option<i32>,
    pub page_size: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetGamesResponse {
    pub data: Vec<GameSchema>,
    pub pagination: PaginationSchema,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetGameResponse {
    pub data: GameSchema,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetVersionResponse {
    pub data: Vec<GameVersionsByTypeSchema>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetVersionTypesResponse {
    pub data: Vec<GameVersionTypeSchema>,
}


impl V1Client {
        pub async fn get_games(&self, params: Option<&GetGamesParams>) -> ApiResult<GetGamesResponse> {
            self.get(ENDPOINT, params).await
        }

        pub async fn get_game(&self, game_id: i32) -> ApiResult<GetGameResponse> {
            let endpoint = format!("{ENDPOINT}/{}", game_id);
            self.get_no_params(endpoint.as_str()).await
        }

        pub async fn get_game_versions(&self, game_id: i32) -> ApiResult<GetVersionResponse> {
            let endpoint = format!("{ENDPOINT}/{}/versions", game_id);
            self.get_no_params(endpoint.as_str()).await
        }

        pub async fn get_game_version_types(&self, game_id: i32) -> ApiResult<GetVersionTypesResponse> {
            let endpoint = format!("{ENDPOINT}/{}/version-types", game_id);
            self.get_no_params(endpoint.as_str()).await
        }
}