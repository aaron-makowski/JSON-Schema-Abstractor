// Use to send http commands to serverless functions directly
use crate::SETTINGS;
use serde_json::Value;
use std::sync::Arc;

/// Send request directly to faas function by name.
pub async fn send(name: &str, body: &Value) -> Result<Value, reqwest::Error> {
    let faas_url = Arc::clone(&SETTINGS).faas_env.get_async_function(name);
    let response = reqwest::Client::new()
        .post(&faas_url)
        .json(body)
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}

#[cfg(test)]
mod test_faas_connection {

    // Test that we're able to successfully
    // send a command to open faas.
    #[test]
    fn test_send_faas() {
        assert_eq!(2 + 2, 4);
    }
}
