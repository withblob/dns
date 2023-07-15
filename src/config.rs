use std::time::Duration;
use reqwest::{Client};
use serde::{Deserialize, Serialize};

const GLOBAL_CONFIG_URL: &str = "https://api.withblob.com/dns/v1/config.json";
const GLOBAL_CONFIG_FETCH_TIMEOUT_SEC: u64 = 3;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolverConfig {
    pub id: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(rename = "cloudProvider")]
    pub cloud_provider: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub resolvers: Vec<ResolverConfig>,
}

impl GlobalConfig {
    pub async fn fetch() -> Result<GlobalConfig, &'static str> {
        let http_client = Client::builder()
            .connect_timeout(Duration::from_secs(GLOBAL_CONFIG_FETCH_TIMEOUT_SEC))
            .https_only(true)
            .build()
            .expect("Unable to get remote config: failed to build HTTP client");
        let response = http_client.get(GLOBAL_CONFIG_URL).send().await;
        return match response {
            Err(e) => {
                if e.is_connect() {
                    return Err("Unable to get remote config: failed to connect");
                }
                if e.is_timeout() {
                    return Err("Unable to get remote config: timeout exceeded");
                }
                Err("Unable to get remote config")
            },
            Ok(response) => {
                let result = response.json::<GlobalConfig>().await;
                return match result {
                    Err(_) => Err("Unable to get remote config: failed to parse JSON"),
                    Ok(gc) => Ok(gc)
                }
            }
        }
    }
}

