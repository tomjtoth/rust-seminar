use dioxus::prelude::*;

mod components;
mod routes;
mod utils;

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
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
        components::desktop::use_menu_handlers();
    }

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: utils::TAILWIND_CSS }

        Router::<routes::Route> {}
    }
}
