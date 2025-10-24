// 实现获取指定玩家的 GCL 和 GPL

use screeps_rust_api::{ScreepsApi, ScreepsError, ScreepsResult};

#[tokio::main]
async fn main() {
    let api = ScreepsApi::default();
    let username = "keqing";
    let (gcl, gpl) = get_gcl_gpl(&api, username).await.unwrap();
    println!("user {username}: GCL: {}, GPL: {}", gcl, gpl);
}

async fn get_gcl_gpl(api: &ScreepsApi, username: &str) -> ScreepsResult<(i32, i32)> {
    let user_info = api.get_user_info_by_name(username).await?;
    if user_info.base_data.ok.unwrap_or(0) != 1 {
        return Err(ScreepsError::Api(format!(
            "获取 {} 信息失败，错误: {}",
            username,
            user_info.base_data.error.unwrap()
        )));
    }
    let user_info = user_info.user.unwrap();
    let gcl = calculate_gcl(user_info.gcl);
    let gpl = calculate_gpl(user_info.power);

    Ok((gcl, gpl))
}

/// GCL 计算公式
fn calculate_gcl(gcl: u64) -> i32 {
    ((gcl as f64) / 1000000.0).powf(1.0 / 2.4).floor() as i32 + 1
}

/// GPL 计算公式
fn calculate_gpl(power: u64) -> i32 {
    ((power as f64) / 1000.0).powf(0.5).floor() as i32
}
