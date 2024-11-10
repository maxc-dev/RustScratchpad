mod publisher;
mod consumer;


use crossbeam::channel;
use crate::publisher::Publisher;
use crate::consumer::Consumer;

pub struct Controller {
    num_producers: usize,
    num_consumers: usize,
}

impl Controller {
    pub fn new(num_producers: usize, num_consumers: usize) -> Self {
        Controller { num_producers, num_consumers }
    }

    pub fn start(&self) {
        let (sender, receiver) = channel::unbounded();

        // Create producers
        for i in 0..self.num_producers {
            let publisher = Publisher::new(i, sender.clone());
            publisher.start();
        }

        // Create consumers
        for i in 0..self.num_consumers {
            let consumer = Consumer::new(i, receiver.clone());
            consumer.start();
        }
    }
}

fn main() {
    let controller = Controller::new(3, 2); // 3 producers & 2 consumers
    controller.start();

    std::thread::sleep(std::time::Duration::from_secs(5)); // Allow time for messages to be processed
}