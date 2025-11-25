use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Value {
    I8(i8),
    Str(String),
}

#[server]
pub async fn roundtrip(value: Value) -> Result<Value, ServerFnError> {
    Ok(value)
}
