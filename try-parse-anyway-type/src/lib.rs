pub use serde_json::Value;
pub use std::collections::HashMap;

#[derive(Debug)]
pub struct TryParseAnywayError<T> {
    pub partial_retrieved: Option<T>,
    pub errors: HashMap<&'static str, TryParseAnywayErrorItem>,
}

#[derive(Debug)]
pub struct TryParseAnywayErrorItem {
    pub value: Value,
    pub error: String,
}
