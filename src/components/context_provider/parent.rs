use dioxus::{logger::tracing, prelude::*};

#[derive(Clone, PartialEq)]
pub(super) struct ParentContext {
    pub(super) bg: Signal<String>,
}

#[derive(Clone, PartialEq, Props)]
pub struct ParentProps {
    children: Element,
    bg: &'static str,
}

#[component]
pub fn Parent(props: ParentProps) -> Element {
    let mut bg = use_signal(|| props.bg.to_string());
    use_context_provider(|| ParentContext { bg });

    use_effect(move || tracing::debug!("current bg: {bg}"));

    rsx! {
        div {
            class: "flex flex-column p-2 gap-2",

            button {
                onclick: move |_| bg.with_mut(|w| *w = props.bg.to_string()),
                "set bg of this to {props.bg}"
            }

            div {
                class: "border rounded p-2 {bg}",
                {props.children}
            }
        }
    }
}
