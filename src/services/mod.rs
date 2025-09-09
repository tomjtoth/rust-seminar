use dioxus::prelude::*;

#[server]
pub async fn get_0i8_from_server() -> Result<i8, ServerFnError> {
    Ok(0)
}
