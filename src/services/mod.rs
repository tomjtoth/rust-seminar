use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[server]
pub async fn get_0i8_from_server() -> Result<i8, ServerFnError> {
    Ok(0)
}

#[derive(Serialize, Deserialize)]
pub struct FancyStruct {
    pub inner: (u8, Vec<i8>),
}

#[server]
pub async fn get_fancy_struct_from_server() -> Result<FancyStruct, ServerFnError> {
    Ok(FancyStruct {
        inner: (123, vec![2, -3, 5, 0]),
    })
}

#[server]
pub async fn get_string_from_server(delay: Option<u32>) -> Result<String, ServerFnError> {
    if let Some(millis) = delay {
        crate::utils::std_sleep(millis);
    }

    Ok("string from server".to_string())
}
