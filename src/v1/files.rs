
use apiclient_rs::ApiResult;
use serde::{Deserialize, Serialize};

use crate::schema::{FileSchema, ModLoaderType, PaginationSchema};

use super::V1Client;

const ENDPOINT: &str = "v1/mods";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetModFileResponse {
    pub data: FileSchema,
}   

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetModFilesParams {
    pub game_version: Option<String>,
    pub mod_loader_type: Option<ModLoaderType>,
    pub game_version_type_id: Option<i32>,
    pub index: Option<i32>,
    pub page_size: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetModFilesResponse {
    pub data: Vec<FileSchema>,
    pub pagination: PaginationSchema,
}   

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFilesParams {
    pub file_ids: Vec<i32>,
}                   


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFilesResponse {
    pub data: Vec<FileSchema>,
}                   

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetModFileChangelogResponse
{
    pub data: String,
}  

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetModFileDownloadUrl
{
    pub data: String,
}  

impl V1Client {  
    pub async fn get_mod_file(&self, mod_id: i32, file_id:i32) -> ApiResult<GetModFileResponse> {
        let endpoint = format!("{ENDPOINT}/{}/files/{}", mod_id, file_id);
        self.get_no_params(endpoint.as_str()).await
    }

    pub async fn get_mod_files(&self, mod_id: i32, params: Option<&GetModFilesParams>) -> ApiResult<GetModFilesResponse> {
        let endpoint = format!("{ENDPOINT}/{}/files",mod_id);
        self.post(endpoint.as_str(), params).await
    }   

    pub async fn get_files(&self, params: &GetFilesParams) -> ApiResult<GetFilesResponse> {
        self.post(ENDPOINT, Some(params)).await
    }

    pub async fn get_mod_file_changelog(&self, mod_id: i32, file_id:i32) -> ApiResult<GetModFileChangelogResponse> {
        let endpoint = format!("{ENDPOINT}/{}/files/{}/changelog",mod_id, file_id);
        self.get_no_params(endpoint.as_str()).await
    }

    pub async fn get_mod_file_download_url(&self, mod_id: i32, file_id:i32) -> ApiResult<GetModFileDownloadUrl> {
        let endpoint = format!("{ENDPOINT}/{}/files/{}/download-url",mod_id, file_id);
        self.get_no_params(endpoint.as_str()).await
    }
}   