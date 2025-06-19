use crate::config::AppConfig;
use crate::models::webhook::Context;
use crate::utils::http_client::post_json;
use crate::utils::mock_responses::load_mock_response;
use anyhow::Result;
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;
use tracing::info;

pub fn generate_response(
    action: &str,
    context: Context,
    message: Value,
    config: &AppConfig,
) -> Value {
    if !config.use_mock_bpp_response {
        // TODO: Add logic for getting dat from bpp db/reference  here
        json!({
            "context": context,
            "message": message
        })
    } else {
        info!("Fallback to mock response: ...");
        // Fallback to mock response
        let mut mock = load_mock_response(action).unwrap_or_else(|| {
            json!({
                "context": context,
                "message": {
                    "note": "Default mock response",
                    "action": action
                }
            })
        });

        // Overwrite `transaction_id` and `message_id` in mock["context"]
        if let Some(ctx) = mock.get_mut("context") {
            ctx["transaction_id"] = json!(context.transaction_id.clone());
            ctx["message_id"] = json!(context.message_id.clone());
        }

        mock
    }
}

pub async fn send_to_bpp_caller(
    action: &str,
    payload: Value,
    config: Arc<AppConfig>,
) -> Result<()> {
    let bpp_url = &config.bpp.caller_uri;
    let full_url = format!("{}/on_{}", bpp_url, action);
    post_json(&full_url, payload).await
}
