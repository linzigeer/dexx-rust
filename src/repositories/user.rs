use sqlx::MySqlPool;
use crate::models::user::{User, CreateUserRequest};
use crate::utils::{AppResult, AppError};
use crate::utils::CryptoUtils;

pub struct UserRepository {
    pool: MySqlPool,
}

impl UserRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
    
    pub async fn create_user(&self, request: CreateUserRequest) -> AppResult<User> {
        // 检查邮箱是否已存在
        if self.find_by_email(&request.email).await?.is_some() {
            return Err(AppError::validation("Email already exists"));
        }
        
        // 加密密码
        let hashed_password = CryptoUtils::hash_password(&request.password);
        
        // 创建用户
        let mut user = User::new(request.username, request.email, hashed_password);
        if let Some(parent) = request.parent {
            user.parent = parent;
        }
        
        let query = r#"
            INSERT INTO cook_jcc_user (
                username, email, password, parent, sol_commission, base_commission, 
                eth_commission, sol_commission_total, base_commission_total, eth_commission_total,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NOW(), NOW())
        "#;
        
        let result = sqlx::query(query)
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.password)
            .bind(&user.parent)
            .bind(user.sol_commission)
            .bind(user.base_commission)
            .bind(user.eth_commission)
            .bind(user.sol_commission_total)
            .bind(user.base_commission_total)
            .bind(user.eth_commission_total)
            .execute(&self.pool)
            .await?;
        
        user.id = result.last_insert_id() as u32;
        
        Ok(user)
    }
    
    pub async fn find_by_id(&self, id: u32) -> AppResult<Option<User>> {
        let query = r#"
            SELECT id, created_at, updated_at, deleted_at, username, email, password, parent,
                   sol_commission, base_commission, eth_commission, sol_commission_total,
                   base_commission_total, eth_commission_total
            FROM cook_jcc_user 
            WHERE id = ? AND deleted_at IS NULL
        "#;
        
        let user = sqlx::query_as::<_, User>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        
        Ok(user)
    }
    
    pub async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let query = r#"
            SELECT id, created_at, updated_at, deleted_at, username, email, password, parent,
                   sol_commission, base_commission, eth_commission, sol_commission_total,
                   base_commission_total, eth_commission_total
            FROM cook_jcc_user 
            WHERE email = ? AND deleted_at IS NULL
        "#;
        
        let user = sqlx::query_as::<_, User>(query)
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;
        
        Ok(user)
    }
    
    pub async fn update_user(&self, user: &User) -> AppResult<()> {
        let query = r#"
            UPDATE cook_jcc_user 
            SET username = ?, sol_commission = ?, base_commission = ?, eth_commission = ?,
                sol_commission_total = ?, base_commission_total = ?, eth_commission_total = ?,
                updated_at = NOW()
            WHERE id = ?
        "#;
        
        sqlx::query(query)
            .bind(&user.username)
            .bind(user.sol_commission)
            .bind(user.base_commission)
            .bind(user.eth_commission)
            .bind(user.sol_commission_total)
            .bind(user.base_commission_total)
            .bind(user.eth_commission_total)
            .bind(user.id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    pub async fn update_commission(&self, user_id: u32, sol_commission: f64, base_commission: f64, eth_commission: f64) -> AppResult<()> {
        let query = r#"
            UPDATE cook_jcc_user 
            SET sol_commission = sol_commission + ?, 
                base_commission = base_commission + ?, 
                eth_commission = eth_commission + ?,
                sol_commission_total = sol_commission_total + ?,
                base_commission_total = base_commission_total + ?,
                eth_commission_total = eth_commission_total + ?,
                updated_at = NOW()
            WHERE id = ?
        "#;
        
        sqlx::query(query)
            .bind(sol_commission)
            .bind(base_commission)
            .bind(eth_commission)
            .bind(sol_commission)
            .bind(base_commission)
            .bind(eth_commission)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    pub async fn verify_password(&self, email: &str, password: &str) -> AppResult<Option<User>> {
        if let Some(user) = self.find_by_email(email).await? {
            if CryptoUtils::verify_password(password, &user.password) {
                return Ok(Some(user));
            }
        }
        Ok(None)
    }
    
    pub async fn delete_user(&self, id: u32) -> AppResult<()> {
        let query = "UPDATE cook_jcc_user SET deleted_at = NOW() WHERE id = ?";
        
        sqlx::query(query)
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    pub async fn list_users(&self, limit: u32, offset: u32) -> AppResult<Vec<User>> {
        let query = r#"
            SELECT id, created_at, updated_at, deleted_at, username, email, password, parent,
                   sol_commission, base_commission, eth_commission, sol_commission_total,
                   base_commission_total, eth_commission_total
            FROM cook_jcc_user 
            WHERE deleted_at IS NULL
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
        "#;
        
        let users = sqlx::query_as::<_, User>(query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;
        
        Ok(users)
    }
    
    pub async fn count_users(&self) -> AppResult<i64> {
        let query = "SELECT COUNT(*) as count FROM cook_jcc_user WHERE deleted_at IS NULL";
        
        let result: (i64,) = sqlx::query_as(query)
            .fetch_one(&self.pool)
            .await?;
        
        Ok(result.0)
    }
}
