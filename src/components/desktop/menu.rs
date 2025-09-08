use dioxus::{
    desktop::{
        muda::{accelerator::Accelerator, Menu, MenuItem, Submenu},
        use_muda_event_handler,
    },
    events::{Code, Modifiers},
};

#[derive(Debug, Clone, Copy)]
enum Menus {
    File,
    FileNew,
    FileOpen,
    FileSave,
    FileQuit,
    Help,
    HelpAbout,
}

impl ToString for Menus {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl PartialEq<Menus> for &str {
    fn eq(&self, other: &Menus) -> bool {
        *self == other.to_string()
    }
}

pub fn window_menu() -> Menu {
    let file_menu = Submenu::with_id_and_items(
        Menus::File,
        "File",
        true,
        &[
            &MenuItem::with_id(
                Menus::FileNew,
                "New",
                true,
                Some(Accelerator::new(Some(Modifiers::CONTROL), Code::KeyN)),
            ),
            &MenuItem::with_id(
                Menus::FileOpen,
                "Open",
                true,
                Some(Accelerator::new(Some(Modifiers::CONTROL), Code::KeyO)),
            ),
            &MenuItem::with_id(
                Menus::FileSave,
                "Save",
                true,
                Some(Accelerator::new(Some(Modifiers::CONTROL), Code::KeyS)),
            ),
            &MenuItem::with_id(
                Menus::FileQuit,
                "Quit",
                true,
                Some(Accelerator::new(Some(Modifiers::CONTROL), Code::KeyQ)),
            ),
        ],
    )
    .unwrap();

    let help_menu = Submenu::with_id_and_items(
        Menus::Help,
        "Help",
        true,
        &[&MenuItem::with_id(Menus::HelpAbout, "About", true, None)],
    )
    .unwrap();

    Menu::with_items(&[&file_menu, &help_menu]).unwrap()
}

pub(super) fn use_handler() {
    use_muda_event_handler(|evt| {
        let as_str = evt.id.0.as_str();

        if as_str == Menus::HelpAbout {
            super::about::open_about_view()
        } else if as_str == Menus::FileQuit {
            std::process::exit(0);
        }
    });
}
