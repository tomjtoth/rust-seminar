use eframe::egui;

use crate::components::{
    navbar::{View, navbar},
    router::router,
};
struct CallbackState {
    counter: u8,
    text: String,
}

impl Default for CallbackState {
    fn default() -> Self {
        CallbackState {
            counter: 0,
            text: String::from("initial"),
        }
    }
}

pub(super) struct MyEguiApp {
    counters: [u8; 3],
    global_counter: i8,
    view: View,
    text: String,
    callback: CallbackState,
    context_provider_values: [&'static str; 2],
}

impl Default for MyEguiApp {
    fn default() -> Self {
        MyEguiApp {
            counters: [0; 3],
            global_counter: 0,
            view: View::Counters,
            text: String::new(),
            callback: Default::default(),
            context_provider_values: ["orange", "red"],
        }
    }
}

impl MyEguiApp {
    fn set_callback_counter(&mut self, val: u8) {
        self.callback.counter = val;
    }

    pub(super) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        let mut style = (*cc.egui_ctx.style()).clone();

        style.visuals.panel_fill = egui::Color32::LIGHT_GRAY;

        cc.egui_ctx.set_style(style);
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                navbar(ui, self);
                router(ui, self);
            });
        });
    }
}
