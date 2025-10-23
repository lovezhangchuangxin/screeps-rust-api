use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    EncodedRoomTerrain, RoomStatus, RoomTerrain,
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
    pub token: String,
}

/// 房间对象数据
#[derive(Serialize, Deserialize, Debug)]
pub struct RoomObjectsData {
    #[serde(flatten)]
    pub base_data: BaseData,
    pub objects: Vec<RoomObject>,
    pub users: HashMap<String, UserWithId>,
}

/// 房间地形数据
#[derive(Serialize, Deserialize, Debug)]
pub struct RoomTerrainData {
    #[serde(flatten)]
    pub base_data: BaseData,
    pub terrain: Vec<RoomTerrain>,
}

/// 编码后的房间数据
#[derive(Serialize, Deserialize, Debug)]
pub struct EncodedRoomTerrainData {
    #[serde(flatten)]
    pub base_data: BaseData,
    pub terrain: Vec<EncodedRoomTerrain>,
}

/// 房间状态数据
#[derive(Serialize, Deserialize, Debug)]
pub struct RoomStatusData {
    #[serde(flatten)]
    pub base_data: BaseData,
    pub rooms: Option<RoomStatus>,
}
