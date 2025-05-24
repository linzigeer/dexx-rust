pub mod database;
pub mod redis_repo;
pub mod user;
pub mod solana;
pub mod trade;
pub mod commission;
pub mod listen;

use sqlx::MySqlPool;
use redis::aio::ConnectionManager;
use crate::config::Config;
use crate::utils::AppResult;

pub use database::*;
pub use redis_repo::*;
pub use user::*;
pub use solana::*;
pub use trade::*;
pub use commission::*;
pub use listen::*;

pub struct Repositories {
    pub user: UserRepository,
    pub solana: SolanaRepository,
    pub trade: TradeRepository,
    pub commission: CommissionRepository,
    pub listen: ListenRepository,
}

impl Repositories {
    pub fn new(database: MySqlPool, redis: ConnectionManager) -> Self {
        Self {
            user: UserRepository::new(database.clone()),
            solana: SolanaRepository::new(database.clone()),
            trade: TradeRepository::new(database.clone()),
            commission: CommissionRepository::new(database.clone()),
            listen: ListenRepository::new(database.clone()),
        }
    }
}

pub async fn init_database(config: &Config) -> AppResult<MySqlPool> {
    let database_url = config.database_url();
    let pool = MySqlPool::connect(&database_url).await?;
    
    // 运行数据库迁移
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    Ok(pool)
}

pub async fn init_redis(config: &Config) -> AppResult<ConnectionManager> {
    let redis_url = config.redis_url();
    let client = redis::Client::open(redis_url)?;
    let manager = ConnectionManager::new(client).await?;
    Ok(manager)
}
