mod menu;

pub use menu::window_menu;

pub fn use_menu_handlers() {
    menu::use_handler();
}
