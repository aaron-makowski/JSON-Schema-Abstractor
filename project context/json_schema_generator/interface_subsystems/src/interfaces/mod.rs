use crate::errors::ServiceIOError;
use async_trait::async_trait;
use serde_json::Value;

pub trait DatabaseIO {
    /// Creates a request-response IO mechanism.
    /// Use to do all of the basic commands. We wrap this command
    /// with the responses to the client via GRPC
    fn database_request(data: &str) -> Result<String, ServiceIOError>;
}

#[async_trait]
pub trait ServerlessCommand {
    /// Use this to send commands to the serverless functions.
    /// Basically the exact same thing as database IO, only
    fn new() -> Self;
    async fn call_serverless(&self, data: Box<Value>) -> Result<Value, ServiceIOError>;
}
