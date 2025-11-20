use dioxus::prelude::*;

#[cfg(feature = "desktop")]
pub mod desktop;

pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

pub fn init_client_side() {
    dioxus_fullstack::set_server_url(server_url());
}

pub fn server_url() -> &'static str {
    option_env!("SERVER_URL").unwrap_or("http://127.0.0.1:8080")
}

pub fn std_sleep(millis: u32) {
    std::thread::sleep(std::time::Duration::from_millis(1000));
}
