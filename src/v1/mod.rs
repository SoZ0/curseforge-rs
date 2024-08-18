use std::sync::Arc;

use apiclient_rs::ApiResult;
use serde::{de::DeserializeOwned, Serialize};

use crate::client::CurseForgeApiClient;

pub mod games;
pub mod categories;
pub mod mods;
pub mod files;
pub mod fingerprints;

#[derive(Debug, Clone)]
pub struct V1Client {
    client: Option<Arc<CurseForgeApiClient>>,
}

impl V1Client {
    pub fn new(client: Arc<CurseForgeApiClient>) -> Self {
        Self { client: Some(client) }
    }

    pub fn new_empty() -> Self {
        Self { client: None }
    }

    pub async fn get<T, B>(&self, endpoint: &str, params: Option<&B>) -> ApiResult<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.client.as_ref().unwrap().get(endpoint, params).await
    }

    pub async fn get_no_params<T>(&self, endpoint: &str) -> ApiResult<T>
    where
        T: DeserializeOwned,
    {
        self.client.as_ref().unwrap().get_no_params(endpoint).await
    }

    pub async fn post<T, B>(&self, endpoint: &str, body: Option<&B>) -> ApiResult<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.client.as_ref().unwrap().post(endpoint, body).await
    }
}