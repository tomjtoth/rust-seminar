use dioxus::prelude::*;

#[component]
pub fn ControlledInput() -> Element {
    let mut txt = use_signal(|| "".to_string());

    rsx! {
        form {
            class: "flex flex-row items-center gap-2",
            label {
                input {
                    placeholder: "text here",

                    oninput:move |evt| {
                        let mut writable = txt.write();
                        *writable = evt.value()
                    },

                    value: "{txt}"
                },
                "type here something"
            }

            button {
                onclick: move |evt| {
                    evt.prevent_default();
                    txt.with_mut(|writable| {*writable = "preset".to_string()})
                },

                r#"reset input to "preset""#
            }
        }
    }
}
