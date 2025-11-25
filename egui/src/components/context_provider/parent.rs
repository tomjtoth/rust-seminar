use eframe::egui::{Color32, Ui};

use crate::MyEguiApp;

pub fn parent(
    ui: &mut Ui,
    state: &mut MyEguiApp,
    bg: &'static str,
    idx: usize,
    children: impl Fn(&mut Ui, &mut MyEguiApp),
) {
    ui.scope(|ui| {
        let color = match state.context_provider_values[idx] {
            "red" => Color32::DARK_RED,
            "purple" => Color32::PURPLE,
            "green" => Color32::DARK_GREEN,
            "blue" => Color32::DARK_BLUE,
            "yellow" => Color32::YELLOW,
            _ => Color32::ORANGE,
        };

        ui.style_mut().visuals.widgets.hovered.weak_bg_fill = color;
        ui.style_mut().visuals.widgets.inactive.weak_bg_fill = color;

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(format!(
                    "current value is: {}",
                    state.context_provider_values[idx]
                ));

                if ui.button(format!("set bg to {}", bg)).clicked() {
                    state.context_provider_values[idx] = bg;
                }
            });

            ui.vertical(|ui| {
                children(ui, state);
            });
        });
    });
}
