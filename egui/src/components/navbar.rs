use eframe::egui::Ui;

use crate::MyEguiApp;

#[derive(PartialEq)]
pub enum View {
    Counters,
    ControlledInput,
    ContextProvider,
    GlobalSignals,
    Callback,
    Fullstack,
}

pub fn navbar(ui: &mut Ui, state: &mut MyEguiApp) {
    ui.horizontal(|ui| {
        if ui.button("counter").clicked() {
            // reset all if coming from a different view
            if state.view != View::Counters {
                state.counters[0] = 0;
                state.counters[1] = 0;
                state.counters[2] = 0;
            }
            state.view = View::Counters;
        }

        if ui.button("controlled input").clicked() {
            // reset if coming from a different view
            if state.view != View::ControlledInput {
                state.text.truncate(0);
            }
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
