use dioxus::prelude::*;

use crate::components::global_signal::buttons::Incrementer;

mod futures;
mod nullifiers;

#[component]
pub fn FullstackExamples() -> Element {
    let class = "border rounded p-2 flex flex-col gap-2 [&_button]:w-fit items-center";

    rsx! {
        div {
            class,

            p {
                "This manipulates the same global signal as seen previously.."
            }

            Incrementer { increment_by: 10 }
            Incrementer { increment_by: 5 }
            Incrementer { increment_by: 1 }

            nullifiers::SimpleNullifier {}
            nullifiers::FancyNullifier {}
        }

        div {
            class,

            futures::Future {}
        }
    }
}
