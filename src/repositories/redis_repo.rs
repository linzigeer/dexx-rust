use redis::aio::ConnectionManager;
use redis::{AsyncCommands, Client};
use crate::config::Config;
use crate::utils::AppResult;
use serde::{Serialize, Deserialize};

pub struct RedisRepository {
    connection: ConnectionManager,
}

impl RedisRepository {
    pub async fn new(config: &Config) -> AppResult<Self> {
        let redis_url = config.redis_url();
        
        tracing::info!("Connecting to Redis: {}", 
            redis_url.replace(&config.redis.password, "***"));
        
        let client = Client::open(redis_url)?;
        let connection = ConnectionManager::new(client).await?;
        
        tracing::info!("Redis connection established successfully");
        
        Ok(Self { connection })
    }
    
    pub async fn get<T>(&mut self, key: &str) -> AppResult<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let value: Option<String> = self.connection.get(key).await?;
        match value {
            Some(v) => {
                let result = serde_json::from_str(&v)?;
                Ok(Some(result))
            }
            None => Ok(None),
        }
    }
    
    pub async fn set<T>(&mut self, key: &str, value: &T, ttl_seconds: Option<usize>) -> AppResult<()>
    where
        T: Serialize,
    {
        let serialized = serde_json::to_string(value)?;
        
        match ttl_seconds {
            Some(ttl) => {
                self.connection.set_ex(key, serialized, ttl).await?;
            }
            None => {
                self.connection.set(key, serialized).await?;
            }
        }
        
        Ok(())
    }
    
    pub async fn delete(&mut self, key: &str) -> AppResult<()> {
        self.connection.del(key).await?;
        Ok(())
    }
    
    pub async fn exists(&mut self, key: &str) -> AppResult<bool> {
        let result: bool = self.connection.exists(key).await?;
        Ok(result)
    }
    
    pub async fn increment(&mut self, key: &str) -> AppResult<i64> {
        let result: i64 = self.connection.incr(key, 1).await?;
        Ok(result)
    }
    
    pub async fn set_string(&mut self, key: &str, value: &str, ttl_seconds: Option<usize>) -> AppResult<()> {
        match ttl_seconds {
            Some(ttl) => {
                self.connection.set_ex(key, value, ttl).await?;
            }
            None => {
                self.connection.set(key, value).await?;
            }
        }
        Ok(())
    }
    
    pub async fn get_string(&mut self, key: &str) -> AppResult<Option<String>> {
        let result: Option<String> = self.connection.get(key).await?;
        Ok(result)
    }
    
    pub async fn health_check(&mut self) -> AppResult<()> {
        let _: String = self.connection.ping().await?;
        Ok(())
    }
    
    // 缓存相关的便捷方法
    pub async fn cache_token_info<T>(&mut self, mint: &str, data: &T, ttl_seconds: usize) -> AppResult<()>
    where
        T: Serialize,
    {
        let key = format!("sol:tokeninfo:{}", mint);
        self.set(&key, data, Some(ttl_seconds)).await
    }
    
    pub async fn get_cached_token_info<T>(&mut self, mint: &str) -> AppResult<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let key = format!("sol:tokeninfo:{}", mint);
        self.get(&key).await
    }
    
    pub async fn cache_user_session(&mut self, user_id: u32, token: &str, ttl_seconds: usize) -> AppResult<()> {
        let key = format!("user:session:{}", user_id);
        self.set_string(&key, token, Some(ttl_seconds)).await
    }
    
    pub async fn get_user_session(&mut self, user_id: u32) -> AppResult<Option<String>> {
        let key = format!("user:session:{}", user_id);
        self.get_string(&key).await
    }
    
    pub async fn invalidate_user_session(&mut self, user_id: u32) -> AppResult<()> {
        let key = format!("user:session:{}", user_id);
        self.delete(&key).await
    }
}

// Redis健康检查
pub async fn health_check(config: &Config) -> AppResult<()> {
    let mut redis = RedisRepository::new(config).await?;
    redis.health_check().await
}
