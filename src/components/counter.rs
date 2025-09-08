use dioxus::prelude::*;

#[component]
pub fn Counter() -> Element {
    let mut counter = use_signal(|| 0u8);

    rsx! {
        "current count: "
        button {
            class: "w-fit",
            title: "increment by one",
            onclick: move |_| {
                counter.with_mut(|w| {
                    let inc_or_zero = if let Some(x) = w.checked_add(1) { x } else { 0 };
                    *w = inc_or_zero;
                })
            },
            "{counter}++"
        }
    }
}
