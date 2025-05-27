use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub debug: bool,
    pub http_listen: String,
    pub mysql: DatabaseConfig,
    pub redis: RedisConfig,
    pub endpoint: EndpointConfig,
    #[serde(rename = "solEndPoint")]
    pub sol_endpoint: SolEndpointConfig,
    pub mailslurp_key: Vec<String>,
    pub jwt_token: JwtTokenConfig,
    pub oklink: OklinkConfig,
    pub gecko: GeckoConfig,
    pub mailgun: MailgunConfig,
    pub commission: CommissionConfig,
    pub commission_wallet: HashMap<String, String>,
    pub static_space: Option<String>,
    pub sign_message: Option<String>,
    pub draw_wallet: HashMap<String, String>,
    pub draw_min: HashMap<String, f64>,
    pub smartdaili: SmartdailiConfig,
    pub clickhouse: ClickhouseConfig,
    pub s3: S3Config,
    pub faucet_owner: String,
    pub cook_contract: String,
    pub raydium_launchpad_program: String,
    pub raydium_platform_id: String,
    pub cpmm_program: String,
    pub cook_complate: f64,
    pub cook_api: CookApiConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub user: String,
    pub password: String,
    pub database: String,
    pub charset: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedisConfig {
    pub host: String,
    pub user: String,
    pub password: String,
    pub default_db: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EndpointConfig {
    pub http: HashMap<String, String>,
    pub wss: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SolEndpointConfig {
    pub http: Vec<String>,
    pub wss: Vec<String>,
    pub rpc_limit: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JwtTokenConfig {
    pub sign: String,
    pub expire: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OklinkConfig {
    #[serde(rename = "apiLimit")]
    pub api_limit: u32,
    #[serde(rename = "apiHost")]
    pub api_host: String,
    pub apikey: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeckoConfig {
    pub apikey: String,
    pub apihost: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MailgunConfig {
    pub apikey: String,
    pub domain: String,
    pub sender: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommissionConfig {
    pub dex: f64,
    pub l1: f64,
    pub l2: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SmartdailiConfig {
    pub token: String,
    pub host: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClickhouseConfig {
    pub host: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct S3Config {
    pub username: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub region: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CookApiConfig {
    pub base_url: String,
    pub api_key: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name("config"))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;
        
        let mut config: Config = settings.try_deserialize()?;
        
        // 设置默认值（类似Go版本的逻辑）
        config.jwt_token.sign = "jincancan".to_string();
        config.jwt_token.expire = 86400 * 365;
        config.static_space = Some("./space".to_string());
        config.sign_message = Some("watermelo.io".to_string());
        
        // 设置环境变量（用于AWS S3）
        std::env::set_var("AWS_ACCESS_KEY_ID", &config.s3.access_key_id);
        std::env::set_var("AWS_SECRET_ACCESS_KEY", &config.s3.secret_access_key);
        std::env::set_var("AWS_DEFAULT_REGION", &config.s3.region);
        
        Ok(config)
    }
    
    pub fn database_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}/{}?charset={}",
            self.mysql.user,
            self.mysql.password,
            self.mysql.host,
            self.mysql.database,
            self.mysql.charset
        )
    }
    
    pub fn redis_url(&self) -> String {
        if self.redis.password.is_empty() {
            format!("redis://{}/{}", self.redis.host, self.redis.default_db)
        } else {
            format!(
                "redis://{}:{}@{}/{}",
                self.redis.user, self.redis.password, self.redis.host, self.redis.default_db
            )
        }
    }
}
