use dioxus::prelude::*;

use crate::components::context_provider::parent::ParentContext;

#[component]
pub fn Child(bg: &'static str, children: Element) -> Element {
    let mut sig_bg = use_context::<ParentContext>().sig_bg;

    rsx! {
        p {
            "Lorem ipsum dolor sit amet, consectetur adipisci elit, sed eiusmod tempor incidunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquid ex ea commodi consequat. Quis aute iure reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint obcaecat cupiditat non provident, sunt in culpa qui official deserunt mollit anim id est laborum."
        }

        button {
            class: sig_bg,
            onclick: move |_| sig_bg.set(bg.to_string()),
            "set bg of this parent to {bg}"
        }
    }
}
