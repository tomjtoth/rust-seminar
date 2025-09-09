use dioxus::prelude::*;

type GsCounter = GlobalSignal<i8>;

pub static COUNTER: GsCounter = Signal::global(|| 0);

pub trait TrCounter {
    fn null_it(&self);
}

impl TrCounter for GsCounter {
    fn null_it(&self) {
        self.with_mut(|w| *w = 0);
    }
}
