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

            // DEMO: passed as Some(String)
            Incrementer { increment_by: 16, title: Some("manipulates the previously seen global signal".to_string()) }

            // passed as String
            Incrementer { increment_by: 8, title: "same as above".to_string() }

            // CANNOT pass Some(&str)
            // Incrementer { increment_by: 4, title: Some("same as above") }

            // passed as &str
            Incrementer { increment_by: 4, title: "same as above" }

            // passed None
            Incrementer { increment_by: 2, title: None }

            // omitted
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
