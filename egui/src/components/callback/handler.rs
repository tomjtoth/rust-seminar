use crate::MyEguiApp;

pub(super) fn handler(state: &mut MyEguiApp, callback: &mut impl FnMut(u8)) {
    let incremented = state.callback.counter.checked_add(1).unwrap_or(0);
    state.callback.counter = incremented;

    callback(incremented);
}
