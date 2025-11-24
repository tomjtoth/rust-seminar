use eframe::egui::Ui;

use crate::{
    MyEguiApp,
    components::{controlled_input::controlled_input, counters::counters, navbar::View::*},
};

pub fn router(ui: &mut Ui, state: &mut MyEguiApp) {
    match state.view {
        Counters => counters(ui, state),
        ControlledInput => controlled_input(ui, state),
        _ => (),
    }
}
