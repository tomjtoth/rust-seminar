use dioxus::prelude::*;

use crate::components::global_signal::model::*;

#[component]
pub fn Incrementer(increment_by: i8, title: Option<String>) -> Element {
    let operator = if increment_by > 0 { "+" } else { "-" };

    let disabled = COUNTER.with(|r| r.checked_add(increment_by).is_none());

    rsx! {
        button {
            disabled,
            title,
            class: if disabled { "cursor-not-allowed! text-gray-200" },
            onclick: move |_| *COUNTER.write() += increment_by,
            "{COUNTER} {operator} {increment_by.abs()}"
        }
    }
}

#[component]
pub fn Nullifier() -> Element {
    rsx! {
        button {
            class: "my-5",
            onclick: |_| COUNTER.null_it(),
            "null it"
        }
    }
}
