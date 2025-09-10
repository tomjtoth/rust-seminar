use dioxus::prelude::*;

#[cfg(feature = "desktop")]
pub mod desktop;

pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

pub fn init_client_side() {
    server_fn::client::set_server_url(option_env!("SERVER_URL").unwrap_or("http://127.0.0.1:8080"));
}
