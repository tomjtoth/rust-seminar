use dioxus::{logger::tracing, prelude::*};

#[derive(Clone, PartialEq)]
pub(super) struct ParentContext {
    pub(super) bg: Signal<String>,
}

// DEMO: `children: Element``

#[component]
pub fn Parent(children: Element, class: &'static str) -> Element {
    let mut sig_bg = use_signal(|| class.to_string());
    use_context_provider(|| ParentContext { bg: sig_bg });

    use_effect(move || tracing::debug!("current bg: {sig_bg}"));

    let color = class.split('-').nth(1).unwrap();

    rsx! {
        div {
            class: "flex p-2 gap-2",

            button {
                class,
                onclick: move |_| sig_bg.set(class.to_string()),
                "set bg of this to {color}"
            }

            div {
                class: "border rounded p-2 {sig_bg} duration-500",
                {children}
            }
        }
    }
}
