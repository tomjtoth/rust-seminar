use dioxus::prelude::*;

use crate::components::global_signal::model::*;

#[derive(Clone, PartialEq, Props)]
pub(super) struct CounterProps {
    increment_by: i8,
}

#[component]
pub fn Incrementer(props: CounterProps) -> Element {
    let operator = if props.increment_by > 0 { "+" } else { "-" };

    let disabled = COUNTER.with(move |r| r.checked_add(props.increment_by).is_none());
    rsx! {
        button {
            disabled,
            class: if disabled { "text-gray-200" },
            style: if disabled { "cursor: not-allowed;" },
            onclick: move |_| *COUNTER.write() += props.increment_by,
            "{COUNTER} {operator} {props.increment_by.abs()}"
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
