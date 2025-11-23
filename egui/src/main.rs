use eframe::egui::{self, Vec2};

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            inner_size: Some(Vec2::new(640.0, 480.0)),
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
    counter: u8,
    view: View,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        MyEguiApp {
            counter: 0,
            view: View::Counter,
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
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");

            counter(ui, &mut self.counter);
        });
    }
}

pub fn counter(ui: &mut egui::Ui, counter: &mut u8) {
    ui.horizontal(|ui| {
        ui.label(format!("current count: {}", counter));

        if ui.button(format!("{}++", counter)).clicked() {
            *counter = counter.checked_add(1).unwrap_or(0);
        }
    });
}
