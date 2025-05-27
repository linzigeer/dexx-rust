//! 钱包签名验证服务 - HTTP实现
//! 
//! 验证Solana钱包的签名和所有权

use std::sync::Arc;
use ed25519_dalek::{Verifier, VerifyingKey, Signature};
use bs58;
use crate::config::Config;
use crate::utils::{AppResult, AppError};
use tracing::{info, warn, error, debug};

/// 钱包验证器
pub struct WalletVerifier {
    config: Arc<Config>,
}

/// 签名验证请求
#[derive(Debug, Clone)]
pub struct SignatureVerificationRequest {
    pub wallet_address: String,
    pub message: String,
    pub signature: String,
}

/// 签名验证结果
#[derive(Debug, Clone)]
pub struct SignatureVerificationResult {
    pub is_valid: bool,
    pub wallet_address: String,
    pub message: String,
    pub error: Option<String>,
}

impl WalletVerifier {
    /// 创建新的钱包验证器
    pub fn new(config: Arc<Config>) -> AppResult<Self> {
        Ok(Self { config })
    }

    /// 验证钱包签名
    pub fn verify_signature(&self, request: &SignatureVerificationRequest) -> AppResult<SignatureVerificationResult> {
        debug!("验证钱包签名: {}", request.wallet_address);

        // 解析钱包地址（公钥）
        let public_key_bytes = match bs58::decode(&request.wallet_address).into_vec() {
            Ok(bytes) => {
                if bytes.len() != 32 {
                    let error_msg = "钱包地址长度不正确".to_string();
                    warn!("{}", error_msg);
                    return Ok(SignatureVerificationResult {
                        is_valid: false,
                        wallet_address: request.wallet_address.clone(),
                        message: request.message.clone(),
                        error: Some(error_msg),
                    });
                }
                bytes
            }
            Err(e) => {
                let error_msg = format!("无效的钱包地址: {}", e);
                warn!("{}", error_msg);
                return Ok(SignatureVerificationResult {
                    is_valid: false,
                    wallet_address: request.wallet_address.clone(),
                    message: request.message.clone(),
                    error: Some(error_msg),
                });
            }
        };

        // 创建验证密钥
        let verifying_key = match VerifyingKey::from_bytes(&public_key_bytes.try_into().unwrap()) {
            Ok(key) => key,
            Err(e) => {
                let error_msg = format!("创建验证密钥失败: {}", e);
                warn!("{}", error_msg);
                return Ok(SignatureVerificationResult {
                    is_valid: false,
                    wallet_address: request.wallet_address.clone(),
                    message: request.message.clone(),
                    error: Some(error_msg),
                });
            }
        };

        // 解析签名
        let signature_bytes = match bs58::decode(&request.signature).into_vec() {
            Ok(bytes) => {
                if bytes.len() != 64 {
                    let error_msg = "签名长度不正确".to_string();
                    warn!("{}", error_msg);
                    return Ok(SignatureVerificationResult {
                        is_valid: false,
                        wallet_address: request.wallet_address.clone(),
                        message: request.message.clone(),
                        error: Some(error_msg),
                    });
                }
                bytes
            }
            Err(e) => {
                let error_msg = format!("无效的签名格式: {}", e);
                warn!("{}", error_msg);
                return Ok(SignatureVerificationResult {
                    is_valid: false,
                    wallet_address: request.wallet_address.clone(),
                    message: request.message.clone(),
                    error: Some(error_msg),
                });
            }
        };

        // 创建签名对象
        let signature = match Signature::try_from(signature_bytes.as_slice()) {
            Ok(sig) => sig,
            Err(e) => {
                let error_msg = format!("创建签名对象失败: {}", e);
                warn!("{}", error_msg);
                return Ok(SignatureVerificationResult {
                    is_valid: false,
                    wallet_address: request.wallet_address.clone(),
                    message: request.message.clone(),
                    error: Some(error_msg),
                });
            }
        };

        // 验证签名
        let message_bytes = request.message.as_bytes();
        let is_valid = verifying_key.verify(message_bytes, &signature).is_ok();

        if is_valid {
            info!("钱包签名验证成功: {}", request.wallet_address);
        } else {
            warn!("钱包签名验证失败: {}", request.wallet_address);
        }

        Ok(SignatureVerificationResult {
            is_valid,
            wallet_address: request.wallet_address.clone(),
            message: request.message.clone(),
            error: None,
        })
    }

    /// 生成登录挑战消息
    pub fn generate_login_challenge(&self, wallet_address: &str) -> AppResult<String> {
        let timestamp = chrono::Utc::now().timestamp();
        let nonce = uuid::Uuid::new_v4().to_string();
        
        let challenge = format!(
            "请签名此消息以验证您的钱包所有权:\n\n钱包地址: {}\n时间戳: {}\n随机数: {}\n\n此签名仅用于身份验证，不会产生任何费用。",
            wallet_address,
            timestamp,
            nonce
        );

        debug!("生成登录挑战: {}", wallet_address);
        Ok(challenge)
    }

    /// 验证登录挑战
    pub fn verify_login_challenge(
        &self,
        wallet_address: &str,
        challenge: &str,
        signature: &str,
    ) -> AppResult<SignatureVerificationResult> {
        // 检查挑战消息的时效性（5分钟内有效）
        if let Some(timestamp_str) = self.extract_timestamp_from_challenge(challenge) {
            if let Ok(timestamp) = timestamp_str.parse::<i64>() {
                let now = chrono::Utc::now().timestamp();
                if now - timestamp > 300 { // 5分钟
                    return Ok(SignatureVerificationResult {
                        is_valid: false,
                        wallet_address: wallet_address.to_string(),
                        message: challenge.to_string(),
                        error: Some("挑战消息已过期".to_string()),
                    });
                }
            }
        }

        // 验证签名
        let request = SignatureVerificationRequest {
            wallet_address: wallet_address.to_string(),
            message: challenge.to_string(),
            signature: signature.to_string(),
        };

        self.verify_signature(&request)
    }

    /// 从挑战消息中提取时间戳
    fn extract_timestamp_from_challenge(&self, challenge: &str) -> Option<String> {
        for line in challenge.lines() {
            if line.starts_with("时间戳: ") {
                return Some(line.replace("时间戳: ", ""));
            }
        }
        None
    }

    /// 验证钱包是否拥有特定代币
    pub async fn verify_token_ownership(
        &self,
        wallet_address: &str,
        mint_address: &str,
        minimum_amount: u64,
    ) -> AppResult<bool> {
        debug!("验证代币所有权: 钱包={}, 代币={}, 最小数量={}", 
               wallet_address, mint_address, minimum_amount);

        // 这里需要集成SolanaClientService来查询代币余额
        // 暂时返回false，等待集成
        
        Ok(false)
    }

    /// 验证钱包是否为代币的铸造权限
    pub async fn verify_mint_authority(
        &self,
        wallet_address: &str,
        mint_address: &str,
    ) -> AppResult<bool> {
        debug!("验证铸造权限: 钱包={}, 代币={}", wallet_address, mint_address);

        // 这里需要集成SolanaClientService来查询代币信息
        // 暂时返回false，等待集成
        
        Ok(false)
    }

    /// 批量验证多个签名
    pub fn verify_multiple_signatures(
        &self,
        requests: &[SignatureVerificationRequest],
    ) -> AppResult<Vec<SignatureVerificationResult>> {
        let mut results = Vec::new();
        
        for request in requests {
            let result = self.verify_signature(request)?;
            results.push(result);
        }
        
        Ok(results)
    }

    /// 生成用于测试的签名（仅用于开发环境）
    #[cfg(debug_assertions)]
    pub fn generate_test_signature(&self, message: &str) -> AppResult<(String, String)> {
        use ed25519_dalek::{Signer, SigningKey};
        use rand::rngs::OsRng;

        let mut csprng = OsRng;
        let signing_key = SigningKey::from_bytes(&rand::random::<[u8; 32]>());
        let verifying_key = signing_key.verifying_key();
        
        let wallet_address = bs58::encode(verifying_key.as_bytes()).into_string();
        
        let message_bytes = message.as_bytes();
        let signature = signing_key.sign(message_bytes);
        let signature_str = bs58::encode(signature.to_bytes()).into_string();
        
        Ok((wallet_address, signature_str))
    }
}

/// 钱包类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum WalletType {
    Phantom,
    Solflare,
    Backpack,
    Other(String),
}

impl WalletType {
    /// 从用户代理字符串识别钱包类型
    pub fn from_user_agent(user_agent: &str) -> Self {
        if user_agent.contains("Phantom") {
            WalletType::Phantom
        } else if user_agent.contains("Solflare") {
            WalletType::Solflare
        } else if user_agent.contains("Backpack") {
            WalletType::Backpack
        } else {
            WalletType::Other(user_agent.to_string())
        }
    }
}

/// 钱包连接信息
#[derive(Debug, Clone)]
pub struct WalletConnection {
    pub address: String,
    pub wallet_type: WalletType,
    pub connected_at: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
    pub is_verified: bool,
}

impl WalletConnection {
    /// 创建新的钱包连接
    pub fn new(address: String, wallet_type: WalletType) -> Self {
        let now = chrono::Utc::now();
        Self {
            address,
            wallet_type,
            connected_at: now,
            last_activity: now,
            is_verified: false,
        }
    }

    /// 更新最后活动时间
    pub fn update_activity(&mut self) {
        self.last_activity = chrono::Utc::now();
    }

    /// 标记为已验证
    pub fn mark_verified(&mut self) {
        self.is_verified = true;
        self.update_activity();
    }

    /// 检查连接是否过期
    pub fn is_expired(&self, timeout_minutes: i64) -> bool {
        let now = chrono::Utc::now();
        let duration = now - self.last_activity;
        duration.num_minutes() > timeout_minutes
    }
}
