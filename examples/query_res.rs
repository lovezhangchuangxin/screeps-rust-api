use std::{collections::HashMap, error::Error};

// 查询玩家每个shard具有的资源
use screeps_rust_api::{RoomObject, ScreepsApi, ScreepsError, ScreepsResult, screeps_api_from_env};

#[tokio::main]
async fn main() -> ScreepsResult<()> {
    let mut api = screeps_api_from_env!().unwrap();
    let res = query_res(&mut api, "6g3y", "all").await;
    match res {
        Ok(res) => {
            println!("{:?}", res);
        }
        Err(e) => {
            if let ScreepsError::Http(e) = e {
                println!("{} {:?}", e, e.source())
            }
        }
    }

    Ok(())
}

async fn query_res(
    api: &mut ScreepsApi,
    username: &str,
    target_shard: &str,
) -> ScreepsResult<HashMap<String, HashMap<String, i32>>> {
    let mut result = HashMap::new();

    // 先根据玩家信息查玩家的 id
    let user_info = api.get_user_info_by_name(username).await?;
    if user_info.base_data.ok != 1 {
        return Err(ScreepsError::Api("玩家不存在".to_string()));
    }

    let user_id = user_info.user.unwrap()._id;
    // 再根据玩家 id 查玩家所有房间
    let user_rooms = api.get_user_rooms(&user_id).await?;
    if user_rooms.base_data.ok != 1 {
        return Err(ScreepsError::Api("玩家没有房间".to_string()));
    }

    // 收集所有需要查询的房间和 shard 信息
    let mut room_shard_pairs = Vec::new();
    for (shard, rooms) in user_rooms.shards.unwrap().iter() {
        if target_shard != "all" && shard != target_shard {
            continue;
        }
        println!("开始处理 shard: {}", shard);
        for room in rooms {
            room_shard_pairs.push((room.clone(), shard.clone()));
        }
    }

    // 创建所有 future
    let futures: Vec<_> = room_shard_pairs
        .iter()
        .map(|(room, shard)| api.get_room_objects(room, shard))
        .collect();

    // 执行所有请求
    let responses = futures::future::join_all(futures).await;
    // 处理响应
    for (response, (room, shard)) in responses.into_iter().zip(room_shard_pairs.iter()) {
        match response {
            Ok(room_objects) => {
                if room_objects.base_data.ok != 1 {
                    eprintln!(
                        "Failed to fetch objects for room {} in shard {}, reason: {}",
                        room,
                        shard,
                        room_objects.base_data.error.unwrap()
                    );
                    continue;
                }
                let shard_res_map = result.entry(shard.clone()).or_insert_with(HashMap::new);
                for room_object in room_objects.objects.unwrap() {
                    match room_object {
                        RoomObject::Storage(storage) => {
                            for (resource_type, amount) in storage.store.iter() {
                                let amount = amount.unwrap_or(0);
                                shard_res_map
                                    .entry(resource_type.to_string())
                                    .and_modify(|a| *a += amount)
                                    .or_insert(amount);
                            }
                        }
                        RoomObject::Terminal(terminal) => {
                            for (resource_type, amount) in terminal.store.iter() {
                                let amount = amount.unwrap_or(0);
                                shard_res_map
                                    .entry(resource_type.to_string())
                                    .and_modify(|a| *a += amount)
                                    .or_insert(amount);
                            }
                        }
                        RoomObject::Factory(link) => {
                            for (resource_type, amount) in link.store.iter() {
                                let amount = amount.unwrap_or(0);
                                shard_res_map
                                    .entry(resource_type.to_string())
                                    .and_modify(|a| *a += amount)
                                    .or_insert(amount);
                            }
                        }
                        _ => {
                            continue;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "Failed to fetch objects for room {} in shard {}: {}",
                    room, shard, e
                );
                return Err(e);
            }
        }
    }

    Ok(result)
}
