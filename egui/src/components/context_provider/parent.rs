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
            "red" => Color32::LIGHT_RED,
            "purple" => Color32::from_rgb(220, 180, 255),
            "green" => Color32::LIGHT_GREEN,
            "blue" => Color32::LIGHT_BLUE,
            "yellow" => Color32::LIGHT_YELLOW,

            // "orange"
            _ => Color32::from_rgb(255, 200, 150),
        };

        ui.style_mut().visuals.widgets.hovered.weak_bg_fill = color;
        ui.style_mut().visuals.widgets.inactive.weak_bg_fill = color;

        ui.horizontal(|ui| {
            if ui.button(format!("set buttons to {}", bg)).clicked() {
                state.context_provider_values[idx] = bg;
            }

            ui.vertical(|ui| {
                children(ui, state);
            });
        });
    });
}
