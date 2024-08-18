use apiclient_rs::ApiResult;
use serde::{Deserialize, Serialize};

use crate::schema::CategorySchema;

use super::V1Client;

const ENDPOINT: &str = "v1/categories";


#[derive(Debug, Serialize, Deserialize)]
pub struct GetCategoriesParams {
    pub game_id: i32,
    pub class_id: Option<i32>,
    pub classes_only: Option<bool>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GetCategoriesResponse {
    pub data: Vec<CategorySchema>,
}


impl V1Client {  
    pub async fn get_categories(&self, params: Option<&GetCategoriesParams>) -> ApiResult<GetCategoriesResponse> {
        self.get(ENDPOINT, params).await
    }
}