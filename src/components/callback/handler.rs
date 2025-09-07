use dioxus::prelude::*;

pub(super) async fn use_handler(callback: Callback<u8>) {
    let mut idx = use_signal(|| 0);

    callback.call(idx.with_mut(|w| {
        *w = if *w as u16 + 1 > u8::MAX as u16 {
            0
        } else {
            *w + 1
        };

        *w
    }))
}
