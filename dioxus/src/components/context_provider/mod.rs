use dioxus::prelude::*;

mod child;
mod parent;

use child::Child;
use parent::Parent;

#[component]
pub fn ContextProvider() -> Element {
    rsx! {
        Parent { class: "bg-orange-200 dark:bg-orange-800",
            Child { class: "bg-purple-200 dark:bg-purple-800" }
            Child { class: "bg-green-200 dark:bg-green-800" }
        }

        Parent { class: "bg-red-200 dark:bg-red-800",
            Child { class: "bg-blue-200 dark:bg-blue-800" }
            Child { class: "bg-yellow-200 dark:bg-yellow-800" }
        }
    }
}
