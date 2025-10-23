/// 创建一个宏来简化 ScreepsApi 的实例化
#[macro_export]
macro_rules! screeps_api_from_env {
    () => {{
        let _ = dotenvy::dotenv();

        let email = std::env::var("SCREEPS_EMAIL").unwrap_or_else(|_| {
            eprintln!("SCREEPS_EMAIL not set, using placeholder");
            "".to_string()
        });

        let password = std::env::var("SCREEPS_PASSWORD").unwrap_or_else(|_| {
            eprintln!("SCREEPS_PASSWORD not set, using placeholder");
            "".to_string()
        });

        let token = std::env::var("SCREEPS_TOKEN").unwrap_or_else(|_| {
            eprintln!("SCREEPS_TOKEN not set, using empty string");
            "".to_string()
        });

        // 检查是否至少有一个认证方法可用
        if std::env::var("SCREEPS_EMAIL").is_err()
            && std::env::var("SCREEPS_PASSWORD").is_err()
            && token.is_empty()
        {
            eprintln!("No authentication credentials found in environment variables");
            None
        } else {
            let config = $crate::config::ScreepsConfig::new(
                Some(token),
                Some(email),
                Some(password),
                "screeps.com".to_string(),
                true,
                10000,
            );
            Some($crate::api::ScreepsApi::new(config))
        }
    }};
}
