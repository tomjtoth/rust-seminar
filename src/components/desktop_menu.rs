use dioxus::{
    desktop::{
        muda::{accelerator::Accelerator, Menu, MenuItem, Submenu},
        use_muda_event_handler,
    },
    events::{Code, Modifiers},
};

pub fn desktop_menu() -> Menu {
    let file_new = MenuItem::new(
        "New",
        true,
        Some(Accelerator::new(Some(Modifiers::CONTROL), Code::KeyN)),
    );

    let file_open = MenuItem::new(
        "Open",
        true,
        Some(Accelerator::new(Some(Modifiers::CONTROL), Code::KeyO)),
    );

    let file_save = MenuItem::new(
        "Save",
        true,
        Some(Accelerator::new(Some(Modifiers::CONTROL), Code::KeyS)),
    );

    let file_quit = MenuItem::new(
        "Quit",
        true,
        Some(Accelerator::new(Some(Modifiers::CONTROL), Code::KeyQ)),
    );

    let file_menu = Submenu::with_items(
        "File",
        true,
        &[&file_new, &file_open, &file_save, &file_quit],
    )
    .unwrap();

    let help_about = MenuItem::new("About", true, None);

    let help_menu = Submenu::with_items("Help", true, &[&help_about]).unwrap();

    use_muda_event_handler(move |evt| {
        if evt.id() == file_quit.id() {
            std::process::exit(0);
        }
    });

    Menu::with_items(&[&file_menu, &help_menu]).unwrap()
}
