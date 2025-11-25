use eframe::egui;

use crate::components::{
    navbar::{View, navbar},
    router::router,
};
pub(crate) struct CallbackState {
    pub(crate) counter: u8,
    pub(crate) text: String,
}

impl Default for CallbackState {
    fn default() -> Self {
        CallbackState {
            counter: 0,
            text: String::from("initial"),
        }
    }
}

pub(crate) struct MyEguiApp {
    pub(crate) counters: [u8; 3],
    pub(crate) global_counter: i8,
    pub(crate) view: View,
    pub(crate) text: String,
    pub(crate) callback: CallbackState,
    pub(crate) context_provider_values: [&'static str; 2],
}

impl Default for MyEguiApp {
    fn default() -> Self {
        MyEguiApp {
            counters: [0; 3],
            global_counter: 0,
            view: View::Counters,
            text: String::new(),
            callback: Default::default(),
            context_provider_values: ["green", "red"],
        }
    }
}

impl MyEguiApp {
    pub(crate) fn set_callback_counter(&mut self, val: u8) {
        self.callback.counter = val;
    }

    pub(super) fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                #[cfg(target_os = "android")]
                ui.add_space(20.);

                navbar(ui, self);
                ui.add_space(10.);
                router(ui, self);
            });
        });
    }
}
