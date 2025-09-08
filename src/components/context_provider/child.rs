use dioxus::prelude::*;

use crate::components::context_provider::parent::ParentContext;

#[derive(Clone, PartialEq, Props)]
pub struct ChildProps {
    pub bg: &'static str,
}

#[component]
pub fn Child(props: ChildProps) -> Element {
    let mut bg = use_context::<ParentContext>().bg;

    rsx! {
        p {
            "Lorem ipsum dolor sit amet, consectetur adipisci elit, sed eiusmod tempor incidunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquid ex ea commodi consequat. Quis aute iure reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint obcaecat cupiditat non provident, sunt in culpa qui official deserunt mollit anim id est laborum."
        }

        button {
            class: props.bg,
            onclick: move |_| bg.set(props.bg.to_string()),
            "set bg of this parent to {props.bg}"
        }
    }
}
