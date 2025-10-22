use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

/// 头像数据
#[derive(Serialize, Deserialize, Debug)]
pub struct Badge {
    #[serde(rename = "type")]
    pub r#type: i32,
    pub color1: i32,
    pub color2: i32,
    pub color3: i32,
    pub param: i32,
    pub flip: bool,
}

/// 用户对象
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub badge: Badge,
}

/// 基本对象数据，每个对象都继承该结构
#[derive(Serialize, Deserialize, Debug)]
pub struct BaseObject {
    /// 对象 id
    pub _id: String,
    /// 对象类型
    pub r#type: String,
    /// x 坐标
    pub x: i32,
    /// y 坐标
    pub y: i32,
    /// 所在房间名
    pub room: String,
}

/// Source 对象
#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    #[serde(flatten)]
    pub base_object: BaseObject,
    pub energy: i32,
    #[serde(rename = "energyCapacity")]
    pub energy_capacity: i32,
    #[serde(rename = "ticksToRegeneration")]
    pub ticks_to_regeneration: i32,
    #[serde(rename = "invaderHarvested")]
    pub invader_harvested: i32,
    #[serde(rename = "nextRegenerationTime")]
    pub next_regeneration_time: i32,
}

/// Mineral 对象
#[derive(Serialize, Deserialize, Debug)]
pub struct Mineral {
    #[serde(flatten)]
    pub base_object: BaseObject,
    #[serde(rename = "mineralType")]
    pub mineral_type: String,
    #[serde(rename = "mineralAmount")]
    pub mineral_amount: i32,
    #[serde(rename = "nextRegenerationTime")]
    pub next_regeneration_time: i32,
}

/// ConstructedWall 对象
#[derive(Serialize, Deserialize, Debug)]
pub struct ConstructedWall {
    #[serde(flatten)]
    pub base_object: BaseObject,
    pub hits: i32,
    #[serde(rename = "hitsMax")]
    pub hits_max: i32,
    #[serde(rename = "notifyWhenAttacked")]
    pub notify_when_attacked: bool,
}

/// Road 对象
#[derive(Serialize, Deserialize, Debug)]
pub struct Road {
    #[serde(flatten)]
    pub base_object: BaseObject,
    pub hits: i32,
    #[serde(rename = "hitsMax")]
    pub hits_max: i32,
    #[serde(rename = "notifyWhenAttacked")]
    pub notify_when_attacked: bool,
    #[serde(rename = "nextDecayTime")]
    pub next_decay_time: i32,
}

/// Controller 对象的 Reservation 字段
#[derive(Serialize, Deserialize, Debug)]
pub struct Reservation {
    pub user: String,
    pub ticks_to_end: i32,
}

/// Controller 对象的 Sign 字段
#[derive(Serialize, Deserialize, Debug)]
pub struct Sign {
    pub user: String,
    pub time: i32,
    pub text: String,
    #[serde(rename = "datetime")]
    pub date_time: i64,
}

/// Controller 对象的 Effect 字段
#[derive(Serialize, Deserialize, Debug)]
pub struct Effect {
    pub effect: i32,
    pub power: i32,
    pub level: i32,
    #[serde(rename = "endTime")]
    pub end_time: i64,
}

/// Controller 对象
#[derive(Serialize, Deserialize, Debug)]
pub struct Controller {
    #[serde(flatten)]
    pub base_object: BaseObject,
    pub level: i32,
    pub progress: i32,
    #[serde(rename = "progressTotal")]
    pub progress_total: i32,
    pub user: String,
    #[serde(rename = "downgradeTime")]
    pub downgrade_time: i64,
    #[serde(rename = "safeMode")]
    pub safe_mode: i32,
    #[serde(rename = "safeModeAvailable")]
    pub safe_mode_available: i32,
    #[serde(rename = "safeModeCooldown")]
    pub safe_mode_cooldown: i64,
    #[serde(rename = "upgradeBlocked")]
    pub upgrade_blocked: i32,
    #[serde(rename = "downgradeBlocked")]
    pub downgrade_blocked: i32,
    pub reservation: Option<Reservation>,
    pub sign: Option<Sign>,
    #[serde(rename = "isPowerEnabled")]
    pub is_power_enabled: bool,
    pub effects: Vec<Effect>,
}

/// Spawn 对象的 Spawning 字段
#[derive(Serialize, Deserialize, Debug)]
pub struct Spawning {
    pub name: String,
    pub need_time: i32,
    pub remaining_time: i32,
}

/// Spawn 对象的 Store 字段
#[derive(Serialize, Deserialize, Debug)]
pub struct SpawnStore {
    pub energy: i32,
}

/// Spawn 对象
#[derive(Serialize, Deserialize, Debug)]
pub struct Spawn {
    #[serde(flatten)]
    pub base_object: BaseObject,
    pub name: String,
    pub hits: i32,
    #[serde(rename = "hitsMax")]
    pub hits_max: i32,
    #[serde(rename = "notifyWhenAttacked")]
    pub notify_when_attacked: bool,
    pub spawning: Option<Spawning>,
    pub off: bool,
    pub store: SpawnStore,
    #[serde(rename = "storeCapacityResource")]
    pub store_capacity_resource: SpawnStore,
}

/// Extension 对象
#[derive(Serialize, Deserialize, Debug)]
pub struct Extension {
    #[serde(flatten)]
    pub base_object: BaseObject,
    pub hits: i32,
    #[serde(rename = "hitsMax")]
    pub hits_max: i32,
    pub user: String,
    #[serde(rename = "notifyWhenAttacked")]
    pub notify_when_attacked: bool,
    pub store: SpawnStore,
    #[serde(rename = "storeCapacityResource")]
    pub store_capacity_resource: SpawnStore,
    pub off: bool,
}

/// Storage 对象的 Store 字段
pub type Store = HashMap<String, i32>;

/// Storage 对象
#[derive(Serialize, Deserialize, Debug)]
pub struct Storage {
    #[serde(flatten)]
    pub base_object: BaseObject,
    pub hits: i32,
    #[serde(rename = "hitsMax")]
    pub hits_max: i32,
    #[serde(rename = "notifyWhenAttacked")]
    pub notify_when_attacked: bool,
    pub user: String,
    pub store: Store,
    #[serde(rename = "storeCapacity")]
    pub store_capacity: i32,
    pub effects: HashMap<String, Effect>,
}

/// Tower 对象的 ActionLog 字段
#[derive(Serialize, Deserialize, Debug)]
pub struct ActionLog {
    pub attack: Option<String>,
    pub heal: Option<String>,
    pub repair: Option<String>,
}

/// Tower 对象
#[derive(Serialize, Deserialize, Debug)]
pub struct Tower {
    #[serde(flatten)]
    pub base_object: BaseObject,
    pub hits: i32,
    #[serde(rename = "hitsMax")]
    pub hits_max: i32,
    #[serde(rename = "notifyWhenAttacked")]
    pub notify_when_attacked: bool,
    pub user: String,
    pub store: SpawnStore,
    #[serde(rename = "storeCapacityResource")]
    pub store_capacity_resource: SpawnStore,
    #[serde(rename = "actionLog")]
    pub action_log: ActionLog,
}

/// Rampart 对象
#[derive(Serialize, Deserialize, Debug)]
pub struct Rampart {
    #[serde(flatten)]
    pub base_object: BaseObject,
    pub hits: i32,
    #[serde(rename = "hitsMax")]
    pub hits_max: i32,
    pub user: String,
    #[serde(rename = "notifyWhenAttacked")]
    pub notify_when_attacked: bool,
    #[serde(rename = "nextDecayTime")]
    pub next_decay_time: i64,
}

/// Extractor 对象
#[derive(Serialize, Deserialize, Debug)]
pub struct Extractor {
    #[serde(flatten)]
    pub base_object: BaseObject,
    pub hits: i32,
    #[serde(rename = "hitsMax")]
    pub hits_max: i32,
    pub user: String,
    #[serde(rename = "notifyWhenAttacked")]
    pub notify_when_attacked: bool,
    pub cooldown: i32,
}

/// Terminal 对象
#[derive(Serialize, Deserialize, Debug)]
pub struct Terminal {
    #[serde(flatten)]
    pub base_object: BaseObject,
    pub hits: i32,
    #[serde(rename = "hitsMax")]
    pub hits_max: i32,
    pub user: String,
    #[serde(rename = "notifyWhenAttacked")]
    pub notify_when_attacked: bool,
    pub store: Store,
    #[serde(rename = "storeCapacity")]
    pub store_capacity: i32,
    #[serde(rename = "cooldownTime")]
    pub cooldown_time: i32,
    pub send: Option<String>,
}

/// Observer 对象
#[derive(Serialize, Deserialize, Debug)]
pub struct Observer {
    #[serde(flatten)]
    pub base_object: BaseObject,
    pub hits: i32,
    #[serde(rename = "hitsMax")]
    pub hits_max: i32,
    pub user: String,
    #[serde(rename = "notifyWhenAttacked")]
    pub notify_when_attacked: bool,
    #[serde(rename = "observeRoom")]
    pub observe_room: Option<String>,
}

/// PowerSpawn 对象
#[derive(Serialize, Deserialize, Debug)]
pub struct PowerSpawn {
    #[serde(flatten)]
    pub base_object: BaseObject,
    pub hits: i32,
    #[serde(rename = "hitsMax")]
    pub hits_max: i32,
    pub user: String,
    #[serde(rename = "notifyWhenAttacked")]
    pub notify_when_attacked: bool,
    pub store: HashMap<String, i32>, // 包含 energy 和 power
    #[serde(rename = "storeCapacityResource")]
    pub store_capacity_resource: HashMap<String, i32>, // 包含 energy 和 power
}

/// Nuker 对象
#[derive(Serialize, Deserialize, Debug)]
pub struct Nuker {
    #[serde(flatten)]
    pub base_object: BaseObject,
    pub hits: i32,
    #[serde(rename = "hitsMax")]
    pub hits_max: i32,
    pub user: String,
    #[serde(rename = "notifyWhenAttacked")]
    pub notify_when_attacked: bool,
    pub store: HashMap<String, i32>, // 包含 energy 和 G
    #[serde(rename = "storeCapacityResource")]
    pub store_capacity_resource: HashMap<String, i32>, // 包含 energy 和 G
    #[serde(rename = "cooldownTime")]
    pub cooldown_time: i32,
}

/// Factory 对象
#[derive(Serialize, Deserialize, Debug)]
pub struct Factory {
    #[serde(flatten)]
    pub base_object: BaseObject,
    pub hits: i32,
    #[serde(rename = "hitsMax")]
    pub hits_max: i32,
    pub user: String,
    #[serde(rename = "notifyWhenAttacked")]
    pub notify_when_attacked: bool,
    pub store: Store,
    #[serde(rename = "storeCapacity")]
    pub store_capacity: i32,
    pub cooldown: i32,
    #[serde(rename = "cooldownTime")]
    pub cooldown_time: i32,
    pub effects: HashMap<String, Effect>,
    pub level: i32,
}

/// Creep 对象的 Body 字段
#[derive(Serialize, Deserialize, Debug)]
pub struct CreepBodyPart {
    #[serde(rename = "type")]
    pub part_type: String,
    pub hits: i32,
    pub boost: Option<String>,
}

/// Creep 对象
#[derive(Serialize, Deserialize, Debug)]
pub struct Creep {
    #[serde(flatten)]
    pub base_object: BaseObject,
    pub name: String,
    pub hits: i32,
    #[serde(rename = "hitsMax")]
    pub hits_max: i32,
    pub user: String,
    pub spawning: bool,
    pub fatigue: i32,
    pub body: Vec<CreepBodyPart>,
    pub store: Store,
    #[serde(rename = "storeCapacity")]
    pub store_capacity: i32,
    #[serde(rename = "notifyWhenAttacked")]
    pub notify_when_attacked: bool,
}

/// PowerCreep 对象
#[derive(Serialize, Deserialize, Debug)]
pub struct PowerCreep {
    #[serde(flatten)]
    pub base_object: BaseObject,
    pub name: String,
    pub hits: i32,
    #[serde(rename = "hitsMax")]
    pub hits_max: i32,
    pub user: String,
    pub spawning: bool,
    pub fatigue: i32,
    pub body: Vec<CreepBodyPart>,
    pub store: Store,
    #[serde(rename = "storeCapacity")]
    pub store_capacity: i32,
    #[serde(rename = "notifyWhenAttacked")]
    pub notify_when_attacked: bool,
    #[serde(rename = "className")]
    pub class_name: String,
    pub power: HashMap<i32, PowerInfo>,
}

/// PowerCreep 对象的 Power 字段
#[derive(Serialize, Deserialize, Debug)]
pub struct PowerInfo {
    pub level: i32,
    #[serde(rename = "cooldownTime")]
    pub cooldown_time: Option<i64>,
}
