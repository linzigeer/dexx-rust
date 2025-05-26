use std::sync::Arc;
use crate::config::Config;
use crate::repositories::RepositoriesImpl;
use crate::models::user::*;
use crate::utils::{AppResult, AppError, CryptoUtils};
use serde::{Deserialize, Serialize};

/// 钱包登录请求
#[derive(Debug, Deserialize)]
pub struct WalletLoginRequest {
    pub wallet_address: String,
    pub network: String,
    pub signature: String,
    pub invite_code: Option<String>,
}

/// 钱包登录响应
#[derive(Debug, Serialize)]
pub struct WalletLoginResponse {
    pub username: String,
    pub wallet: String,
    pub token: String,
    pub invite_code: String,
}

/// 邮箱登录请求
#[derive(Debug, Deserialize)]
pub struct EmailLoginRequest {
    pub email: String,
    pub password: String,
}

/// 用户Token响应
#[derive(Debug, Serialize)]
pub struct UserTokenResponse {
    pub username: String,
    pub email: String,
    pub token: String,
    pub invite_code: Option<String>,
}

/// 用户服务实现
pub struct UserServiceImpl {
    config: Arc<Config>,
    repositories: Arc<RepositoriesImpl>,
}

impl UserServiceImpl {
    /// 创建新的用户服务实例
    pub async fn new(
        config: Arc<Config>,
        repositories: Arc<RepositoriesImpl>,
    ) -> AppResult<Self> {
        Ok(Self {
            config,
            repositories,
        })
    }

    /// 加密密码
    fn encrypt_password(&self, password: &str) -> String {
        // 使用SHA256双重加密，与Go版本保持一致
        let sha256_hash = CryptoUtils::sha256(password);
        CryptoUtils::sha256(&sha256_hash)
    }

    /// 生成用户Token
    async fn create_user_token(&self, user: &User) -> AppResult<String> {
        CryptoUtils::generate_jwt(
            user.id,
            Some(user.email.clone()),
            None, // wallet
            &self.config.jwt_token.sign,
            self.config.jwt_token.expire,
        )
    }

    /// 生成随机用户名
    fn generate_username(&self) -> String {
        CryptoUtils::generate_random_string(8)
    }

    /// 生成邀请码
    pub fn get_invite_code(&self, uid: u32) -> String {
        (uid + 235790).to_string()
    }

    /// 解析邀请码
    pub fn parse_invite_code(&self, invite_code: &str) -> Option<u32> {
        if let Ok(code) = invite_code.parse::<u32>() {
            if code > 235790 {
                Some(code - 235790)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// 邮箱登录 - 简化版本
    pub async fn email_login(&self, req: EmailLoginRequest) -> AppResult<UserTokenResponse> {
        // 查找用户
        let user = match self.repositories.user_repository().find_by_email(&req.email).await? {
            Some(user) => user,
            None => return Err(AppError::validation("Wrong email or password")),
        };

        // 验证密码
        let encrypted_password = self.encrypt_password(&req.password);
        if user.password != encrypted_password {
            return Err(AppError::validation("Wrong email or password"));
        }

        // 生成token
        let token = self.create_user_token(&user).await?;

        Ok(UserTokenResponse {
            username: user.username,
            email: user.email,
            token,
            invite_code: Some(self.get_invite_code(user.id)),
        })
    }

    /// 根据邮箱查找用户
    pub async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        self.repositories.user_repository().find_by_email(email).await
    }

    /// 获取用户信息
    pub async fn user_info(&self, uid: u32) -> AppResult<User> {
        match self.repositories.user_repository().find_by_id(uid).await? {
            Some(user) => Ok(user),
            None => Err(AppError::validation("User not found")),
        }
    }
}
