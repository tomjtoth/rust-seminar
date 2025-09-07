use dioxus::prelude::*;

use crate::components::global_signal::model::*;

#[derive(Clone, PartialEq, Props)]
pub(super) struct CounterProps {
    increment_by: i8,
}

#[component]
pub fn Incrementer(props: CounterProps) -> Element {
    let operator = if props.increment_by > 0 { "+" } else { "-" };

    let disabled = {
        if COUNTER.with(move |r| {
            let res = *r as i16 + props.increment_by as i16;
            res < i8::MIN as i16 || res > i8::MAX as i16
        }) {
            true
        } else {
            false
        }
    };

    rsx! {
        button {
            disabled,
            onclick: move |_| *COUNTER.write() += props.increment_by,
            "{COUNTER} {operator} {props.increment_by.abs()}"
        }
    }
}

#[component]
pub fn Nullifier() -> Element {
    rsx! {
        button {
            onclick: |_| COUNTER.null_it(),
            "null it"
        }
    }
}
