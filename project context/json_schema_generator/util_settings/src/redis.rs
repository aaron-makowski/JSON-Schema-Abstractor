#![allow(unused_variables)]
use envconf::{Error, Setting};

#[derive(Setting)]
pub struct RedisSettings {
    #[conf(env = "RD_HOST", default = "localhost")]
    pub host: String,
    #[conf(env = "RD_PORT", default = 5432)]
    pub port: usize,
    #[conf(env = "RD_USER", default = "myuser")] // hardcoded setting
    pub user: String,
    #[conf(env = "RD_PASSWORD", default = "")] // env variable required
    pub password: String,
    #[conf(env = "RD_NAME", default = "kevin")]
    pub name: String,
}

impl RedisSettings {
    pub fn get_url(&self) -> String {
        let without_username = format!("redis://{}:{}/{}", self.host, self.port, self.name);

        if self.user.is_empty() || self.password.is_empty() {
            return without_username;
        }
        format!(
            "{}?user={}&password={}",
            &without_username, &self.user, &self.password
        )
    }
}
