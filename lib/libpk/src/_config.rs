use config::Config;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::sync::Arc;

use twilight_model::id::{marker::UserMarker, Id};

#[derive(Clone, Deserialize, Debug)]
pub struct ClusterSettings {
    pub node_id: u32,
    pub total_shards: u32,
    pub total_nodes: u32,
}

#[derive(Deserialize, Debug)]
pub struct DiscordConfig {
    pub client_id: Id<UserMarker>,
    pub bot_token: String,
    pub client_secret: String,
    pub max_concurrency: u32,
    pub cluster: Option<ClusterSettings>,
    pub api_base_url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub(crate) data_db_uri: String,
    pub(crate) data_db_max_connections: Option<u32>,
    pub(crate) data_db_min_connections: Option<u32>,
    //    pub(crate) _messages_db_uri: String,
    pub(crate) db_password: Option<String>,
    pub data_redis_addr: String,
}

fn _default_api_addr() -> String {
    "0.0.0.0:5000".to_string()
}

#[derive(Deserialize, Debug)]
pub struct ApiConfig {
    #[serde(default = "_default_api_addr")]
    pub addr: String,

    #[serde(default)]
    pub ratelimit_redis_addr: Option<String>,

    pub remote_url: String,

    #[serde(default)]
    pub temp_token2: Option<String>,
}

fn _metrics_default() -> bool {
    false
}
fn _json_log_default() -> bool {
    false
}

#[derive(Deserialize, Debug)]
pub struct PKConfig {
    pub db: DatabaseConfig,

    pub discord: DiscordConfig,
    pub api: ApiConfig,

    #[serde(default = "_metrics_default")]
    pub run_metrics_server: bool,

    #[serde(default = "_json_log_default")]
    pub(crate) json_log: bool,
}

lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: Arc<PKConfig> = {
        if let Ok(var) = std::env::var("NOMAD_ALLOC_INDEX") {
            std::env::set_var("pluralkit__discord__cluster__node_id", var);
        }

        Arc::new(Config::builder()
        .add_source(config::Environment::with_prefix("pluralkit").separator("__"))
        .build().unwrap()
        .try_deserialize::<PKConfig>().unwrap())
    };
}
