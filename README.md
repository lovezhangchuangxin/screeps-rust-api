# screeps-rust-api

一个用于 Screeps 游戏的 Rust 语言 API 接口库。

目前处于快速迭代中。**很多功能还不完善，敬请期待**。

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
```
