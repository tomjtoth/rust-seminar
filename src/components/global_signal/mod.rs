use dioxus::prelude::*;

use crate::components::global_signal::buttons::{Incrementer, Nullifier};

mod buttons;
mod model;

#[component]
pub fn FnGlobalSignal() -> Element {
    rsx! {
        Incrementer { increment_by: 20 }
        Incrementer { increment_by: 10 }
        Incrementer { increment_by: 5 }
        Incrementer { increment_by: 1 }

        Nullifier {}

        Incrementer { increment_by: -1 as i8 }
        Incrementer { increment_by: -5 as i8 }
        Incrementer { increment_by: -10 as i8 }
        Incrementer { increment_by: -20 as i8 }
    }
}
