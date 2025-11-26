use native_db::{Key, ToKey, native_db};
use native_model::{Model, native_model};
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use ulid::serde::ulid_as_u128;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub struct EventID(#[serde(with = "ulid_as_u128")] Ulid);

impl ToKey for EventID {
    fn to_key(&self) -> Key {
        self.0.0.to_key()
    }
    fn key_names() -> Vec<String> {
        vec!["EventID".to_string()]
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[native_model(id = 1, version = 1)]
#[native_db]
pub struct Event {
    #[primary_key]
    id: EventID,
    #[secondary_key]
    key: String,
    kind: EventKind,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum EventKind {
    Enqueue,
    Dequeue,
    Start,
    Stop,
    Link(String),
    Comment(String),
}

impl Event {
    pub fn new(key: String, kind: EventKind) -> Event {
        Event {
            id: EventID(Ulid::new()),
            key,
            kind,
        }
    }
}
