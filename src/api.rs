use serde::{Serialize, de::DeserializeOwned};

use crate::{
    EncodedRoomTerrainData, MyInfoData, MyNameData, RoomStatusData, RoomTerrainData,
    UserAllRoomsData, UserInfoData,
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
    pub async fn auth(&mut self) -> ScreepsResult<TokenData> {
        self.http_client.auth().await
    }

    /// 请求接口
    pub async fn request<T: Serialize, U: DeserializeOwned>(
        &mut self,
        method: Method,
        path: &str,
        body: Option<T>,
    ) -> ScreepsResult<U> {
        self.http_client.request(method, path, body).await
    }

    /// 获取自己的信息数据
    pub async fn get_my_info(&mut self) -> ScreepsResult<MyInfoData> {
        self.request::<AnyPayload, MyInfoData>(Get, "/auth/me", None)
            .await
    }

    /// 获取我的名字
    pub async fn get_my_name(&mut self) -> ScreepsResult<MyNameData> {
        self.request::<AnyPayload, MyNameData>(Get, "/user/name", None)
            .await
    }

    /// 根据用户id查找用户信息数据
    pub async fn get_user_info_by_id(&mut self, id: String) -> ScreepsResult<UserInfoData> {
        self.request(Get, "/user/find", Some(&[("id", id)])).await
    }

    /// 根据用户名查找用户信息数据
    pub async fn get_user_info_by_name(&mut self, name: String) -> ScreepsResult<UserInfoData> {
        self.request(Get, "/user/find", Some(&[("username", name)]))
            .await
    }

    /// 获取玩家的所有房间
    /// 参数：
    /// - id: 玩家 id
    pub async fn get_user_rooms(&mut self, id: String) -> ScreepsResult<UserAllRoomsData> {
        self.request(Get, "/user/rooms", Some(&[("id", id)])).await
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
        self.request(
            Get,
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
        self.request(
            Get,
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
        self.request(
            Get,
            "/game/room-terrain",
            Some(&[("room", room), ("shard", shard), ("encoded", "true")]),
        )
        .await
    }

    /// 获取房间状态数据
    pub async fn get_room_status(
        &mut self,
        room: &str,
        shard: &str,
    ) -> ScreepsResult<RoomStatusData> {
        self.request(
            Get,
            "/game/room-status",
            Some(&[("room", room), ("shard", shard)]),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screeps_api_from_env;

    #[tokio::test]
    async fn test_get_my_info() {
        let mut api = screeps_api_from_env!().unwrap();
        let my_info = api.get_my_info().await.unwrap();
        // println!("{}", my_info.user._id);
        assert_eq!(my_info.base_data.ok, 1);
    }

    #[tokio::test]
    async fn test_get_my_name() {
        let mut api = screeps_api_from_env!().unwrap();
        let my_name = api.get_my_name().await.unwrap();
        assert_eq!(my_name.base_data.ok, 1);
    }

    #[tokio::test]
    async fn test_get_user_info_by_id() {
        let mut api = screeps_api_from_env!().unwrap();
        let user_info = api
            .get_user_info_by_id("61f26f882181b7ba48c7015c".to_string())
            .await
            .unwrap();
        assert_eq!(user_info.base_data.ok, 1);
    }

    #[tokio::test]
    async fn test_get_user_info_by_name() {
        let mut api = ScreepsApi::new(ScreepsConfig::default());
        let user_info = api
            .get_user_info_by_name("keqing".to_string())
            .await
            .unwrap();
        assert_eq!(user_info.base_data.ok, 1);
    }

    #[tokio::test]
    async fn test_get_user_rooms() {
        let mut api = ScreepsApi::new(ScreepsConfig::default());
        let user_rooms = api
            .get_user_rooms("61f26f882181b7ba48c7015c".to_string())
            .await
            .unwrap();
        assert_eq!(user_rooms.base_data.ok, 1);
    }

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

    #[tokio::test]
    async fn test_get_room_status() {
        let mut api = screeps_api_from_env!().unwrap();
        let room_status = api.get_room_status("E13S13", "shard3").await.unwrap();
        assert_eq!(room_status.base_data.ok, 1);
    }
}
