pub mod faas;
pub mod nats;
pub mod pgsql;
pub mod redis;

use envconf::Setting;
use faas::OpenFaaSettings;
use nats::NatsSettings;
use pgsql::PostgreSettings;
use redis::RedisSettings;

pub struct ServiceEnvVars {
    pub postgres: PostgreSettings,
    pub nats_env: NatsSettings,
    pub redis_env: RedisSettings,
    pub faas_env: OpenFaaSettings,
}

impl ServiceEnvVars {
    pub fn new() -> Self {
        ServiceEnvVars {
            postgres: PostgreSettings::init().unwrap(),
            nats_env: NatsSettings::init().unwrap(),
            redis_env: RedisSettings::init().unwrap(),
            faas_env: OpenFaaSettings::init().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
