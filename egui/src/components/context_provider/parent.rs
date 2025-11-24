use eframe::egui::Ui;

use crate::MyEguiApp;

pub fn parent(
    ui: &mut Ui,
    state: &mut MyEguiApp,
    bg: &'static str,
    idx: usize,
    children: impl Fn(&mut Ui, &mut MyEguiApp),
) {
    ui.horizontal(|ui| {
        ui.label(format!(
            "current value is: {}",
            state.context_provider_values[idx]
        ));

        if ui.button(format!("set bg to {}", bg)).clicked() {
            state.context_provider_values[idx] = bg;
        }

        ui.vertical(|ui| {
            children(ui, state);
        })
    });
}
