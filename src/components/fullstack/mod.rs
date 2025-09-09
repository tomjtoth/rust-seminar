use dioxus::prelude::*;

use crate::{
    components::global_signal::{buttons::Incrementer, model::COUNTER},
    services::get_0i8_from_server,
};

#[component]
pub fn QueryServer() -> Element {
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

            "null counter (async via server route)"
        }
    }
}
