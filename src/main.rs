use std::{fmt::Debug, fs::File};

use apiclient_rs::ApiResult;
use curseforge_rs::{client::CurseForgeApiClient, v1::{categories::GetCategoriesParams, games::GetGamesParams, mods::{GetFeaturedModsParams, GetModsParams, GetSearchModsParams}}};
use serde_json::to_writer;
use tokio::runtime::Runtime;
use tracing::Level;
use tracing_subscriber;

async fn run() -> ApiResult<()> {

    let client = CurseForgeApiClient::new("$2a$10$ki0i8Ev9gHBFPvbrTTiA7eNIrqWPFf4xUceGRB1KDob3FgSE3jI/q".to_string());

    // let params: GetCategoriesParams = GetCategoriesParams {
    //     game_id: 432,
    //     class_id: None,
    //     classes_only: None,
    // };

    // let params: GetSearchModsParams = GetSearchModsParams {
    //     game_id: 432,
    //     class_id: None,
    //     category_id: None,
    //     category_ids: None,
    //     game_version: None,
    //     game_versions: None,
    //     search_filter: None,
    //     sort_field: None,
    //     mod_loader_type: None,
    //     mod_loader_types: None,
    //     game_version_type_id: None,
    //     author_id: None,
    //     primary_author_id: None,
    //     slug: None,
    //     index: None,
    //     page_size: None,    
    // };

    // let params: GetModsParams = GetModsParams {
    //     mod_ids: vec![624165],
    //     filter_pc_only: Some(false),
    // };

    let params: GetFeaturedModsParams = GetFeaturedModsParams {
        game_id: 432,
        excluded_mod_ids: vec![],
        game_version_type_id: None,
    };
    
    //let response = client.v1.get_mod(624165).await?;
    
    let response = client.v1.get_mod_descriptions(624165,None).await?;
    
    // Example using API key authentication
    // let api_key_auth = Arc::new(ApiKeyAuth::new("$2a$10$ki0i8Ev9gHBFPvbrTTiA7eNIrqWPFf4xUceGRB1KDob3FgSE3jI/q".to_string()));
    // let client_with_api_key = ApiClient::new("https://api.curseforge.com", Some(api_key_auth));

    // // Example using Bearer token authentication
    // let bearer_auth = Arc::new(BearerAuth::new("your_bearer_token".to_string()));
    // let client_with_bearer_token = ApiClient::new("https://api.example.com", Some(bearer_auth));

    // // Example without any authentication
    // let client_without_auth = ApiClient::new("https://api.example.com", None);

    // // Example API call with the API key client
    // let response: serde_json::Value = client_with_api_key.get("v1/games", None).await?;
    // println!("Response with API key: {:?}", response);

    let file = File::create("out.json").unwrap();

    // Serialize the struct to JSON and write it to the file
    to_writer(file, &response)?;
    // Example API call with the Bearer token client
    // let response: serde_json::Value = client_with_bearer_t       oken.get("another_endpoint", None).await?;
    // println!("Response with Bearer token: {:?}", response);

    // // Example API call with no authentication
    // let response: serde_json::Value = client_without_auth.get("public_endpoint", None).await?;
    // println!("Response without auth: {:?}", response);

    Ok(())
}

fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt().with_max_level(Level::DEBUG).init();
    
    // Create a Tokio runtime and run the async main function
    let rt = Runtime::new().unwrap();
    if let Err(e) = rt.block_on(run()) {
        eprintln!("Application error: {:?}\n", e);
    }
}
