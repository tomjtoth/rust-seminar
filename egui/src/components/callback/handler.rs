use crate::{MyEguiApp, state::APP_STATE};

pub(super) fn handler(state: &mut MyEguiApp, callback: &mut impl FnMut(u8)) {
    let incremented = state.callback.counter.checked_add(1).unwrap_or(0);

    // DEMO: flip this
    if true {
        state.callback.counter = incremented;
    } else {
        APP_STATE.set_callback_counter(incremented)
    }

    callback(incremented);
}
