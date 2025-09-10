use dioxus::prelude::*;

use crate::{
    components::global_signal::{buttons::Incrementer, model::COUNTER},
    services::{get_0i8_from_server, get_fancy_struct_from_server, FancyStruct},
};

#[component]
pub fn QueryServer() -> Element {
    let fancy_handler = |_| async {
        if let Ok(fs) = get_fancy_struct_from_server().await {
            let FancyStruct { inner: (_idc, arr) } = fs;

            COUNTER.with_mut(|mutable| *mutable = *arr.last().unwrap());
        }
    };

    rsx! {
        p {
            "This manipulates the same global signal as seen previously.."
        }

        Incrementer { increment_by: 10 }
        Incrementer { increment_by: 5 }
        Incrementer { increment_by: 1 }

        button {
            onclick: |_| async {
                if let Ok(res) = get_0i8_from_server().await {
                    COUNTER.with_mut(|mutable| *mutable = res);
                }
            },

            "null counter (via plain server fn)"
        }

        button {
            onclick: fancy_handler,
            "null counter (via fancy server fn)"
        }
    }
}
