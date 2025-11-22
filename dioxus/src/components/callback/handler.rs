use dioxus::prelude::*;

pub(super) async fn handler(callback: Callback<u8>) {
    let mut index = use_signal(|| 0u8);

    let arguments = index.with_mut(|inner| {
        let safely_incremented = inner.checked_add(1).unwrap_or(0);

        *inner = safely_incremented;
        safely_incremented
    });

    callback.call(arguments)
}
