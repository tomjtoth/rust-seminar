use dioxus::prelude::*;

use crate::components::{
    context_provider::{child::Child, parent::Parent},
    controlled_input::ControlledInput,
    counter::Counter,
};

mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div {
            class: "flex flex-col p-2 gap-2 items-center",

            Counter {}

            ControlledInput {}

            hr {}

            // DEMO: context_providers
            Parent { bg: "bg-red-200",
                Child { bg: "bg-purple-200" }
                Child { bg: "bg-green-200" }
            }

            hr {}

            Parent { bg: "bg-gray-200",
                Child { bg: "bg-blue-200" }
                Child { bg: "bg-yellow-200" }
            }
        }

    }
}
