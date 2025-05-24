use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(skip_serializing)]
    pub parent: String,
    pub sol_commission: f64,
    pub base_commission: f64,
    pub eth_commission: f64,
    pub sol_commission_total: f64,
    pub base_commission_total: f64,
    pub eth_commission_total: f64,
}

impl User {
    pub fn table_name() -> &'static str {
        "cook_jcc_user"
    }
    
    pub fn new(username: String, email: String, password: String) -> Self {
        let now = Utc::now();
        Self {
            id: 0, // 将由数据库自动生成
            created_at: now,
            updated_at: now,
            deleted_at: None,
            username,
            email,
            password,
            parent: String::new(),
            sol_commission: 0.0,
            base_commission: 0.0,
            eth_commission: 0.0,
            sol_commission_total: 0.0,
            base_commission_total: 0.0,
            eth_commission_total: 0.0,
        }
    }
    
    pub fn with_parent(username: String, email: String, password: String, parent: String) -> Self {
        let mut user = Self::new(username, email, password);
        user.parent = parent;
        user
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub parent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub sol_commission: f64,
    pub base_commission: f64,
    pub eth_commission: f64,
    pub sol_commission_total: f64,
    pub base_commission_total: f64,
    pub eth_commission_total: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            sol_commission: user.sol_commission,
            base_commission: user.base_commission,
            eth_commission: user.eth_commission,
            sol_commission_total: user.sol_commission_total,
            base_commission_total: user.base_commission_total,
            eth_commission_total: user.eth_commission_total,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}
