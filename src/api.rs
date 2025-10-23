use crate::{
    EncodedRoomTerrainData, RoomTerrainData,
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

    /// 登录获取 token 数据
    pub async fn auth(&mut self) -> ScreepsResult<TokenData> {
        self.http_client.auth().await
    }

    /// 获取房间对象数据
    /// 参数：
    /// - room: 房间名称
    /// - shard: shard 名称
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

    /// 获取房间地形数据
    pub async fn get_room_terrain(
        &mut self,
        room: &str,
        shard: &str,
    ) -> ScreepsResult<RoomTerrainData> {
        self.http_client
            .request(
                Method::Get,
                "/game/room-terrain",
                Some(&[("room", room), ("shard", shard)]),
            )
            .await
    }

    /// 获取编码后的房间地形数据
    pub async fn get_room_terrain_encoded(
        &mut self,
        room: &str,
        shard: &str,
    ) -> ScreepsResult<EncodedRoomTerrainData> {
        self.http_client
            .request(
                Method::Get,
                "/game/room-terrain-encoded",
                Some(&[("room", room), ("shard", shard), ("encoded", "true")]),
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

    #[tokio::test]
    async fn test_get_room_terrain() {
        let mut api = ScreepsApi::new(ScreepsConfig::default());
        let room_terrain = api.get_room_terrain("E13S13", "shard3").await.unwrap();
        assert_eq!(room_terrain.base_data.ok, 1);
    }

    #[tokio::test]
    async fn test_get_room_terrain_encoded() {
        let mut api = ScreepsApi::new(ScreepsConfig::default());
        let room_terrain_encoded = api
            .get_room_terrain_encoded("E13S13", "shard3")
            .await
            .unwrap();
        assert_eq!(room_terrain_encoded.base_data.ok, 1);
    }
}
