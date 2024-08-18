
use apiclient_rs::ApiResult;
use serde::{Deserialize, Serialize};

use crate::schema::{GetFeaturedModsSchema, ModSchema, PaginationSchema};

use super::V1Client;

const ENDPOINT: &str = "v1/mods";


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSearchModsParams {

    pub game_id: i32,
    pub class_id: Option<i32>,
    pub category_id: Option<i32>,
    pub category_ids: Option<String>,
    pub game_version: Option<String>,
    pub game_versions: Option<String>,
    pub search_filter: Option<String>,//ModsSearchSortField
    pub sort_field: Option<String>, //SortOrder
    pub mod_loader_type: Option<String>, //ModLoaderType
    pub mod_loader_types: Option<String>, 
    pub game_version_type_id: Option<i32>,
    pub author_id: Option<i32>,
    pub primary_author_id: Option<i32>,
    pub slug: Option<String>,
    pub index: Option<i32>,
    pub page_size: Option<i32>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GetSearchModsResponse {
    pub data: Vec<ModSchema>,
    pub pagination: PaginationSchema,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GetModResponse {
    pub data: ModSchema,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetModsResponse {
    pub data: Vec<ModSchema>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFeaturedModsResponse {
    pub data: GetFeaturedModsSchema,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetModsParams {
    pub mod_ids: Vec<i32>,
    pub filter_pc_only: Option<bool>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFeaturedModsParams {
    pub game_id: i32,
    pub excluded_mod_ids: Vec<i32>,
    pub game_version_type_id: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetModDescriptionParams {
    pub raw: Option<bool>,
    pub stripped: Option<bool>,
    pub markup: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetModDescriptionResponse {
    pub data: String,
}

impl V1Client {  
    pub async fn get_search_mods(&self, params: Option<&GetSearchModsParams>) -> ApiResult<GetSearchModsResponse> {
        let endpoint = format!("{ENDPOINT}/search");
        self.get(endpoint.as_str(), params).await
    }

    pub async fn get_mod(&self, mod_id: i32) -> ApiResult<GetModResponse> {
        let endpoint = format!("{ENDPOINT}/{}",mod_id);
        self.get_no_params(endpoint.as_str()).await
    }

    pub async fn get_mods(&self, params: &GetModsParams) -> ApiResult<GetModsResponse> {
        self.post(ENDPOINT, Some(params)).await
    }

    pub async fn get_featured_mods(&self, params: &GetFeaturedModsParams) -> ApiResult<GetFeaturedModsResponse> {
        let endpoint = format!("{ENDPOINT}/featured");
        self.post(endpoint.as_str() , Some(params)).await
    }

    pub async fn get_mod_descriptions(&self, mod_id:i32,  params: Option<&GetModDescriptionParams>) -> ApiResult<GetModDescriptionResponse> {
        let endpoint = format!("{ENDPOINT}/{}/description",mod_id);
        self.get(endpoint.as_str(), params).await
    }
}