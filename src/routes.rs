use dioxus::prelude::*;

use crate::components::{
    container::Container, context_provider::ContextProvider, controlled_input::ControlledInput,
    counter::Counter, navbar::NavBar,
};

#[derive(Clone, Routable)]
pub(crate) enum Route {
    #[layout(NavBar)]
    #[layout(Container)]
    #[route("/counter")]
    Counter {},

    #[route("/controlled-input")]
    ControlledInput {},

    #[route("/context-provider")]
    ContextProvider {},
}
