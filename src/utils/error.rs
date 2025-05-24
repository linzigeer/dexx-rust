use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("HTTP client error: {0}")]
    HttpClient(#[from] reqwest::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Solana client error: {0}")]
    SolanaClient(String),
    
    #[error("Ethereum client error: {0}")]
    EthereumClient(String),
    
    #[error("Authentication error: {message}")]
    Authentication { message: String },
    
    #[error("Authorization error: {message}")]
    Authorization { message: String },
    
    #[error("Validation error: {message}")]
    Validation { message: String },
    
    #[error("Business logic error: {message}")]
    Business { message: String },
    
    #[error("External service error: {service}: {message}")]
    ExternalService { service: String, message: String },
    
    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl AppError {
    pub fn authentication(message: impl Into<String>) -> Self {
        Self::Authentication {
            message: message.into(),
        }
    }
    
    pub fn authorization(message: impl Into<String>) -> Self {
        Self::Authorization {
            message: message.into(),
        }
    }
    
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }
    
    pub fn business(message: impl Into<String>) -> Self {
        Self::Business {
            message: message.into(),
        }
    }
    
    pub fn external_service(service: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ExternalService {
            service: service.into(),
            message: message.into(),
        }
    }
    
    pub fn solana_client(message: impl Into<String>) -> Self {
        Self::SolanaClient(message.into())
    }
    
    pub fn ethereum_client(message: impl Into<String>) -> Self {
        Self::EthereumClient(message.into())
    }
}

pub type AppResult<T> = Result<T, AppError>;

// HTTP响应错误转换
impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        use axum::http::StatusCode;
        use axum::Json;
        use serde_json::json;
        
        let (status, error_message) = match &self {
            AppError::Authentication { .. } => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Authorization { .. } => (StatusCode::FORBIDDEN, self.to_string()),
            AppError::Validation { .. } => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::Business { .. } => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
            AppError::Redis(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Cache error".to_string()),
            AppError::ExternalService { .. } => (StatusCode::BAD_GATEWAY, self.to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        };
        
        let body = Json(json!({
            "code": status.as_u16(),
            "message": error_message,
            "data": serde_json::Value::Null
        }));
        
        (status, body).into_response()
    }
}

// JWT错误处理
impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError::authentication(format!("JWT error: {}", err))
    }
}
