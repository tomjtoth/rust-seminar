use dioxus::prelude::*;

use crate::{
    components::global_signal::model::COUNTER,
    services::{roundtrip, Value::I8},
};

#[component]
pub fn FsIncrementer(increment_by: i8, title: Option<String>) -> Element {
    let operator = if increment_by > 0 { "+" } else { "-" };

    let disabled = COUNTER.with(|r| r.checked_add(increment_by).is_none());

    rsx! {
        button {
            disabled,
            title,
            class: if disabled { "cursor-not-allowed! text-gray-200" },
            onclick: move |_| async move {
                if let Ok(I8(val)) = roundtrip(I8(increment_by),None).await {
                    *COUNTER.write() += val
                }
            },
            "{COUNTER} {operator} {increment_by.abs()}"
        }
    }
}

#[component]
pub(super) fn FsNullifier() -> Element {
    rsx! {
        button {
            onclick: |_| async {
                if let Ok(I8(res)) = roundtrip(I8(0), None).await {
                    COUNTER.with_mut(|inner| *inner = res);
                }
            },
            "null it"
        }
    }
}
