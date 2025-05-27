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
    pub solana: SolanaConfig, // 新增的Solana配置
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

/// 新的Solana配置结构
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SolanaConfig {
    pub rpc_url: String,
    pub ws_url: Option<String>,
    pub timeout_seconds: Option<u64>,
    pub max_retries: Option<u32>,
    pub price_sources: PriceSourcesConfig,
    pub monitoring: MonitoringConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriceSourcesConfig {
    pub jupiter_enabled: bool,
    pub coingecko_enabled: bool,
    pub coingecko_api_key: Option<String>,
    pub update_interval_seconds: u64,
    pub cache_ttl_seconds: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MonitoringConfig {
    pub enabled: bool,
    pub watch_addresses: Vec<String>,
    pub watch_tokens: Vec<String>,
    pub block_check_interval_seconds: u64,
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
        
        let mut config: Config = settings.try_deserialize().unwrap_or_else(|_| {
            // 如果配置文件不存在或解析失败，使用默认配置
            Self::default()
        });
        
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

impl Default for Config {
    fn default() -> Self {
        Self {
            debug: true,
            http_listen: "0.0.0.0:8080".to_string(),
            mysql: DatabaseConfig {
                host: "localhost:3306".to_string(),
                user: "root".to_string(),
                password: "password".to_string(),
                database: "dexx".to_string(),
                charset: "utf8mb4".to_string(),
            },
            redis: RedisConfig {
                host: "localhost:6379".to_string(),
                user: "".to_string(),
                password: "".to_string(),
                default_db: 0,
            },
            endpoint: EndpointConfig {
                http: HashMap::new(),
                wss: HashMap::new(),
            },
            sol_endpoint: SolEndpointConfig {
                http: vec!["https://api.mainnet-beta.solana.com".to_string()],
                wss: vec!["wss://api.mainnet-beta.solana.com".to_string()],
                rpc_limit: 100,
            },
            solana: SolanaConfig {
                rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
                ws_url: Some("wss://api.mainnet-beta.solana.com".to_string()),
                timeout_seconds: Some(30),
                max_retries: Some(3),
                price_sources: PriceSourcesConfig {
                    jupiter_enabled: true,
                    coingecko_enabled: true,
                    coingecko_api_key: None,
                    update_interval_seconds: 300, // 5分钟
                    cache_ttl_seconds: 300,
                },
                monitoring: MonitoringConfig {
                    enabled: false,
                    watch_addresses: vec![],
                    watch_tokens: vec![],
                    block_check_interval_seconds: 5,
                },
            },
            mailslurp_key: vec![],
            jwt_token: JwtTokenConfig {
                sign: "jincancan".to_string(),
                expire: 86400 * 365,
            },
            oklink: OklinkConfig {
                api_limit: 100,
                api_host: "https://www.oklink.com".to_string(),
                apikey: vec![],
            },
            gecko: GeckoConfig {
                apikey: "".to_string(),
                apihost: "https://api.coingecko.com".to_string(),
            },
            mailgun: MailgunConfig {
                apikey: "".to_string(),
                domain: "".to_string(),
                sender: "".to_string(),
            },
            commission: CommissionConfig {
                dex: 0.0,
                l1: 0.0,
                l2: 0.0,
            },
            commission_wallet: HashMap::new(),
            static_space: Some("./space".to_string()),
            sign_message: Some("watermelo.io".to_string()),
            draw_wallet: HashMap::new(),
            draw_min: HashMap::new(),
            smartdaili: SmartdailiConfig {
                token: "".to_string(),
                host: "".to_string(),
            },
            clickhouse: ClickhouseConfig {
                host: "".to_string(),
                user: "".to_string(),
                password: "".to_string(),
            },
            s3: S3Config {
                username: "".to_string(),
                access_key_id: "".to_string(),
                secret_access_key: "".to_string(),
                region: "us-east-1".to_string(),
            },
            faucet_owner: "".to_string(),
            cook_contract: "".to_string(),
            raydium_launchpad_program: "".to_string(),
            raydium_platform_id: "".to_string(),
            cpmm_program: "".to_string(),
            cook_complate: 0.0,
            cook_api: CookApiConfig {
                base_url: "".to_string(),
                api_key: "".to_string(),
            },
        }
    }
}
