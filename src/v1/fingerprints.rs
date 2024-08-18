use apiclient_rs::ApiResult;
use serde::{Deserialize, Serialize};

use crate::schema::{FingerprintsFuzzyMatchesResultSchema, FingerprintsMatchesResultSchema, FolderFingerprintSchema};

use super::V1Client;

const ENDPOINT: &str = "v1/fingerprints";


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFingerprintMatchesRequestParams {
    pub fingerprints: Vec<i32>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFingerprintMatchesResponse
{
    pub data: FingerprintsMatchesResultSchema,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFingerprintFuzzyMatchesRequestParams {
    pub game_id: i32,
    pub fingerprints: Vec<FolderFingerprintSchema>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFingerprintFuzzyMatchesResponse
{
    pub data: FingerprintsFuzzyMatchesResultSchema,
}

impl V1Client {  
    pub async fn get_fingerpints_matches_by_game_id(&self, game_id: i32, params: &GetFingerprintMatchesRequestParams) -> ApiResult<GetFingerprintMatchesResponse> {
        let endpoint = format!("{ENDPOINT}/{}",game_id);
        self.post(endpoint.as_str(), Some(params)).await
    }

    pub async fn get_fingerpints_matches(&self, params: &GetFingerprintMatchesRequestParams) -> ApiResult<GetFingerprintMatchesResponse> {
        self.post(ENDPOINT, Some(params)).await
    }

    pub async fn get_fingerpints_fuzzy_matches_by_game_id(&self, game_id: i32, params: &GetFingerprintFuzzyMatchesRequestParams) -> ApiResult<GetFingerprintFuzzyMatchesResponse> {
        let endpoint = format!("{ENDPOINT}/fuzzy/{}",game_id);
        self.post(endpoint.as_str(), Some(params)).await
    }

    pub async fn get_fingerpints_fuzzy_matches(&self, params: &GetFingerprintFuzzyMatchesRequestParams) -> ApiResult<GetFingerprintFuzzyMatchesResponse> {
        let endpoint = format!("{ENDPOINT}/fuzzy");
        self.post(endpoint.as_str(), Some(params)).await
    }
}

