use std::{
    sync::Mutex,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use reqwest::{Client, header::HeaderMap};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::json;

use crate::{
    config::ScreepsConfig,
    error::{ScreepsError, ScreepsResult},
    model::TokenData,
    rate_limit::RateLimits,
};

/// 请求方法
pub enum Method {
    Get,
    Post,
}
pub use Method::*;

pub type AnyPayload = Option<&'static [(&'static str, &'static str)]>;

/// Screeps http 客户端
pub struct ScreepsHttpClient {
    /// 请求客户端
    pub client: Client,
    /// 配置
    pub config: ScreepsConfig,
    /// 限速信息
    pub rate_limits: Mutex<RateLimits>,
    /// 最新的 token
    pub token: Mutex<Option<String>>,
}

impl ScreepsHttpClient {
    pub fn new(config: ScreepsConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout))
            .build()
            .unwrap();

        Self {
            client,
            token: Mutex::new(config.token.clone()),
            config,
            rate_limits: Mutex::new(RateLimits::default()),
        }
    }

    /// 封装 get 请求和 post 请求
    pub async fn request<T: Serialize, U: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<T>,
    ) -> ScreepsResult<U> {
        let url = self.build_url(path);
        let request_builder = match method {
            Method::Get => self.client.get(url).query(&body),
            Method::Post => self.client.post(url).json(&body),
        }
        .headers(self.build_headers());

        // 先检查速率限制
        let rate_limit = self.rate_limits.lock().unwrap().get_limit(&method, path);
        if rate_limit.remaining <= 0 {
            let wait_time = rate_limit.reset * 1000
                - SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u128;
            if wait_time > 0 {
                thread::sleep(Duration::from_millis(wait_time as u64));
            }
        }
        let response = request_builder.send().await?;
        if let Some(token) = response.headers().get("x-token") {
            *self.token.lock().unwrap() = Some(token.to_str().unwrap().to_string());
        }
        self.rate_limits
            .lock()
            .unwrap()
            .update_from_headers(&method, path, response.headers());
        let result = response.json::<U>().await?;
        Ok(result)
    }

    /// 构造请求头，添加 token
    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let token = self.token.lock().unwrap().as_ref().cloned();
        if let Some(token) = token {
            headers.insert("X-Token", token.parse().unwrap());
            headers.insert("X-Username", token.parse().unwrap());
        }
        headers
    }

    /// 根据路径构造完整的 api
    pub fn build_url(&self, path: &str) -> String {
        format!("{}{}", self.config.build_base_url(), path)
    }
}

impl ScreepsHttpClient {
    /// 登录以获取 token
    pub async fn auth(&self) -> ScreepsResult<TokenData> {
        if self.config.email.is_none() || self.config.password.is_none() {
            return Err(ScreepsError::Config(
                "email or password is none".to_string(),
            ));
        }

        let result: Result<TokenData, ScreepsError> = self
            .request(
                Method::Post,
                "/auth/signin",
                Some(json!({
                    "email": self.config.email.clone().unwrap(),
                    "password": self.config.password.clone().unwrap(),
                })),
            )
            .await;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_build_url() {
        let config = ScreepsConfig::new(
            Some("token".to_string()),
            Some("email".to_string()),
            Some("password".to_string()),
            "screeps.com".to_string(),
            true,
            10000,
        );
        let client = ScreepsHttpClient::new(config);
        assert_eq!(
            client.build_url("/auth/signin"),
            "https://screeps.com/api/auth/signin"
        )
    }

    #[test]
    fn test_build_headers() {
        let config = ScreepsConfig::new(
            Some("token".to_string()),
            Some("email".to_string()),
            Some("password".to_string()),
            "screeps.com".to_string(),
            true,
            10000,
        );
        let client = ScreepsHttpClient::new(config);
        assert_eq!(
            client
                .build_headers()
                .get("X-Token")
                .unwrap()
                .to_str()
                .unwrap(),
            "token"
        );
    }

    #[tokio::test]
    async fn test_auth() {
        let _ = dotenvy::dotenv();

        // 从环境变量获取凭据，如果不存在则跳过测试
        let email = env::var("SCREEPS_EMAIL").unwrap_or_else(|_| {
            println!("SCREEPS_EMAIL not set, skipping test");
            "test@example.com".to_string() // 占位符值
        });

        let password = env::var("SCREEPS_PASSWORD").unwrap_or_else(|_| {
            println!("SCREEPS_PASSWORD not set, skipping test");
            "password".to_string() // 占位符值
        });

        // 如果没有设置环境变量，则跳过测试
        if env::var("SCREEPS_EMAIL").is_err() || env::var("SCREEPS_PASSWORD").is_err() {
            println!(
                "Skipping authentication test because SCREEPS_EMAIL or SCREEPS_PASSWORD is not set"
            );
            return;
        }

        let config = ScreepsConfig::new(
            None,
            Some(email),
            Some(password),
            "screeps.com".to_string(),
            true,
            10000,
        );
        let client = ScreepsHttpClient::new(config);
        let result = client.auth().await;

        // 只有当设置了环境变量时才检查结果
        if env::var("SCREEPS_EMAIL").is_ok() && env::var("SCREEPS_PASSWORD").is_ok() {
            match result {
                Ok(data) => {
                    println!("Authentication successful: {:?}", data);
                    assert_eq!(data.base_data.ok.unwrap(), 1);
                }
                Err(e) => {
                    println!("Authentication failed: {:?}", e);
                    panic!("Authentication failed");
                }
            }
        }
    }
}
