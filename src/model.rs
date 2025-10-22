use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TokenData {
    /// 1 成功，0 表示有错误
    pub ok: i32,
    /// 错误信息
    pub error: Option<String>,
    /// token
    pub token: String,
}
