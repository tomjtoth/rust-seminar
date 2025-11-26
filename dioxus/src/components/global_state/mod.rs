use dioxus::prelude::*;

use crate::components::global_state::buttons::{Incrementer, Nullifier};

pub mod buttons;
pub mod model;

#[component]
pub fn GlobalState() -> Element {
    rsx! {
        Incrementer { increment_by: 20 }
        Incrementer { increment_by: 10 }
        Incrementer { increment_by: 5 }
        Incrementer { increment_by: 1 }

        Nullifier {}

        Incrementer { increment_by: -1 }
        Incrementer { increment_by: -5 }
        Incrementer { increment_by: -10 }
        Incrementer { increment_by: -20 }
    }
}
