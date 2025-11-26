use std::sync::LazyLock;

use native_db::Models;

use crate::event::Event;

pub mod event;
pub mod state;

pub static MODELS: LazyLock<Models> = LazyLock::new(|| {
    let mut models = Models::new();
    models.define::<Event>().unwrap();
    models
});
