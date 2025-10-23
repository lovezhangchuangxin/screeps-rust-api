use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    EncodedRoomTerrain, MyInfo, RoomStatus, RoomTerrain, UserInfo,
    types::{RoomObject, UserWithId},
};

/// 响应的基础数据，每个接口返回的数据结构都继承自该结构
#[derive(Serialize, Deserialize, Debug)]
pub struct BaseData {
    /// 1 成功，0 失败
    pub ok: i32,
    /// 错误信息
    pub error: Option<String>,
}

/// auth 认证返回 token 数据
#[derive(Serialize, Deserialize, Debug)]
pub struct TokenData {
    #[serde(flatten)]
    pub base_data: BaseData,
    /// token
    pub token: Option<String>,
}

/// 我的信息数据
#[derive(Serialize, Deserialize, Debug)]
pub struct MyInfoData {
    #[serde(flatten)]
    pub base_data: BaseData,
    #[serde(flatten)]
    pub user: Option<MyInfo>,
}

/// 其他用户信息数据
#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfoData {
    #[serde(flatten)]
    pub base_data: BaseData,
    pub user: Option<UserInfo>,
}

/// 我的名字数据
#[derive(Serialize, Deserialize, Debug)]
pub struct MyNameData {
    #[serde(flatten)]
    pub base_data: BaseData,
    pub username: Option<String>,
}

/// 玩家的所有房间
#[derive(Serialize, Deserialize, Debug)]
pub struct UserAllRoomsData {
    #[serde(flatten)]
    pub base_data: BaseData,
    /// 占有的房间
    pub shards: Option<HashMap<String, Vec<String>>>,
    /// 预定房间
    pub reservations: Option<HashMap<String, Vec<String>>>,
}

/// 房间对象数据
#[derive(Serialize, Deserialize, Debug)]
pub struct RoomObjectsData {
    #[serde(flatten)]
    pub base_data: BaseData,
    pub objects: Option<Vec<RoomObject>>,
    pub users: Option<HashMap<String, UserWithId>>,
}

/// 房间地形数据
#[derive(Serialize, Deserialize, Debug)]
pub struct RoomTerrainData {
    #[serde(flatten)]
    pub base_data: BaseData,
    pub terrain: Option<Vec<RoomTerrain>>,
}

/// 编码后的房间数据
#[derive(Serialize, Deserialize, Debug)]
pub struct EncodedRoomTerrainData {
    #[serde(flatten)]
    pub base_data: BaseData,
    pub terrain: Option<Vec<EncodedRoomTerrain>>,
}

/// 房间状态数据
#[derive(Serialize, Deserialize, Debug)]
pub struct RoomStatusData {
    #[serde(flatten)]
    pub base_data: BaseData,
    pub rooms: Option<RoomStatus>,
}
