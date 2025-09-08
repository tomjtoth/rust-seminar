use dioxus::prelude::*;

mod components;
mod routes;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    #[cfg(feature = "desktop")]
    {
        use dioxus::desktop::{Config, WindowBuilder};

        let root_menu = components::desktop_menu::desktop_menu();

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
        components::desktop_menu::use_handler();
    }

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<routes::Route> {}
    }
}
