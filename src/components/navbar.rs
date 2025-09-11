use dioxus::prelude::*;

use crate::routes::Route;

#[component]
pub fn NavBar() -> Element {
    // TODO: find a way to restrict this to Web only, this results in weird behavior

    // #[cfg(target_arch = "wasm32")]
    // let oauth = rsx! { li { Link { to: Route::OAuthLoginPage {}, "OAuth" } } };

    // #[cfg(not(target_arch = "wasm32"))]
    // let oauth = rsx! {};

    rsx! {
        nav {
            ul {
                class: "p-2 flex gap-2 *:border *:rounded **:p-2",
                li { Link {to: Route::Counter {}, "counter"} }
                li { Link {to: Route::ControlledInput {}, "controlled input"} }
                li { Link {to: Route::ContextProvider {}, "context providers"} }
                li { Link {to: Route::FnGlobalSignal {}, "global signals"} }
                li { Link {to: Route::CallbackComponent {}, "callback"} }
                li { Link {to: Route::QueryServer {}, "fullstack"} }
                // {oauth}
            }
        }

        Outlet::<Route> {}
    }
}
