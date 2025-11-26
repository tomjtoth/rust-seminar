mod external_logic;

use eframe::egui::Ui;

use crate::{MyEguiApp, components::callback::external_logic::use_external_logic};

pub fn callbacks(ui: &mut Ui, state: &mut MyEguiApp) {
    ui.label(format!(r#"currently: "{}""#, state.callback.text));

    let mut temp_helper = None;

    if ui.button("trigger callback").clicked() {
        let mut external_logic = use_external_logic(state);

        let callback = |idx| {
            // DEMO: uncomment
            // state.callback.text = format!("done-{}", idx);
            temp_helper = Some(format!("done-{}", idx))
        };

        external_logic(callback);
    }

    if let Some(str) = temp_helper {
        state.callback.text = str;
    }
}
