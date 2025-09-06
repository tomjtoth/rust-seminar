use dioxus::{logger::tracing, prelude::*};

#[component]
pub fn ControlledInput() -> Element {
    let mut txt = use_signal(|| "".to_string());

    rsx! {
        form {
            class: "flex flex-row gap-2",
            onsubmit: |_| tracing::debug!("you shall not PASS/be printed ever"),

            label {
                "click me to focus the input"

                input {
                    class: "ml-2",
                    placeholder: "text here",

                    oninput:move |evt| {
                        let mut writable = txt.write();
                        *writable = evt.value()
                    },

                    value: "{txt}"
                }
            }

            button {
                onclick: move |evt| {
                    // DEMO: comment this out
                    evt.prevent_default();
                    txt.with_mut(|writable| { *writable = "CHEESE".to_string() })
                },

                r#"say "CHEESE""#
            }
        }
    }
}
