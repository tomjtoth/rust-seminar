use eframe::egui::Ui;

use crate::MyEguiApp;

pub fn counter(ui: &mut Ui, state: &mut MyEguiApp) {
    ui.horizontal(|ui| {
        if ui.button(format!("{}++", state.counter)).clicked() {
            state.counter = state.counter.checked_add(1).unwrap_or(0);
        }
    });
}
