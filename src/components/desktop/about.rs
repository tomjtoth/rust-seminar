use dioxus::{
    desktop::{use_window, Config, LogicalSize, WindowBuilder},
    prelude::*,
};

#[component]
fn About() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: crate::utils::TAILWIND_CSS }

        img {
            src: "https://preview.redd.it/parsejsonstatham-v0-gesojxd4t96f1.jpeg?width=640&crop=smart&auto=webp&s=2008eba0d6bb2eb28892c2edb2af8d9052e03a5e",
            alt: "famous movie star",
            draggable: false,
            title: "close window",
            class: "cursor-pointer",
            onclick: |_| dioxus::desktop::window().close(),
        }
    }
}

pub(super) fn open_about_view() {
    let svc = use_window();
    let dom = VirtualDom::new(About);

    svc.new_window(
        dom,
        Config::new().with_window(
            WindowBuilder::new()
                .with_decorations(false)
                .with_inner_size(LogicalSize::new(640, 512)),
        ),
    );
}
