use crate::app::MyEguiApp;

mod app;
mod components;
mod state;

#[cfg(target_os = "android")]
use egui_winit::winit;
#[cfg(target_os = "android")]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use eframe::{NativeOptions, Renderer};

    unsafe {
        std::env::set_var("RUST_BACKTRACE", "full");
    }
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let options = NativeOptions {
        android_app: Some(app),
        renderer: Renderer::Wgpu,
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Rust seminar - egui on Android",
        options,
        Box::new(|_cc| Ok(Box::new(MyEguiApp::default()))),
    );
}
