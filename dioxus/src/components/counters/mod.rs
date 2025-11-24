use dioxus::prelude::*;

pub mod counter;

#[component]
pub fn CountersView() -> Element {
    rsx! {
        p { "3 counters with their own states" }

        counter::Counter {}
        counter::Counter {}
        counter::Counter {}
    }
}
