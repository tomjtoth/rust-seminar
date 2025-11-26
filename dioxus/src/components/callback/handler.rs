use dioxus::prelude::*;

pub(super) fn use_external_logic_dx() -> impl FnMut(Callback<u8>) {
    let mut index = use_signal(|| 0u8);

    move |callback| {
        let arguments = index.with_mut(|inner| {
            let safely_incremented = inner.checked_add(1).unwrap_or(0);

            *inner = safely_incremented;
            safely_incremented
        });

        callback.call(arguments);
    }
}

pub(super) fn use_external_logic<T>() -> impl FnMut(T)
where
    T: FnMut(u8),
{
    let mut index = use_signal(|| 0u8);

    move |mut callback: T| {
        let arguments = index.with_mut(|inner| {
            let safely_incremented = inner.checked_add(1).unwrap_or(0);

            *inner = safely_incremented;
            safely_incremented
        });

        callback(arguments);
    }
}
