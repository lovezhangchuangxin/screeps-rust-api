use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use thiserror::Error;

/// Screeps 错误类型
#[derive(Error, Debug)]
pub enum ScreepsError {
    /// http 请求失败
    #[error("HTTP request failed: {0}")]
    Http(#[from] ReqwestError),

    /// json 解析失败，可能是响应数据格式错误，或者是数据类型不匹配
    #[error("JSON parse failed: {0}")]
    Json(#[from] SerdeError),

    /// api 接口报错
    #[error("API error: {0}")]
    Api(String),

    /// auth 认证失败
    #[error("Auth failed")]
    Auth,

    /// 配置错误
    #[error("Invalid config: {0}")]
    Config(String),
}

pub type ScreepsResult<T> = Result<T, ScreepsError>;
