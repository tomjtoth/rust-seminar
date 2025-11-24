use eframe::egui::Ui;

use crate::{MyEguiApp, components::callback::handler::handler};

pub fn callback_view(ui: &mut Ui, state: &mut MyEguiApp) {
    ui.label(state.callback.text.clone());

    let mut res: Option<String> = None;

    {
        let mut callback = |idx| res = Some(format!("done-{}", idx));

        if ui.button("trigger callback").clicked() {
            handler(state, &mut callback);
        }
    }

    if let Some(str) = res {
        state.callback.text = str;
    }
}
