use std::{fs::File, io::Write, sync::Arc};

use apiclient_rs::{ApiClient, ApiClientError, ApiKeyAuth, ApiResult};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use tracing::{debug, error};

use crate::{v1::V1Client, v2::V2Client};

const BASE_URL: &str = "https://api.curseforge.com";
//"$2a$10$ki0i8Ev9gHBFPvbrTTiA7eNIrqWPFf4xUceGRB1KDob3FgSE3jI/q".to_string()

#[derive(Debug, Clone)]
pub struct CurseForgeApiClient {
    client: Arc<ApiClient>,
    pub v1: V1Client,
    pub v2: V2Client,
}

impl CurseForgeApiClient {
    pub fn new(api_key: String) -> Arc<Self> {
        let api_key_auth = Arc::new(ApiKeyAuth::new(api_key));
        let client_with_api_key = Arc::new(ApiClient::new(BASE_URL, Some(api_key_auth)));

        // Initialize CurseForgeApiClient without the v1 and v2 clients
        let api_client = Self {
            client: client_with_api_key,
            v1: V1Client::new_empty(),
            v2: V2Client::new_empty(),
        };

        let arc_api_client = Arc::new(api_client);

        // Now initialize the v1 and v2 clients with the arc_api_client reference
        let v1 = V1Client::new(Arc::clone(&arc_api_client));
        let v2 = V2Client::new(Arc::clone(&arc_api_client));

        // Create a new CurseForgeApiClient with v1 and v2 properly set
        let api_client = Arc::new(Self {
            client: Arc::clone(&arc_api_client.client),
            v1,
            v2,
        });

        api_client
    }

    pub async fn get<T, B>(&self, endpoint: &str, params: Option<&B>) -> ApiResult<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let json_string = serde_json::to_string(&params)?;
        println!("Serialized JSON: {}", json_string);
        
        let pairs: Vec<(String, String)> = convert_json_to_pairs(&json_string)?;
        
        let pair_refs: Vec<(&str, &str)> = pairs.iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();

        let response: Value;
        response = self.client.get(&endpoint, Some(&pair_refs)).await?;
        debug!("Response: {:?}", response);

        let result = serde_path_to_error::deserialize::<_, T>(response);
        match result {
            Ok(parsed) => Ok(parsed),
            Err(err) => {
                let path = err.path().to_string();
                error!("Deserialization error at {}: {}", path, err);
                Err(ApiClientError::DeserializeError(err.to_string())) // Adjust error handling to match your ApiResult type
            }
        }   
    }

    pub async fn get_no_params<T>(&self, endpoint: &str) -> ApiResult<T>
    where
        T: DeserializeOwned,
    {
        let response: Value = self.client.get(&endpoint, None).await?;

        debug!("Response: {:?}", response);

        let result = serde_path_to_error::deserialize::<_, T>(response);
        match result {
            Ok(parsed) => Ok(parsed),
            Err(err) => {
                let path = err.path().to_string();
                error!("Deserialization error at {}: {}", path, err);
                Err(ApiClientError::DeserializeError(err.to_string())) // Adjust error handling to match your ApiResult type
            }
        }   
    }

    pub async fn post<T, B>(&self, endpoint: &str, params: Option<&B>) -> ApiResult<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let response: Value;

        response = self.client.post(&endpoint, Some(&params)).await?;
        
        debug!("Response: {:?}", response);

        let mut file = File::create("output.json").expect("Failed to create file");
        let response_str = serde_json::to_string_pretty(&response).expect("Failed to serialize response");
        file.write_all(response_str.as_bytes()).expect("Failed to write to file");

        let result = serde_path_to_error::deserialize::<_, T>(response);

      

        match result {
            Ok(parsed) => Ok(parsed),
            Err(err) => {
                let path = err.path().to_string();
                error!("Deserialization error at {}: {}", path, err);
                Err(ApiClientError::DeserializeError(err.to_string())) // Adjust error handling to match your ApiResult type
            }
        }   
    }
}


fn convert_json_to_pairs(json_string: &str) -> Result<Vec<(String, String)>, ApiClientError> {
    let value: Value = serde_json::from_str(json_string)?;
    let mut pairs: Vec<(String, String)> = Vec::new();

    if let Value::Object(map) = value {
        for (key, value) in map.iter() {
            let key_str = key.clone(); // Clone the key to convert it to String 
            let value_str = match value {
                Value::String(s) => s.clone(),
                Value::Bool(b) => if *b { "true".to_string() } else { "false".to_string() },
                Value::Number(n) => n.to_string(),
                _ => continue, // Skip other types for simplicity
            };
            pairs.push((key_str, value_str));
        }
    }

    Ok(pairs)
}