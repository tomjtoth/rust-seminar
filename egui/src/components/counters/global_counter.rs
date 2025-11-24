use eframe::egui::Ui;

use crate::MyEguiApp;

pub fn global_counter(ui: &mut Ui, state: &mut MyEguiApp, diff: i8) {
    if ui
        .button(if diff == 0 {
            "null it".to_string()
        } else {
            let op = if diff > 0 { "+" } else { "-" };
            format!("{} {} {}", state.global_counter, op, diff.abs())
        })
        .clicked()
    {
        state.global_counter = if diff == 0 {
            0
        } else {
            state.global_counter.checked_add(diff).unwrap_or(0)
        }
    }
}
