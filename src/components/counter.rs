use dioxus::prelude::*;

#[component]
pub fn Counter() -> Element {
    let mut counter = use_signal(|| 0);

    rsx! {
        "current count: "
        button {
            class: "w-fit",
            title: "increment by one",
            onclick: move |_evt| {
                counter.with_mut(|w| *w += 1)
            },
            "{counter}++"
        }
    }
}
