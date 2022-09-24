use crate::errors::ServiceIOError;
use crate::interfaces::ServerlessCommand;
use crate::GLOBAL_LOGGER;
use async_trait::async_trait;
use serde_json::Value;
use slog;

pub struct SchemaManager {
    #[allow(dead_code)]
    name: String,
}
#[async_trait]
impl ServerlessCommand for SchemaManager {
    fn new() -> Self {
        SchemaManager {
            name: String::from("mock_placeholder"),
        }
    }

    async fn call_serverless(&self, data: Box<Value>) -> Result<Value, ServiceIOError> {
        slog::info!(GLOBAL_LOGGER, "Calling serverless server!");
        Ok(*data)
    }
}

#[cfg(test)]
mod tests {

    use super::SchemaManager;
    use crate::interfaces::ServerlessCommand;
    #[tokio::test]
    async fn test_calling_serverless() {
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
        let manager = SchemaManager::new();
        let json_data = Box::new(serde_json::from_str(data)).unwrap();
        let man_res = manager.call_serverless(json_data).await;
        assert!(man_res.is_ok());
    }
}
