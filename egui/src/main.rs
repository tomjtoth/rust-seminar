use eframe::egui::{self, Vec2};

use crate::app::MyEguiApp;

mod app;
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
