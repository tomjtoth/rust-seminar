use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Value {
    I8(i8),
    Str(String),
}

#[server]
pub async fn roundtrip(value: Value, delay: Option<u32>) -> Result<Value, ServerFnError> {
    if let Some(millis) = delay {
        crate::utils::std_sleep(millis);
    }

    Ok(value)
}
