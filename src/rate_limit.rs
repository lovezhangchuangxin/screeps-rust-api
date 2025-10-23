use std::collections::HashMap;

use reqwest::header::HeaderMap;

use crate::http_client::Method;

/// 接口速度限制
#[derive(Debug, Clone)]
pub struct RateLimit {
    /// 限制次数
    pub limit: i32,
    /// 限制时期
    pub period: Period,
    /// 剩余次数
    pub remaining: i32,
    /// 重置时间，单位 s
    pub reset: u128,
}

impl RateLimit {
    /// 创建一个接口限制
    pub fn new(limit: i32, period: Period, remaining: i32, reset: u128) -> Self {
        Self {
            limit,
            period,
            remaining,
            reset,
        }
    }

    /// 构造默认的速度限制
    pub fn default(limit: i32, period: Period) -> Self {
        Self::new(limit, period, limit, 0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Period {
    Minute,
    Hour,
    Day,
}

/// 接口速度限制集合
pub struct RateLimits {
    /// 全局限制
    pub global_limit: RateLimit,
    /// GET 请求限制
    pub get_limits: HashMap<String, RateLimit>,
    /// POST 请求限制
    pub post_limits: HashMap<String, RateLimit>,
}

impl RateLimits {
    /// 获取指定请求方法和请求路径的限速信息，找不到返回 global 的限速信息
    /// 参数：
    /// - method: 请求方法，GET or POST
    /// - path: 请求路径
    pub fn get_limit(&self, method: &Method, path: &str) -> RateLimit {
        match method {
            Method::Get => self.get_limits.get(path).cloned().unwrap_or(self.global_limit.clone()),
            Method::Post => self.post_limits.get(path).cloned().unwrap_or(self.global_limit.clone()),
        }
    }

    /// 获取指定请求方法和请求路径的限速信息，找不到返回 global 的限速信息
    /// 参数：
    /// - method: 请求方法，GET or POST
    /// - path: 请求路径
    pub fn get_limit_mut(&mut self, method: &Method, path: &str) -> &mut RateLimit {
        match method {
            Method::Get => self
                .get_limits
                .get_mut(path)
                .unwrap_or(&mut self.global_limit),
            Method::Post => self
                .post_limits
                .get_mut(path)
                .unwrap_or(&mut self.global_limit),
        }
    }

    /// 更新限速信息
    /// 参数：
    /// - method: 请求方法，GET or POST
    /// - path: 请求路径
    /// - limit: 限速信息
    pub fn update_limit(&mut self, method: &Method, path: &str, rate_limit: RateLimit) {
        let now_rate_limit = self.get_limit_mut(method, path);
        *now_rate_limit = rate_limit;
    }

    /// 根据响应头更新限速信息
    pub fn update_from_headers(&mut self, method: &Method, path: &str, headers: &HeaderMap) {
        let limit = headers
            .get("x-ratelimit-limit")
            .and_then(|s| s.to_str().unwrap_or("0").parse::<i32>().ok())
            .unwrap_or(0);
        if limit <= 0 {
            return;
        }
        let remaining = headers
            .get("x-ratelimit-remaining")
            .and_then(|s| s.to_str().unwrap_or("0").parse::<i32>().ok())
            .unwrap_or(0);
        let reset = headers
            .get("x-ratelimit-reset")
            .and_then(|s| s.to_str().unwrap_or("0").parse::<u128>().ok())
            .unwrap_or(0);
        let rate_limit = self.get_limit_mut(method, path);
        rate_limit.limit = limit;
        rate_limit.remaining = remaining;
        rate_limit.reset = reset;
    }
}

impl Default for RateLimits {
    fn default() -> Self {
        let global_limit = RateLimit::default(120, Period::Minute);
        let mut get_limits = HashMap::new();
        let mut post_limits = HashMap::new();

        // GET 请求限制
        get_limits.insert(
            "/game/room-terrain".to_string(),
            RateLimit::default(360, Period::Hour),
        );
        get_limits.insert(
            "/user/code".to_string(),
            RateLimit::default(60, Period::Hour),
        );
        get_limits.insert(
            "/user/memory".to_string(),
            RateLimit::default(1440, Period::Day),
        );
        get_limits.insert(
            "/user/memory-segment".to_string(),
            RateLimit::default(360, Period::Hour),
        );
        get_limits.insert(
            "/game/market/orders-index".to_string(),
            RateLimit::default(60, Period::Hour),
        );
        get_limits.insert(
            "/game/market/orders".to_string(),
            RateLimit::default(60, Period::Hour),
        );
        get_limits.insert(
            "/game/market/my-orders".to_string(),
            RateLimit::default(60, Period::Hour),
        );
        get_limits.insert(
            "/game/market/stats".to_string(),
            RateLimit::default(60, Period::Hour),
        );
        get_limits.insert(
            "/game/user/money-history".to_string(),
            RateLimit::default(60, Period::Hour),
        );

        // POST 请求限制
        post_limits.insert(
            "/user/console".to_string(),
            RateLimit::default(360, Period::Hour),
        );
        post_limits.insert(
            "/game/map-stats".to_string(),
            RateLimit::default(60, Period::Hour),
        );
        post_limits.insert(
            "/user/code".to_string(),
            RateLimit::default(240, Period::Day),
        );
        post_limits.insert(
            "/user/set-active-branch".to_string(),
            RateLimit::default(240, Period::Day),
        );
        post_limits.insert(
            "/user/memory".to_string(),
            RateLimit::default(240, Period::Day),
        );
        post_limits.insert(
            "/user/memory-segment".to_string(),
            RateLimit::default(60, Period::Hour),
        );

        Self {
            global_limit,
            get_limits,
            post_limits,
        }
    }
}
