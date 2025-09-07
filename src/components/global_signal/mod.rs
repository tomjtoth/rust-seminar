use dioxus::prelude::*;

use crate::components::global_signal::buttons::{Incrementer, Nullifier};

mod buttons;
mod model;

#[component]
pub fn FnGlobalSignal() -> Element {
    rsx! {
        Incrementer { increment_by: 1 }
        Incrementer { increment_by: 5 }
        Incrementer { increment_by: -10 as i8 }
        Nullifier {}
    }
}
