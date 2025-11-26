use eframe::egui::Ui;

use crate::app::MyEguiApp;

use global_counter::global_counter;

mod global_counter;

pub fn global_state(ui: &mut Ui, state: &mut MyEguiApp) {
    global_counter(ui, state, 20);
    global_counter(ui, state, 10);
    global_counter(ui, state, 5);
    global_counter(ui, state, 1);

    ui.add_space(10.);
    global_counter(ui, state, 0);
    ui.add_space(10.);

    global_counter(ui, state, -1);
    global_counter(ui, state, -5);
    global_counter(ui, state, -10);
    global_counter(ui, state, -20);
}
