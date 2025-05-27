use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use crate::utils::AppError;

/// 标准API响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    /// 创建成功响应
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    /// 创建成功响应（无数据）
    pub fn success_empty() -> ApiResponse<()> {
        ApiResponse {
            code: 0,
            message: "success".to_string(),
            data: None,
        }
    }

    /// 创建错误响应
    pub fn error(code: i32, message: impl Into<String>) -> ApiResponse<()> {
        ApiResponse {
            code,
            message: message.into(),
            data: None,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let json = Json(self);
        json.into_response()
    }
}

/// 从AppError转换为API响应
impl From<AppError> for ApiResponse<()> {
    fn from(error: AppError) -> Self {
        let (code, message) = match error {
            AppError::Database(msg) => (1001, format!("Database error: {}", msg)),
            AppError::Redis(msg) => (1002, format!("Redis error: {}", msg)),
            AppError::Config(msg) => (1003, format!("Config error: {}", msg)),
            AppError::Jwt(msg) => (1004, format!("JWT error: {}", msg)),
            AppError::Crypto(msg) => (1005, format!("Crypto error: {}", msg)),
            AppError::Http(msg) => (1006, format!("HTTP error: {}", msg)),
            AppError::Io(msg) => (1007, format!("IO error: {}", msg)),
            AppError::Serde(msg) => (1008, format!("Serialization error: {}", msg)),
            AppError::NotFound(msg) => (404, msg),
            AppError::Unauthorized(msg) => (401, msg),
            AppError::Forbidden(msg) => (403, msg),
            AppError::BadRequest(msg) => (400, msg),
            AppError::Internal(msg) => (500, msg),
            AppError::SolanaClient(msg) => (1009, format!("Solana client error: {}", msg)),
            AppError::EthereumClient(msg) => (1010, format!("Ethereum client error: {}", msg)),
            AppError::Authentication { message } => (401, message),
            AppError::Authorization { message } => (403, message),
            AppError::Validation { message } => (400, message),
        AppError::Business { message } => (400, message),
        AppError::ExternalService { service, message } => (502, format!("{}: {}", service, message)),
        AppError::BlockchainError(msg) => (502, format!("Blockchain error: {}", msg)),
        AppError::PriceServiceError(msg) => (502, format!("Price service error: {}", msg)),
        AppError::WalletVerificationError(msg) => (400, format!("Wallet verification error: {}", msg)),
        AppError::TransactionParsingError(msg) => (400, format!("Transaction parsing error: {}", msg)),
            AppError::Business { message } => (400, message),
            AppError::ExternalService { service, message } => (502, format!("{}: {}", service, message)),
        };
        
        ApiResponse::<()>::error(code, message)
    }
}

/// 分页响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, total: i64, page: u32, limit: u32) -> Self {
        let total_pages = if limit > 0 {
            ((total as f64) / (limit as f64)).ceil() as u32
        } else {
            0
        };

        Self {
            items,
            total,
            page,
            limit,
            total_pages,
        }
    }
}

/// 创建成功响应的便捷函数
pub fn success<T: Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse::success(data)
}

/// 创建成功响应（无数据）的便捷函数
pub fn success_empty() -> ApiResponse<()> {
    ApiResponse::<()>::success_empty()
}

/// 创建错误响应的便捷函数
pub fn error(code: i32, message: impl Into<String>) -> ApiResponse<()> {
    ApiResponse::<()>::error(code, message)
}

/// 创建分页响应的便捷函数
pub fn paginated<T: Serialize>(
    items: Vec<T>,
    total: i64,
    page: u32,
    limit: u32,
) -> ApiResponse<PaginatedResponse<T>> {
    ApiResponse::success(PaginatedResponse::new(items, total, page, limit))
}

/// 常用错误代码常量
pub mod error_codes {
    pub const SUCCESS: i32 = 0;
    pub const PARAM_ERROR: i32 = 400;
    pub const UNAUTHORIZED: i32 = 401;
    pub const FORBIDDEN: i32 = 403;
    pub const NOT_FOUND: i32 = 404;
    pub const INTERNAL_ERROR: i32 = 500;
    pub const TOKEN_ERROR: i32 = 1001;
    pub const TOKEN_EXPIRED: i32 = 1002;
    pub const DATABASE_ERROR: i32 = 1003;
    pub const REDIS_ERROR: i32 = 1004;
    pub const VALIDATION_ERROR: i32 = 1005;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_response() {
        let response = ApiResponse::success("test data");
        assert_eq!(response.code, 0);
        assert_eq!(response.message, "success");
        assert_eq!(response.data, Some("test data"));
    }

    #[test]
    fn test_error_response() {
        let response = ApiResponse::<()>::error(400, "Bad request");
        assert_eq!(response.code, 400);
        assert_eq!(response.message, "Bad request");
        assert_eq!(response.data, None);
    }

    #[test]
    fn test_paginated_response() {
        let items = vec![1, 2, 3];
        let response = PaginatedResponse::new(items, 10, 1, 3);
        assert_eq!(response.total, 10);
        assert_eq!(response.page, 1);
        assert_eq!(response.limit, 3);
        assert_eq!(response.total_pages, 4);
    }
}
