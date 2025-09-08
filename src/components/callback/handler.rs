use dioxus::prelude::*;

pub(super) async fn handler(callback: Callback<u8>) {
    let mut idx = use_signal(|| 0);

    let arguments = idx.with_mut(|w| {
        *w = if *w as u16 + 1 > u8::MAX as u16 {
            0
        } else {
            *w + 1
        };

        *w
    });

    callback.call(arguments)
}
