use std::sync::LazyLock;

use native_db::{Models, ToKey, native_db};
use native_model::{Model, native_model};
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use ulid::serde::ulid_as_u128;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[native_model(id = 1, version = 1)]
#[native_db(
    primary_key(pk -> u128),
    secondary_key(sk -> String)
)]
pub enum Event {
    Enqueue {
        #[serde(with = "ulid_as_u128")]
        id: Ulid,
        key: String,
    },
    Dequeue {
        #[serde(with = "ulid_as_u128")]
        id: Ulid,
        key: String,
    },
}

impl Event {
    fn pk(&self) -> u128 {
        match self {
            Event::Enqueue { id, .. } => (*id).into(),
            Event::Dequeue { id, .. } => (*id).into(),
        }
    }
    fn sk(&self) -> Option<String> {
        match self {
            Event::Enqueue { key, .. } => Some(key.into()),
            Event::Dequeue { key, .. } => Some(key.into()),
        }
    }
}

pub static MODELS: LazyLock<Models> = LazyLock::new(|| {
    let mut models = Models::new();
    models.define::<Event>().unwrap();
    models
});
