use apiclient_rs::ApiResult;
use serde::{Deserialize, Serialize};

use crate::schema::{FingerprintsFuzzyMatchesResultSchema, FingerprintsMatchesResultSchema, FolderFingerprintSchema};

use super::V1Client;

const ENDPOINT: &str = "v1/minecraft";


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMinecraftVersionsParams {
    pub sort_descending: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponseOfListOfMinecraftVersionsSchema
{
    pub data: Vec<MinecraftGameVersionSchema>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponseOfMinecraftGameVersion
{
    pub data: MinecraftGameVersionSchema,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMinecraftModLoaderParams {
    pub version: Option<String>,
    pub include_all: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponseOfListOfMinecraftModLoaderIndex {
    pub data: Vec<MinecraftModLoaderIndexSchema>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponseOfMinecraftModLoaderVersion {
    pub data: MinecraftModLoaderVersionSchema,
}



impl V1Client {  
    pub async fn get_minecraft_versions(&self, params: Option<&GetMinecraftVersionsParams>) -> ApiResult<ApiResponseOfListOfMinecraftVersionsSchema> {
        let endpoint = format!("{ENDPOINT}/versions");
        self.get(endpoint.as_str(), Some(params)).await
    }

    pub async fn get_specific_minecraft_versions(&self, game_version_string: &String) -> ApiResult<ApiResponseOfMinecraftGameVersion> {
        let endpoint = format!("{ENDPOINT}/version/{}",game_version_string);
        self.get_no_params(endpoint.as_str()).await
    }

    pub async fn get_minecraft_modloader(&self, game_id: i32, params: Option<&GetMinecraftModLoaderParams>) -> ApiResult<ApiResponseOfListOfMinecraftModLoaderIndex> {
        let endpoint = format!("{ENDPOINT}/modloader");
        self.get(endpoint.as_str(), Some(params)).await
    }

    pub async fn get_minecraft_modloader(&self, mod_loader_name: String) -> ApiResult<ApiResponseOfMinecraftModLoaderVersion> {
        let endpoint = format!("{ENDPOINT}/modloader/{}",mod_loader_name);
        self.get_no_params(endpoint.as_str()).await
    }
}

