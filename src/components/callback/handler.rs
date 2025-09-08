use dioxus::prelude::*;

pub(super) async fn handler(callback: Callback<u8>) {
    let mut idx = use_signal(|| 0u8);

    let arguments = idx.with_mut(|w| {
        let inc_or_zero = if let Some(x) = w.checked_add(1) { x } else { 0 };

        *w = inc_or_zero;
        inc_or_zero
    });

    callback.call(arguments)
}
