use eframe::egui::Ui;

use crate::MyEguiApp;

pub fn global_counter(ui: &mut Ui, state: &mut MyEguiApp, increment_by: i8) {
    if ui
        .button(if increment_by == 0 {
            "null it".to_string()
        } else {
            let op = if increment_by > 0 { "+" } else { "-" };
            format!("{} {} {}", state.global_counter, op, increment_by.abs())
        })
        .clicked()
    {
        state.global_counter = if increment_by == 0 {
            0
        } else {
            state.global_counter.checked_add(increment_by).unwrap_or(0)
        }
    }
}
