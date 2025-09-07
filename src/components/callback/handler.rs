use dioxus::{hooks::use_signal, prelude::Callback, signals::Writable};

pub(super) async fn use_handler(callback: Callback<u8>) {
    let mut idx = use_signal(|| 0);

    callback.call(idx.with_mut(|w| {
        *w += 1;
        *w
    }))
}
