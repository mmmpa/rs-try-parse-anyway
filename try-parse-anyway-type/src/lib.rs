pub use serde_json::Value;
pub use std::collections::HashMap;

#[derive(Debug)]
pub enum TryParseAnywayError<T> {
    Total(String),
    Partial { retrieved: T, errors: HashMap < & 'static str, TryParseAnywayErrorItem > },
}

#[derive(Debug)]
pub struct TryParseAnywayErrorItem {
    pub value: Value,
    pub error: String,
}

pub trait TryIntoAnyway<T> {
    fn try_into_anyway(self) ->  Result<T, TryParseAnywayError<T>>;
}
