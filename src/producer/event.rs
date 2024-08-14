use std::sync::Arc;
use crate::event::event::Event;
use crate::event::data::data::EventData;
use crate::queue::event::EventQueue;

pub struct EventProducer {
    queue: Arc<EventQueue>,
}

impl EventProducer {
    pub fn new(queue: Arc<EventQueue>) -> Self {
        EventProducer { queue }
    }

    pub fn produce_event(&self, data: EventData) {
        let event = Event { data };
        self.queue.push(event);
    }
}