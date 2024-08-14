use crate::event::event::Event;
use std::sync::Mutex;
pub struct EventQueue {
    events: Mutex<Vec<Event>>,
}

impl EventQueue {
    pub fn new() -> Self {
        EventQueue {
            events: Mutex::new(Vec::new()),
        }
    }

    pub fn push(&self, event: Event) {
        let mut events = self.events.lock().unwrap();
        events.push(event);
    }

    pub fn pop(&self) -> Option<Event> {
        let mut events = self.events.lock().unwrap();
        events.pop()
    }
}
