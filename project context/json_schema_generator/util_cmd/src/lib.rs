#[macro_use]
extern crate lazy_static;

pub mod net;

use std::sync::Arc;
use util_settings::ServiceEnvVars;

lazy_static! {
    pub static ref SETTINGS: Arc<ServiceEnvVars> = Arc::new(ServiceEnvVars::new());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
