use serde::{Serialize, de::DeserializeOwned};

use crate::{
    AllShardData, EncodedRoomTerrainData, MyInfoData, MyNameData, RoomStatusData, RoomTerrainData,
    ShardTimeData, UserAllRoomsData, UserInfoData,
    config::ScreepsConfig,
    error::ScreepsResult,
    http_client::*,
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
    pub async fn auth(&self) -> ScreepsResult<TokenData> {
        self.http_client.auth().await
    }

    /// 请求接口
    pub async fn request<T: Serialize, U: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<T>,
    ) -> ScreepsResult<U> {
        self.http_client.request(method, path, body).await
    }

    /// 获取自己的信息数据
    pub async fn get_my_info(&self) -> ScreepsResult<MyInfoData> {
        self.request::<AnyPayload, MyInfoData>(Get, "/auth/me", None)
            .await
    }

    /// 获取我的名字
    pub async fn get_my_name(&self) -> ScreepsResult<MyNameData> {
        self.request::<AnyPayload, MyNameData>(Get, "/user/name", None)
            .await
    }

    /// 获取玩家的所有房间
    pub async fn get_user_rooms(&self, id: &str) -> ScreepsResult<UserAllRoomsData> {
        self.request(Get, "/user/rooms", Some(&[("id", id)])).await
    }

    /// 根据用户名获取玩家信息
    pub async fn get_user_info_by_name(&self, username: &str) -> ScreepsResult<UserInfoData> {
        self.request(Get, "/user/find", Some(&[("username", username)]))
            .await
    }

    /// 根据用户ID获取玩家信息
    pub async fn get_user_info_by_id(&self, id: &str) -> ScreepsResult<UserInfoData> {
        self.request(Get, "/user/find", Some(&[("id", id)])).await
    }

    /// 获取房间对象数据
    /// 参数：
    /// - room: 房间名称
    /// - shard: shard 名称
    pub async fn get_room_objects(
        &self,
        room: &str,
        shard: &str,
    ) -> ScreepsResult<RoomObjectsData> {
        self.request(
            Get,
            "/game/room-objects",
            Some(&[("room", room), ("shard", shard)]),
        )
        .await
    }

    /// 获取房间地形数据
    pub async fn get_room_terrain(
        &self,
        room: &str,
        shard: &str,
    ) -> ScreepsResult<RoomTerrainData> {
        self.request(
            Get,
            "/game/room-terrain",
            Some(&[("room", room), ("shard", shard)]),
        )
        .await
    }

    /// 获取编码后的房间地形数据
    pub async fn get_room_terrain_encoded(
        &self,
        room: &str,
        shard: &str,
    ) -> ScreepsResult<EncodedRoomTerrainData> {
        self.request(
            Get,
            "/game/room-terrain",
            Some(&[("room", room), ("shard", shard), ("encoded", "true")]),
        )
        .await
    }

    /// 获取房间状态数据
    pub async fn get_room_status(&self, room: &str, shard: &str) -> ScreepsResult<RoomStatusData> {
        self.request(
            Get,
            "/game/room-status",
            Some(&[("room", room), ("shard", shard)]),
        )
        .await
    }

    /// 获取所有shard的信息
    pub async fn get_shards(&self) -> ScreepsResult<AllShardData> {
        self.request::<AnyPayload, AllShardData>(Get, "/game/shards/info", None)
            .await
    }

    /// 获取指定 shard 的游戏时间
    pub async fn get_shard_time(&self, shard: &str) -> ScreepsResult<ShardTimeData> {
        self.request(Get, "/game/time", Some(&[("shard", shard)]))
            .await
    }
}

impl Default for ScreepsApi {
    /// 默认实现只能调用无 token 要求的接口
    fn default() -> Self {
        let http_client = ScreepsHttpClient::new(ScreepsConfig::default());
        Self { http_client }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screeps_api_from_env;

    #[tokio::test]
    async fn test_get_my_info() {
        let api = screeps_api_from_env!().unwrap();
        let my_info = api.get_my_info().await.unwrap();
        // println!("{}", my_info.user._id);
        assert_eq!(my_info.base_data.ok.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_get_my_name() {
        let api = screeps_api_from_env!().unwrap();
        let my_name = api.get_my_name().await.unwrap();
        assert_eq!(my_name.base_data.ok.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_get_user_info_by_id() {
        let api = screeps_api_from_env!().unwrap();
        let user_info = api
            .get_user_info_by_id("61f26f882181b7ba48c7015c")
            .await
            .unwrap();
        assert_eq!(user_info.base_data.ok.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_get_user_info_by_name() {
        let api = ScreepsApi::new(ScreepsConfig::default());
        let user_info = api.get_user_info_by_name("keqing").await.unwrap();
        assert_eq!(user_info.base_data.ok.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_get_user_rooms() {
        let api = ScreepsApi::default();
        let user_rooms = api
            .get_user_rooms("61f26f882181b7ba48c7015c")
            .await
            .unwrap();
        assert_eq!(user_rooms.base_data.ok.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_get_room_objects() {
        let api = ScreepsApi::default();
        let room_objects = api.get_room_objects("E13S13", "shard3").await;
        match room_objects {
            Ok(room_objects) => {
                assert_eq!(room_objects.base_data.ok.unwrap(), 1);
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[tokio::test]
    async fn test_get_room_terrain() {
        let api = ScreepsApi::default();
        let room_terrain = api.get_room_terrain("E21N19", "shard0").await.unwrap();
        assert_eq!(room_terrain.base_data.ok.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_get_room_terrain_encoded() {
        let api = ScreepsApi::default();
        let room_terrain_encoded = api
            .get_room_terrain_encoded("E13S13", "shard3")
            .await
            .unwrap();
        assert_eq!(room_terrain_encoded.base_data.ok.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_get_room_status() {
        let api = screeps_api_from_env!().unwrap();
        let room_status = api.get_room_status("E13S13", "shard3").await.unwrap();
        assert_eq!(room_status.base_data.ok.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_get_shards() {
        let api = ScreepsApi::default();
        let my_info = api.get_shards().await.unwrap();
        assert_eq!(my_info.base_data.ok.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_get_shard_time() {
        let api = ScreepsApi::default();
        let game_time = api.get_shard_time("shard3").await.unwrap();
        assert_eq!(game_time.base_data.ok.unwrap(), 1);
    }
}
