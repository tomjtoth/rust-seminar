use dioxus::{
    desktop::muda::{accelerator::Accelerator, Menu, MenuItem, Submenu},
    events::{Code, Modifiers},
};

pub fn desktop_menu() -> dioxus::desktop::muda::Menu {
    let root_menu = Menu::new();

    let file_menu = Submenu::new("File", true);

    file_menu
        .append(&MenuItem::new(
            "New",
            true,
            Some(Accelerator::new(Some(Modifiers::CONTROL), Code::KeyN)),
        ))
        .unwrap();

    file_menu
        .append(&MenuItem::new(
            "Open",
            true,
            Some(Accelerator::new(Some(Modifiers::CONTROL), Code::KeyO)),
        ))
        .unwrap();

    file_menu
        .append(&MenuItem::new(
            "Save",
            true,
            Some(Accelerator::new(Some(Modifiers::CONTROL), Code::KeyS)),
        ))
        .unwrap();

    file_menu
        .append(&MenuItem::new(
            "Quit",
            true,
            Some(Accelerator::new(Some(Modifiers::CONTROL), Code::KeyQ)),
        ))
        .unwrap();

    let help_menu = Submenu::new("Help", true);

    help_menu
        .append(&MenuItem::new("About", true, None))
        .unwrap();

    root_menu.append(&file_menu).unwrap();
    root_menu.append(&help_menu).unwrap();

    root_menu
}
