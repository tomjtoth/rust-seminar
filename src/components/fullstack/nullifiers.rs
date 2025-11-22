use dioxus::prelude::*;

use crate::{
    components::global_signal::model::COUNTER,
    services::{get_0i8_from_server, get_fancy_struct_from_server, FancyStruct},
};

#[component]
pub(super) fn SimpleNullifier() -> Element {
    rsx! {
        button {
            onclick: |_| async {
                if let Ok(res) = get_0i8_from_server().await {
                    COUNTER.with_mut(|inner| *inner = res);
                }
            },
            "null counter (via plain server fn)"
        }
    }
}

#[component]
pub(super) fn FancyNullifier() -> Element {
    let fancy_handler = |_| async {
        if let Ok(fs) = get_fancy_struct_from_server().await {
            let FancyStruct { inner: (_, arr) } = fs;

            COUNTER.with_mut(|inner| *inner = *arr.last().unwrap());
        }
    };

    rsx! {
        button {
            onclick: fancy_handler,
            "null counter (via fancy server fn)"
        }
    }
}
