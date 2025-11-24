use eframe::egui::{self, Vec2};

use crate::components::{
    navbar::{View, navbar},
    router::router,
};

mod components;
mod state;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            inner_size: Some(Vec2::new(640.0, 480.0)),
            min_inner_size: Some(Vec2::new(320.0, 240.0)),
            ..Default::default()
        },
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Rust seminar - egui",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    );
}

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

struct MyEguiApp {
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

    fn new(cc: &eframe::CreationContext<'_>) -> Self {
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
