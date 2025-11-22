use dioxus::prelude::*;

#[component]
pub fn Container() -> Element {
    rsx! {
        div {
            class: "flex flex-col p-2 gap-2 items-center",
            Outlet::<crate::routes::Route> {}
        }
    }
}
