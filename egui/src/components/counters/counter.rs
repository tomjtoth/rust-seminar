use eframe::egui::Ui;

use crate::MyEguiApp;

pub fn counter(ui: &mut Ui, state: &mut MyEguiApp, idx: usize) {
    ui.label("current count: ");
    if ui.button(format!("{}++", state.counters[idx])).clicked() {
        state.counters[idx] = state.counters[idx].checked_add(1).unwrap_or(0);
    }
}
