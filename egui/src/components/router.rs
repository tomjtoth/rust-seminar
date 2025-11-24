use eframe::egui::Ui;

use crate::{
    MyEguiApp,
    components::{counters::counters, navbar::View::*},
};

pub fn router(ui: &mut Ui, state: &mut MyEguiApp) {
    match state.view {
        Counters => counters(ui, state),
        _ => (),
    }
}
