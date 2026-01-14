# Screeps API Endpoints 实现情况（screeps-rust-api）

以下是根据 `screeps-rust-api` 仓库目前实际代码和文档，对照 [`screepers/python-screeps` Endpoints.md](https://github.com/screepers/python-screeps/blob/b4dd23c4d7d987ea73bfc53faca445abd4f5b58d/docs/Endpoints.md) 提供的 API 端点判定的实现状态。

✅ 已实现（或明确有接口封装）  
❌ 暂未实现/缺失或未知

---

## 1. Authentication & Registration

- ✅ [POST] `/api/auth/signin` （`auth()` 实现、由 `ScreepsHttpClient::auth` 使用）
- ❌ `/api/auth/steam-ticket` 
- ❌ `/api/auth/query-token`
- ❌ `/api/register/check-email`
- ❌ `/api/register/check-username`
- ❌ `/api/register/set-username`
- ❌ `/api/register/submit`

## 2. Messaging

- ❌ `/api/user/messages/index`
- ❌ `/api/user/messages/list`
- ❌ `/api/user/messages/unread-count`
- ❌ `/api/user/messages/send`
- ❌ `/api/user/messages/mark-read`

## 3. Room 信息 & 地图相关

- ✅ [GET] `/api/game/room-objects` （`get_room_objects()`）
- ✅ [GET] `/api/game/room-terrain` （`get_room_terrain()`、`get_room_terrain_encoded()`）
- ✅ [GET] `/api/game/room-status` （`get_room_status()`）
- ❌ `/api/game/room-overview`
- ❌ `/api/experimental/pvp`
- ❌ `/api/experimental/nukes`

## 4. 市场（Market）

- ❌ `/api/game/market/orders-index`
- ❌ `/api/game/market/my-orders`
- ❌ `/api/game/market/orders`
- ❌ `/api/user/money-history`

## 5. 排行榜 & 赛季

- ❌ `/api/leaderboard/seasons`
- ❌ `/api/leaderboard/find`
- ❌ `/api/leaderboard/list`

## 6. 用户信息 & 设置

- ✅ [GET] `/api/auth/me` （`get_my_info()`）
- ✅ [GET] `/api/user/name` (`get_my_name()`)
- ✅ [GET] `/api/user/find?username=...`/`?id=...`（`get_user_info_by_name()`/`get_user_info_by_id()`）
- ✅ [GET] `/api/user/rooms` (`get_user_rooms()`)
- ❌ `/api/user/stats`
- ❌ `/api/user/overview`
- ❌ `/api/user/respawn-prohibited-rooms`
- ❌ `/api/user/world-size`
- ❌ `/api/user/world-status`
- ❌ `/api/user/world-start-room`
- ❌ `/api/user/console`
- ❌ `/api/user/memory`
- ❌ `/api/user/memory-segment`
- ❌ `/api/user/code`
- ❌ `/api/user/branches`
- ❌ `/api/user/set-active-branch`
- ❌ `/api/user/clone-branch`
- ❌ `/api/user/delete-branch`
- ❌ `/api/user/notify-prefs`
- ❌ `/api/user/tutorial-done`
- ❌ `/api/user/badge`

## 7. 游戏世界操作

- ❌ `/api/game/gen-unique-object-name`
- ❌ `/api/game/check-unique-object-name`
- ❌ `/api/game/gen-unique-flag-name`
- ❌ `/api/game/create-flag`
- ❌ `/api/game/change-flag`
- ❌ `/api/game/change-flag-color`
- ❌ `/api/game/remove-flag`
- ❌ `/api/game/add-object-intent`
- ❌ `/api/game/set-notify-when-attacked`
- ❌ `/api/game/create-construction`
- ❌ `/api/game/place-spawn`
- ❌ `/api/game/create-invader`
- ❌ `/api/game/remove-invader`

## 8. Decorations

- ❌ `/api/decorations/inventory`
- ❌ `/api/decorations/themes`
- ❌ `/api/decorations/convert`
- ❌ `/api/decorations/pixelize`
- ❌ `/api/decorations/activate`
- ❌ `/api/decorations/deactivate`

## 9. 其他接口

- ✅ [GET] `/api/game/shards/info` (`get_shards()`)
- ✅ [GET] `/api/game/time` (`get_shard_time()`)
- ❌ `/api/servers/list`
- ❌ `/api/version`
- ❌ `/api/game/map-stats`
- ❌ `/room-history/shard0/{ROOM}/{TICK}.json`
- ❌ `/api/user/activate-ptr`
- ❌ `/api/scoreboard/list`

---
