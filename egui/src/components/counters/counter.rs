use eframe::egui::{Align, Layout, Ui};

use crate::MyEguiApp;

pub fn counter(ui: &mut Ui, state: &mut MyEguiApp, idx: usize) {
    ui.with_layout(
        Layout::left_to_right(Align::Center).with_main_justify(true),
        |ui| {
            ui.label("current count: ");
            if ui.button(format!("{}++", state.counters[idx])).clicked() {
                state.counters[idx] = state.counters[idx].checked_add(1).unwrap_or(0);
            }
        },
    );
}
