use crate::{
    config::ScreepsConfig,
    error::ScreepsResult,
    http_client::{Method, ScreepsHttpClient},
    model::{RoomObjectsData, TokenData},
};

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

    pub async fn auth(&mut self) -> ScreepsResult<TokenData> {
        self.http_client.auth().await
    }

    pub async fn get_room_objects(
        &mut self,
        room: &str,
        shard: &str,
    ) -> ScreepsResult<RoomObjectsData> {
        self.http_client
            .request(
                Method::Get,
                "/game/room-objects",
                Some(&[("room", room), ("shard", shard)]),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_room_objects() {
        let mut api = ScreepsApi::new(ScreepsConfig::default());
        let room_objects = api.get_room_objects("E13S13", "shard3").await;
        match room_objects {
            Ok(room_objects) => {
                assert_eq!(room_objects.base_data.ok, 1);
            }
            Err(err) => panic!("{}", err),
        }
    }
}
