use eframe::egui::{TextEdit, Ui};

use crate::MyEguiApp;

pub fn controlled_input(ui: &mut Ui, state: &mut MyEguiApp) {
    ui.horizontal(|ui| {
        let input_id = ui.make_persistent_id("my_text_input");

        if ui
            .label("Clicking me will *NOT* focus the textbox")
            .clicked()
        {
            ui.memory_mut(|mem| mem.request_focus(input_id));
        }

        ui.add(TextEdit::singleline(&mut state.text).id(input_id));
        if ui.button(r#"say "CHEESE""#).clicked() {
            state.text = String::from("CHEESE");
        }
    });
}
