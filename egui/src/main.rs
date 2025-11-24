use eframe::egui::{self, Vec2};

use crate::components::{
    navbar::{View, navbar},
    router::router,
};

mod components;

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

struct MyEguiApp {
    counters: [u8; 3],
    view: View,
    text: String,
    context_provider_values: [&'static str; 2],
}

impl Default for MyEguiApp {
    fn default() -> Self {
        MyEguiApp {
            counters: [0; 3],
            view: View::Counters,
            text: String::new(),
            context_provider_values: ["orange", "red"],
        }
    }
}

impl MyEguiApp {
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
            navbar(ui, self);
            router(ui, self);
        });
    }
}
