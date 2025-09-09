use dioxus::desktop::trayicon::{default_tray_icon, init_tray_icon};

mod about;
mod menu;

pub use menu::window_menu;

use crate::utils::desktop::tray_icon;

pub fn init_desktop_runtime() {
    init_tray_icon(default_tray_icon(), Some(tray_icon()));

    menu::use_handler();
}
