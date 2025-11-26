mod handler;

use dioxus::prelude::*;

use crate::components::callback::handler::use_external_logic;

#[component]
pub fn Callbacks() -> Element {
    let mut text = use_signal(|| "initial".to_string());

    let mut external_logic = use_external_logic();

    let local_logic =
        move |safely_incremented_index| text.set(format!("done-{safely_incremented_index}"));

    rsx! {
        div {
            r#"currently: "{text}""#
        }

        button {
            onclick: move |_| external_logic(local_logic),

            "trigger callback"
        }
    }
}
