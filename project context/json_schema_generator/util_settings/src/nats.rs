#![allow(unused_variables)]
use envconf::{Setting, Error};


#[derive(Setting)]
pub struct NatsSettings {
    #[conf(env = "NATS_HOST", default = "localhost")]
    pub host: String,
    #[conf(env = "NATS_PORT", default = 5432)]
    pub port: usize,
    #[conf(env = "NATS_USER", default = "")]  // hardcoded setting
    pub user: String,
    #[conf(env = "NATS_PASSWORD", default = "")] // env variable required
    pub password: String,
    #[conf(env = "NATS_SUBJECT", default = "router")]
    pub name: String,
    #[conf(env = "IS_TLS", default = false)]
    pub is_tls: bool,
    #[conf(env = "CONN_NAME", default = "basic")]
    pub connection_name: String,
}


impl NatsSettings {
    pub fn get_url(&self) -> String{
        // nats://127.0.0.1:4222
        let without_username = format!("nats://{}:{}", self.host, self.port);
        
        if self.user.is_empty() == true ||self.password.is_empty() == true {
            return without_username;
        }

        format!("{}?user={}&password={}", &without_username, &self.user, &self.password)
    }
}
