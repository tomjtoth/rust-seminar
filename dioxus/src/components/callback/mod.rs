mod handler;

use dioxus::prelude::*;

use crate::components::callback::handler::use_external_logic;

#[component]
pub fn Callbacks() -> Element {
    let mut text = use_signal(|| "initial".to_string());

    let mut handler = use_external_logic();

    rsx! {
        div {
            r#"currently: "{text}""#
        }

        button {
            onclick: move |_| handler(move |safely_incremented_index| {
                text.set(format!("done-{safely_incremented_index}"))
            }),

            "trigger callback"
        }
    }
}
