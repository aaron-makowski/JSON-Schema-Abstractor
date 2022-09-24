#![allow(unused_variables)]
use envconf::{Setting, Error};


#[derive(Setting)]
pub struct OpenFaaSettings {
    #[conf(env = "GATEWAY_URL", default = "localhost")]
    pub host: String,
    #[conf(env = "GATEWAY_PORT", default = 8080)]
    pub port: usize,
    #[conf(env = "IS_TLS", default = false)]
    pub is_tls: bool,
}


impl OpenFaaSettings {
    fn get_protocol(&self) -> String{
        match self.is_tls {
            true => String::from("https"),
            false => String::from("http"),
        }
    }

    pub fn get_url(&self) -> String{
        // https://<gateway URL>:<port>/async-function/<function name>
        let protocol = self.get_protocol();
        format!("{}://{}:{}", protocol, self.host, self.port)
    }


    /// Returns an async function URL
    pub fn get_function(&self, name: &str) -> String{
        format!("{}/function/{}", self.get_url(), name)
    }
    pub fn get_async_function(&self, name: &str) -> String{
        format!("{}/async-function/{}", self.get_url(), name)
    }
}
