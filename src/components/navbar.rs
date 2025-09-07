use dioxus::prelude::*;

use crate::routes::Route;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav {
            ul {
                class: "p-2 flex gap-2 *:border *:rounded **:p-2",
                li { Link {to: Route::Counter {}, "counter"} }
                li { Link {to: Route::ControlledInput {}, "controlled input"} }
                li { Link {to: Route::ContextProvider {}, "context providers"} }
                li { Link {to: Route::FnGlobalSignal {}, "global signals"} }
            }
        }

        Outlet::<Route> {}
    }
}
