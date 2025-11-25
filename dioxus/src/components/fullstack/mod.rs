use dioxus::prelude::*;

mod buttons;
mod futures;

#[component]
pub fn FullstackExamples() -> Element {
    let class = "border rounded p-2 flex flex-col gap-2 [&_button]:w-fit items-center";

    rsx! {
        div {
            class,

            // DEMO: passed as Some(String)
            buttons::FsIncrementer { increment_by: 16, title: Some("manipulates the previously seen global signal".to_string()) }

            // passed as String
            buttons::FsIncrementer { increment_by: 8, title: "same as above".to_string() }

            // CANNOT pass Some(&str)
            // buttons::FsIncrementer { increment_by: 4, title: Some("same as above") }

            // passed as &str
            buttons::FsIncrementer { increment_by: 4, title: "same as above" }

            // passed None
            buttons::FsIncrementer { increment_by: 2, title: None }

            // omitted
            buttons::FsIncrementer { increment_by: 1 }

            buttons::FsNullifier {}
        }

        div {
            class,

            futures::Future {}
        }
    }
}
