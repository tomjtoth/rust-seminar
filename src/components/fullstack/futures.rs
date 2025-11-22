use dioxus::prelude::*;

use crate::services::get_string_from_server;

#[component]
pub fn Future() -> Element {
    let mut value = use_signal(|| String::new());
    let mut fut = use_future(move || async move {
        if let Ok(str) = get_string_from_server(Some(1000), false).await {
            value.set(str);
        }
    });

    rsx! {
        label {
            "use_future: "

            input {
                value,
                oninput: move |evt| value.set(evt.value()),
                placeholder: "type here"
            }
        }

        button {
            class: "inline-block",
            // DEMO: rm "async move"
            onclick: move |_| async move {
                fut.restart();
            },
            "reset future"
        }
    }
}
