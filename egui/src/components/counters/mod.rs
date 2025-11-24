use eframe::egui::Ui;

use crate::MyEguiApp;

mod counter;

use counter::counter;

pub fn counters(ui: &mut Ui, state: &mut MyEguiApp) {
    ui.label("3 counters with their own states:");

    counter(ui, state, 0);
    counter(ui, state, 1);
    counter(ui, state, 2);
}
