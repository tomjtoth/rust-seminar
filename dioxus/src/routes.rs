use dioxus::prelude::*;

use crate::components::{
    callback::Callbacks, container::Container, context_provider::ContextProvider,
    controlled_input::ControlledInput, counters::CountersView, fullstack::Fullstack,
    global_state::GlobalState, navbar::NavBar,
};

#[derive(Clone, Routable)]
pub enum Route {
    #[layout(NavBar)]
    #[layout(Container)]
    #[redirect("/", || Route::CountersView {})]
    #[route("/counter")]
    CountersView {},

    #[route("/controlled-input")]
    ControlledInput {},

    #[route("/context-provider")]
    ContextProvider {},

    #[route("/global-signal")]
    GlobalState {},

    #[route("/callback")]
    Callbacks {},

    #[route("/fullstack")]
    Fullstack {},
}
