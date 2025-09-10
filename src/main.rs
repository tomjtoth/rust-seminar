use dioxus::prelude::*;

mod components;
mod routes;
mod services;
mod utils;

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    #[cfg(not(feature = "server"))]
    {
        let server_url = option_env!("SERVER_URL").unwrap_or("http://127.0.0.1:8080");
        server_fn::client::set_server_url(server_url);
    }

    #[cfg(feature = "desktop")]
    {
        use dioxus::desktop::{Config, WindowBuilder};

        let root_menu = components::desktop::window_menu();

        dioxus::LaunchBuilder::desktop()
            .with_cfg(
                Config::new()
                    .with_window(
                        WindowBuilder::new()
                            .with_title("Rust Seminar Demo")
                            .with_minimizable(true)
                            .with_maximizable(true),
                    )
                    .with_menu(root_menu),
            )
            .launch(App);
    }

    #[cfg(not(feature = "desktop"))]
    {
        dioxus::launch(App);
    }
}

#[component]
fn App() -> Element {
    #[cfg(feature = "desktop")]
    {
        components::desktop::init_desktop_runtime();
    }

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: utils::TAILWIND_CSS }

        Router::<routes::Route> {}
    }
}
