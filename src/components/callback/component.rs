use dioxus::prelude::*;

use crate::components::callback::handler::handler;

#[component]
pub fn CallbackComponent() -> Element {
    let mut sig = use_signal(|| "initial".to_string());

    let cb = use_callback(move |a| sig.set(format!("done-{a}")));

    rsx! {
        div {
            r#"currently: "{sig}""#
        }

        button {
            onclick: move |_| async move {
                handler(cb).await;
            },
            "trigger callback"
        }
    }
}
