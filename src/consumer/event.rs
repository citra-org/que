use std::sync::Arc;
use crate::queue::event::EventQueue;

pub struct EventConsumer {
    queue: Arc<EventQueue>,
}

impl EventConsumer {
    pub fn new(queue: Arc<EventQueue>) -> Self {
        EventConsumer { queue }
    }

    pub fn consume_events(&self) {

    }
}