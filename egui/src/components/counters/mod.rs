use eframe::egui::Ui;

use crate::MyEguiApp;

mod counter;
mod global_counter;

use counter::counter;
use global_counter::global_counter;

pub fn counters(ui: &mut Ui, state: &mut MyEguiApp) {
    ui.label("3 counters with their own states:");

    counter(ui, state, 0);
    counter(ui, state, 1);
    counter(ui, state, 2);
}

pub fn global_counters(ui: &mut Ui, state: &mut MyEguiApp) {
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
