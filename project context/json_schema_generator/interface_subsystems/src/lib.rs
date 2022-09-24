pub mod errors;
pub mod interfaces;
pub mod mocks;

#[macro_use]
extern crate lazy_static;
extern crate pretty_env_logger;

use slog::Logger;
use sloggers::terminal::TerminalLoggerBuilder;
use sloggers::types::Severity;
use sloggers::Build;

fn create_logger() -> Logger {
    let mut builder = TerminalLoggerBuilder::new();
    builder.level(Severity::Debug);
    let logger = builder.build().unwrap();
    logger
}

lazy_static! {
    pub static ref GLOBAL_LOGGER: Logger = create_logger();
}

// let mut bggs = TerminalLoggerBuilder::new();
// .build().unwrap();

#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
