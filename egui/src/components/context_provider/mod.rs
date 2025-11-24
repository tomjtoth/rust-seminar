use eframe::egui::Ui;

use crate::MyEguiApp;

mod child;
mod parent;

pub fn context_providers(ui: &mut Ui, state: &mut MyEguiApp) {
    parent::parent(ui, state, "orange", 0, |ui, state| {
        child::child(ui, state, "purple", 0);
        child::child(ui, state, "green", 0);
    });

    parent::parent(ui, state, "red", 0, |ui, state| {
        child::child(ui, state, "blue", 1);
        child::child(ui, state, "yellow", 1);
    });
}
