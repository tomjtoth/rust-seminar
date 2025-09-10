use dioxus::{logger::tracing, prelude::*};

#[derive(Clone, PartialEq)]
pub(super) struct ParentContext {
    pub(super) sig_bg: Signal<String>,
}

#[component]
pub fn Parent(children: Element, bg: &'static str) -> Element {
    let mut sig_bg = use_signal(|| bg.to_string());
    use_context_provider(|| ParentContext { sig_bg });

    use_effect(move || tracing::debug!("current bg: {sig_bg}"));

    rsx! {
        div {
            class: "flex p-2 gap-2",

            button {
                class: bg,
                onclick: move |_| sig_bg.set(bg.to_string()),
                "set bg of this to {bg}"
            }

            div {
                class: "border rounded p-2 {sig_bg} duration-500",
                {children}
            }
        }
    }
}
