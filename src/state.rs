use std::iter::once;

use crate::event::Event;

#[derive(Debug, Clone)]
pub struct State {
    active: Option<String>,
    queue: Vec<String>,
}

pub enum TransitionError {
    Unknown,
}

impl State {
    pub fn apply(&self, e: &Event) -> Result<State, TransitionError> {
        match e.kind {
            crate::event::EventKind::Enqueue => {
                if self.queue.contains(&e.key) {
                    Err(TransitionError::Unknown)?
                };
                Ok(State {
                    queue: self.queue.iter().chain(once(&e.key)).cloned().collect(),
                    ..self.clone()
                })
            }
            crate::event::EventKind::Dequeue => {
                if !self.queue.contains(&e.key) {
                    Err(TransitionError::Unknown)?
                };
                Ok(State {
                    queue: self
                        .queue
                        .iter()
                        .filter(|&x| *x != e.key)
                        .cloned()
                        .collect(),
                    ..self.clone()
                })
            }
            crate::event::EventKind::Start => {
                if self.queue.first() != Some(&e.key) {
                    Err(TransitionError::Unknown)?
                };
                Ok(State {
                    queue: self
                        .queue
                        .iter()
                        .filter(|&x| *x != e.key)
                        .cloned()
                        .collect(),
                    active: Some(e.key.clone()),
                    ..self.clone()
                })
            }
            crate::event::EventKind::Stop => {
                if self.active != Some(e.key.clone()) {
                    Err(TransitionError::Unknown)?
                };
                Ok(State {
                    queue: once(&e.key).chain(self.queue.iter()).cloned().collect(),
                    active: None,
                    ..self.clone()
                })
            }
            _ => Ok(self.clone()),
        }
    }
}
