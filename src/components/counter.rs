use dioxus::prelude::*;

#[component]
pub fn Counter() -> Element {
    let mut counter = use_signal(|| 0);

    rsx! {
        div {
            class: "cursor-pointer select-none p-5 m-5 text-center",
            onclick: move |_evt| {
                counter.with_mut(|w| *w += 1)
            },
            "current count: {counter}"
        }
    }
}
