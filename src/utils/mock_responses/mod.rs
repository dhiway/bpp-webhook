use serde_json::Value;

pub fn load_mock_response(action: &str) -> Option<Value> {
    match action {
        "search" => Some(serde_json::from_str(include_str!("response.search.json")).unwrap()),
        // ... add other actions
        _ => None,
    }
}
