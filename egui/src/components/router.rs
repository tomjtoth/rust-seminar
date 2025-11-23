use eframe::egui::Ui;

use crate::{
    MyEguiApp,
    components::{counter::counter, navbar::View::*},
};

pub fn router(ui: &mut Ui, state: &mut MyEguiApp) {
    match state.view {
        Counter => counter(ui, state),
        _ => (),
    }
}
