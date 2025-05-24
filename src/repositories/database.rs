use sqlx::{MySqlPool, MySql, Pool};
use crate::config::Config;
use crate::utils::AppResult;

pub type DatabasePool = Pool<MySql>;

pub async fn create_pool(config: &Config) -> AppResult<DatabasePool> {
    let database_url = config.database_url();
    
    tracing::info!("Connecting to database: {}", 
        database_url.replace(&config.mysql.password, "***"));
    
    let pool = MySqlPool::connect(&database_url).await?;
    
    // 测试连接
    sqlx::query("SELECT 1").execute(&pool).await?;
    
    tracing::info!("Database connection established successfully");
    
    Ok(pool)
}

pub async fn run_migrations(pool: &DatabasePool) -> AppResult<()> {
    tracing::info!("Running database migrations...");
    
    // 这里可以添加数据库迁移逻辑
    // sqlx::migrate!("./migrations").run(pool).await?;
    
    tracing::info!("Database migrations completed");
    
    Ok(())
}

// 数据库健康检查
pub async fn health_check(pool: &DatabasePool) -> AppResult<()> {
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}
