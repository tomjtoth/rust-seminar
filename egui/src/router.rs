use eframe::egui::Ui;

use crate::{
    MyEguiApp,
    components::{
        callback::component::callback_view, context_provider::context_providers,
        controlled_input::controlled_input, counters::counters, global_counters::global_counters,
        navbar::View::*,
    },
};

pub fn router(ui: &mut Ui, state: &mut MyEguiApp) {
    match state.view {
        Counters => counters(ui, state),
        ControlledInput => controlled_input(ui, state),
        ContextProviders => context_providers(ui, state),
        GlobalSignals => global_counters(ui, state),
        Callbacks => callback_view(ui, state),
    }
}
