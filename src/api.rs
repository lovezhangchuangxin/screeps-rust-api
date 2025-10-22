use crate::{config::ScreepsConfig, http_client::ScreepsHttpClient};

/// Screeps Api
pub struct ScreepsApi {
    /// http 客户端
    pub http_client: ScreepsHttpClient,
}

impl ScreepsApi {
    pub fn new(config: ScreepsConfig) -> Self {
        let http_client = ScreepsHttpClient::new(config);
        Self { http_client }
    }
}
