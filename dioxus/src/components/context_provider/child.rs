use dioxus::prelude::*;

use crate::components::context_provider::parent::ParentContext;

#[component]
pub fn Child(class: &'static str) -> Element {
    let mut parent_bg = use_context::<ParentContext>().bg;
    let color = class.split('-').nth(1).unwrap();

    rsx! {
        p {
            "Lorem ipsum dolor sit amet, consectetur adipisci elit, sed eiusmod tempor incidunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquid ex ea commodi consequat. Quis aute iure reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint obcaecat cupiditat non provident, sunt in culpa qui official deserunt mollit anim id est laborum."
        }

        button {
            class,
            onclick: move |_| parent_bg.set(class.to_string()),
            "set bg of parent to {color}"
        }
    }
}
