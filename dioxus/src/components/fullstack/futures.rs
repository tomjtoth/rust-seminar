use dioxus::prelude::*;

use crate::services::{roundtrip, Value::Str};

#[component]
pub fn Future() -> Element {
    let mut value = use_signal(|| String::new());
    let mut fut = use_future(move || async move {
        if let Ok(Str(str)) = roundtrip(
            Str(String::from("value from initial roundtrip")),
            Some(1000),
        )
        .await
        {
            value.set(str);
        }
    });

    rsx! {
        label {
            "use_future for async ops: "

            input {
                value,
                oninput: move |evt| async move {
                    if let Ok(Str(val)) = roundtrip(Str(evt.value()),None).await {
                        value.set(val)
                    }
                },
                placeholder: "type here"
            }
        }

        button {
            class: "inline-block",
            // DEMO: rm "async move"
            onclick: move |_| async move {
                fut.restart();
            },
            "reset slowly"
        }
    }
}
