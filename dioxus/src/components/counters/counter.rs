use dioxus::prelude::*;

#[component]
pub fn Counter() -> Element {
    let mut counter = use_signal(|| 0u8);

    rsx! {
        div {
            "current count: "
            button {
                class: "w-fit inline-block",
                title: "increment by one",
                onclick: move |_| {
                    counter.with_mut(|inner| {
                        *inner = inner.checked_add(1).unwrap_or(0)
                    })
                },
                "{counter}++"
            }
        }
    }
}
