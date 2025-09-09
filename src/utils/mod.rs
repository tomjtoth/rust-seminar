use dioxus::prelude::*;

#[cfg(feature = "desktop")]
pub mod desktop;

pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
