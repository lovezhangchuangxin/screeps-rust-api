# screeps-rust-api

一个用于 Screeps 游戏的 Rust 语言 API 接口库。

目前处于快速迭代中。**很多功能还不完善，敬请期待**。

## 功能特性

- 异步 HTTP 客户端支持
- 自动速率限制处理
- Screeps API 接口封装
- 支持认证和 Token 管理

## 使用示例

```rust
use screeps_rust_api::{ScreepsApi, ScreepsConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ScreepsConfig::new(
        Some("your_token".to_string()),
        None,
        None,
        "https://screeps.com".to_string(),
        true,
        10,
    );

    let api = ScreepsApi::new(config);

    // 获取用户信息
    let user_info = api.get_my_info().await?;
    println!("User ID: {}", user_info.user.unwrap()._id);

    // 获取房间对象
    let room_objects = api.get_room_objects("E13S13", "shard3").await?;
    println!(
        "Found {} objects in room",
        room_objects.objects.unwrap().len()
    );

    Ok(())
}
```

更多用法可以查看 `examples` 下的示例代码。

## 支持的 API 接口

### 用户相关

- `get_my_info()` - 获取当前用户信息
- `get_my_name()` - 获取当前用户名
- `get_user_info_by_name(username)` - 根据用户名获取用户信息
- `get_user_info_by_id(id)` - 根据用户 ID 获取用户信息
- `get_user_rooms(id)` - 获取指定用户的所有房间

### 房间相关

- `get_room_objects(room, shard)` - 获取房间内所有对象
- `get_room_terrain(room, shard)` - 获取房间地形信息
- `get_room_terrain_encoded(room, shard)` - 获取编码后的房间地形信息
- `get_room_status(room, shard)` - 获取房间状态

### 游戏相关

- `get_shards()` - 获取所有 shard 信息
- `get_shard_time(shard)` - 获取指定 shard 的游戏时间

### 认证相关

- `auth()` - 用户认证获取 token

## 构建

```bash
cargo build
```

## 测试

```bash
cargo test
```

注意：某些测试需要有效的 Screeps 账户凭据，这些凭据通过环境变量提供。

要运行需要认证的测试，请创建一个 `.env` 文件并设置以下环境变量：

```env
SCREEPS_EMAIL=your_email@example.com
SCREEPS_PASSWORD=your_password
SCREEPS_TOKEN=your_token
```
