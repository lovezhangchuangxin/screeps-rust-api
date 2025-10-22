/// Screeps 配置
#[derive(Debug)]
pub struct ScreepsConfig {
    /// 游戏 token
    pub token: Option<String>,
    /// 游戏绑定的邮箱
    pub email: Option<String>,
    /// 游戏密码
    pub password: Option<String>,
    /// 游戏主机地址：域名+端口
    pub host: String,
    /// 是否使用 https
    pub secure: bool,
    /// 请求超时时间
    pub timeout: u64,
}

impl ScreepsConfig {
    pub fn new(
        token: Option<String>,
        email: Option<String>,
        password: Option<String>,
        host: String,
        secure: bool,
        timeout: u64,
    ) -> Self {
        Self {
            token,
            email,
            password,
            host,
            secure,
            timeout,
        }
    }

    /// 设置 token
    pub fn with_token(&mut self, token: String) {
        self.token = Some(token);
    }

    /// 设置邮箱
    pub fn with_email(&mut self, email: String) {
        self.email = Some(email);
    }

    /// 设置密码
    pub fn with_password(&mut self, password: String) {
        self.password = Some(password);
    }

    /// 设置游戏主机地址
    pub fn with_host(&mut self, host: String) {
        self.host = host;
    }

    /// 设置是否启用 https
    pub fn with_secure(&mut self, secure: bool) {
        self.secure = secure;
    }

    /// 设置请求超时
    pub fn with_timeout(&mut self, timeout: u64) {
        self.timeout = timeout;
    }

    /// 构造游戏服务器请求 url 前缀 url
    pub fn build_base_url(&self) -> String {
        let protocol = if self.secure { "https" } else { "http" };
        format!("{}://{}/api", protocol, self.host)
    }
}

impl Default for ScreepsConfig {
    fn default() -> Self {
        Self {
            token: None,
            email: None,
            password: None,
            host: "screeps.com".to_string(),
            secure: true,
            timeout: 15,
        }
    }
}
