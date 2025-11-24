use eframe::egui::Ui;

use crate::MyEguiApp;

pub fn child(ui: &mut Ui, state: &mut MyEguiApp, bg: &'static str, parent_idx: usize) {
    ui.vertical(|ui|{
        ui.label("Lorem ipsum dolor sit amet, consectetur adipisci elit, sed eiusmod tempor incidunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquid ex ea commodi consequat. Quis aute iure reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint obcaecat cupiditat non provident, sunt in culpa qui official deserunt mollit anim id est laborum.");
        
        if ui.button(format!("set bg of this parent to {}", bg)).clicked() {
            state.context_provider_values[parent_idx] = bg;
        }
    });
}
