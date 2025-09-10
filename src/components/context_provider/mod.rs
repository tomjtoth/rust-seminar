use dioxus::prelude::*;

pub mod child;
pub mod parent;

use child::Child;
use parent::Parent;

#[component]
pub fn ContextProvider() -> Element {
    rsx! {
        Parent { bg: "bg-red-200 dark:bg-red-800",
            Child { bg: "bg-purple-200 dark:bg-purple-800" }
            Child { bg: "bg-green-200 dark:bg-green-800" }
        }

        Parent { bg: "bg-orange-200 dark:bg-orange-800",
            Child { bg: "bg-blue-200 dark:bg-blue-800" }
            Child { bg: "bg-yellow-200 dark:bg-yellow-800" }
        }
    }
}
