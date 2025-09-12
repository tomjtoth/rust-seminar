use dioxus::prelude::*;
use dioxus_oauth::component::OAuthPopup;

use crate::components::{
    callback::component::CallbackComponent, container::Container,
    context_provider::ContextProvider, controlled_input::ControlledInput, counter::Counter,
    fullstack::FullstackExamples, global_signal::FnGlobalSignal, navbar::NavBar,
    oauth::OAuthLoginPage,
};

#[derive(Clone, Routable)]
pub(crate) enum Route {
    #[layout(NavBar)]
    #[layout(Container)]
    #[redirect("/", || Route::Counter {})]
    #[route("/counter")]
    Counter {},

    #[route("/controlled-input")]
    ControlledInput {},

    #[route("/context-provider")]
    ContextProvider {},

    #[route("/global-signal")]
    FnGlobalSignal {},

    #[route("/callback")]
    CallbackComponent {},

    #[route("/fullstack")]
    FullstackExamples {},

    #[route("/oauth/code")]
    OAuthPopup {},

    #[route("/login")]
    OAuthLoginPage {},
}
