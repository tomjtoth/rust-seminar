use eframe::egui::Ui;

use crate::MyEguiApp;

#[derive(PartialEq)]
pub enum View {
    Counter,
    ControlledInput,
    ContextProvider,
    GlobalSignals,
    Callback,
    Fullstack,
}

pub fn navbar(ui: &mut Ui, state: &mut MyEguiApp) {
    ui.horizontal(|ui| {
        if ui.button("counter").clicked() {
            if state.view != View::Counter {
                state.counter = 0;
            }
            state.view = View::Counter;
        }

        if ui.button("controlled input").clicked() {
            state.view = View::ControlledInput;
        }

        if ui.button("context provider").clicked() {
            state.view = View::ContextProvider;
        }

        if ui.button("global signals").clicked() {
            state.view = View::GlobalSignals;
        }

        if ui.button("callback").clicked() {
            state.view = View::Callback;
        }

        if ui.button("fullstack").clicked() {
            state.view = View::Fullstack;
        }
    });
}
