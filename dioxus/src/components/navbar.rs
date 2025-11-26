use dioxus::prelude::*;

use crate::routes::Route;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav {
            ul {
                class: "p-2 flex flex-wrap gap-2 *:border *:rounded **:p-2 *:text-nowrap",

                li { Link {to: Route::CountersView {}, "counters"} }
                li { Link {to: Route::ControlledInput {}, "controlled input"} }
                li { Link {to: Route::ContextProvider {}, "context providers"} }
                li { Link {to: Route::GlobalState {}, "global state"} }
                li { Link {to: Route::Callbacks {}, "callbacks"} }
                li { Link {to: Route::Fullstack {}, "fullstack"} }
            }
        }

        Outlet::<Route> {}
    }
}
