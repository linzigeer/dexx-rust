use axum::response::IntoResponse;
use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use crate::handlers::response::{error, error_codes};
use crate::utils::{AppResult, CryptoUtils};
use crate::models::user::User;

/// JWT认证中间件
pub async fn jwt_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // 从header中获取token
    let token = headers
        .get("token")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");

    if token.is_empty() {
        return Ok(Json(error(error_codes::TOKEN_ERROR, "Token is required")).into_response());
    }

    // 这里需要从配置中获取JWT密钥，暂时硬编码
    // TODO: 从AppState中获取配置
    let jwt_secret = "your-jwt-secret"; // 临时占位符

    // 验证JWT token
    match CryptoUtils::verify_jwt(token, jwt_secret) {
        Ok(claims) => {
            // 将用户信息添加到请求扩展中
            // 这里需要根据claims创建User对象
            // TODO: 实现从claims到User的转换
            request.extensions_mut().insert(claims);
            Ok(next.run(request).await)
        }
        Err(_) => {
            Ok(Json(error(error_codes::TOKEN_EXPIRED, "Token is invalid or expired")).into_response())
        }
    }
}

/// CORS中间件
pub async fn cors_middleware(
    request: Request,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;
    
    let headers = response.headers_mut();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers.insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS".parse().unwrap());
    headers.insert("Access-Control-Allow-Headers", "Authorization, Content-Type, Token, faucet-token".parse().unwrap());
    headers.insert("Access-Control-Expose-Headers", "Content-Length, Access-Control-Allow-Origin, Access-Control-Allow-Headers, Content-Type".parse().unwrap());
    
    response
}

/// 请求日志中间件
pub async fn logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = std::time::Instant::now();
    
    let response = next.run(request).await;
    
    let duration = start.elapsed();
    let status = response.status();
    
    tracing::info!(
        method = %method,
        uri = %uri,
        status = %status,
        duration_ms = duration.as_millis(),
        "HTTP request processed"
    );
    
    response
}

/// 错误处理中间件
pub async fn error_handling_middleware(
    request: Request,
    next: Next,
) -> Response {
    let uri = request.uri().clone();
    let response = next.run(request).await;
    
    // 如果响应状态码表示错误，记录日志
    if response.status().is_client_error() || response.status().is_server_error() {
        tracing::warn!(
            status = %response.status(),
            uri = %uri,
            "HTTP request failed"
        );
    }
    
    response
}

/// 限流中间件（简单实现）
pub async fn rate_limit_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // TODO: 实现基于Redis的限流逻辑
    // 这里暂时直接通过
    Ok(next.run(request).await)
}

/// 从请求扩展中获取用户信息的辅助函数
pub fn get_user_from_request(request: &Request) -> Option<&User> {
    request.extensions().get::<User>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Method, Request},
    };

    #[tokio::test]
    async fn test_cors_middleware() {
        // TODO: 实现CORS中间件测试
    }

    #[tokio::test]
    async fn test_jwt_middleware() {
        // TODO: 实现JWT中间件测试
    }
}
