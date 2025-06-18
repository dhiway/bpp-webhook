use crate::config::AppConfig;
use crate::models::webhook::Context;
use crate::services::webhook::{generate_response, send_to_bpp_caller};
use serde_json::Value;
use std::sync::Arc;
use tokio::task;

pub fn spawn_processing_task(
    context: Context,
    message: Value,
    action: String,
    config: Arc<AppConfig>,
) {
    task::spawn(async move {
        let response = generate_response(&action, context, message, &config);
        if let Err(e) = send_to_bpp_caller(&action, response, config).await {
            eprintln!("Error sending to BPP client: {:?}", e);
        }
    });
}
