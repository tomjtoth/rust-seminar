use eframe::egui::Ui;

use crate::MyEguiApp;

#[derive(PartialEq)]
pub enum View {
    Counters,
    ControlledInput,
    ContextProviders,
    GlobalState,
    Callbacks,
    // Fullstack,
}

pub fn navbar(ui: &mut Ui, state: &mut MyEguiApp) {
    ui.horizontal(|ui| {
        if ui.button("counters").clicked() {
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

        if ui.button("context providers").clicked() {
            if state.view != View::ContextProviders {
                state.context_provider_values = ["green", "red"]
            }
            state.view = View::ContextProviders;
        }

        if ui.button("global state").clicked() {
            // not resetting on view change
            state.view = View::GlobalState;
        }

        if ui.button("callbacks").clicked() {
            if state.view != View::Callbacks {
                state.callback = Default::default();
            }
            state.view = View::Callbacks;
        }

        // if ui.button("fullstack").clicked() {
        //     state.view = View::Fullstack;
        // }
    });
}
