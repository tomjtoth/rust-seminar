use dioxus::prelude::*;

use crate::components::{
    callback::component::CallbackComponent, container::Container,
    context_provider::ContextProvider, controlled_input::ControlledInput, counter::Counter,
    fullstack::QueryServer, global_signal::FnGlobalSignal, navbar::NavBar,
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
    QueryServer {},
}
