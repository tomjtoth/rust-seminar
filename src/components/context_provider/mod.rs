use dioxus::prelude::*;

pub mod child;
pub mod parent;

use child::Child;
use parent::Parent;

#[component]
pub fn ContextProvider() -> Element {
    rsx! {
        Parent { bg: "bg-red-200",
            Child { bg: "bg-purple-200" }
            Child { bg: "bg-green-200" }
        }

        Parent { bg: "bg-gray-200",
            Child { bg: "bg-blue-200" }
            Child { bg: "bg-yellow-200" }
        }
    }
}
