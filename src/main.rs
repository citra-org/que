mod event;
mod queue;
mod producer;
mod consumer;

use std::sync::Arc;
use event::data::data::EventData;
use queue::event::EventQueue;
use producer::event::EventProducer;
use consumer::event::EventConsumer;

fn main() {
    let queue = Arc::new(EventQueue::new());

    let producers = (0..5).map(|i| {
        let queue = Arc::clone(&queue);
        std::thread::spawn(move || {
            let producer = EventProducer::new(queue);
            producer.produce_event(EventData {
                id: i,
                payload: format!("Event {} payload", i),
            });
        })
    });

    let consumer = {
        let queue = Arc::clone(&queue);
        std::thread::spawn(move || {
            let consumer = EventConsumer::new(queue);
            consumer.consume_events();
        })
    };

    for producer in producers {
        producer.join().unwrap();
    }

    consumer.join().unwrap();
}

