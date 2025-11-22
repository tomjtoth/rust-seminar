use dioxus::prelude::*;

use crate::components::callback::handler::handler;

#[component]
pub fn CallbackComponent() -> Element {
    let mut text = use_signal(|| "initial".to_string());

    let callback = use_callback(move |safely_incremented_index| {
        text.set(format!("done-{safely_incremented_index}"))
    });

    rsx! {
        div {
            r#"currently: "{text}""#
        }

        button {
            onclick: move |_| async move {
                handler(callback).await;
            },
            "trigger callback"
        }
    }
}
