use dioxus::prelude::*;

mod components;
mod routes;
mod services;
mod utils;

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    #[cfg(not(feature = "server"))]
    utils::init_client_side();

    if cfg!(feature = "desktop") {
        utils::desktop::launch(App);
    } else {
        dioxus::launch(App);
    }
}

#[component]
fn App() -> Element {
    #[cfg(feature = "desktop")]
    utils::desktop::init_desktop_runtime();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: utils::TAILWIND_CSS }

        Router::<routes::Route> {}
    }
}
