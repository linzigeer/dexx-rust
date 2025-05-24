use sha2::{Digest, Sha256};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use crate::utils::AppResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // 用户ID
    pub exp: i64,     // 过期时间
    pub iat: i64,     // 签发时间
    pub email: Option<String>,
    pub wallet: Option<String>,
}

pub struct CryptoUtils;

impl CryptoUtils {
    /// 生成SHA256哈希
    pub fn sha256(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    /// 密码加密（使用SHA256 + 盐）
    pub fn hash_password(password: &str) -> String {
        let salt = "watermelo_salt_2024"; // 在实际项目中应该使用随机盐
        Self::sha256(&format!("{}{}", password, salt))
    }
    
    /// 验证密码
    pub fn verify_password(password: &str, hash: &str) -> bool {
        Self::hash_password(password) == hash
    }
    
    /// 生成JWT Token
    pub fn generate_jwt(
        user_id: u32,
        email: Option<String>,
        wallet: Option<String>,
        secret: &str,
        expire_hours: i64,
    ) -> AppResult<String> {
        let now = Utc::now();
        let exp = now + Duration::hours(expire_hours);
        
        let claims = Claims {
            sub: user_id.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            email,
            wallet,
        };
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )?;
        
        Ok(token)
    }
    
    /// 验证JWT Token
    pub fn verify_jwt(token: &str, secret: &str) -> AppResult<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )?;
        
        Ok(token_data.claims)
    }
    
    /// 生成随机字符串
    pub fn generate_random_string(length: usize) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789";
        let mut rng = rand::thread_rng();
        
        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
    
    /// 生成邀请码
    pub fn generate_invite_code(user_id: u32) -> String {
        // 简单的邀请码生成逻辑，可以根据需要调整
        let base = format!("USER{}", user_id);
        let hash = Self::sha256(&base);
        hash[..8].to_uppercase()
    }
    
    /// 解析邀请码
    pub fn parse_invite_code(_invite_code: &str) -> Option<u32> {
        // 这里需要实现邀请码解析逻辑
        // 暂时返回None，需要根据实际的邀请码生成算法来实现
        None
    }
}
