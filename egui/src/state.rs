use std::cell::LazyCell;

use crate::MyEguiApp;

pub const APP_STATE: LazyCell<MyEguiApp> = LazyCell::new(|| MyEguiApp::default());
