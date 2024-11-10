use crossbeam::channel::Receiver;
use std::thread;

pub struct Consumer {
    id: usize,
    receiver: Receiver<String>,
}

impl Consumer {
    pub fn new(id: usize, receiver: Receiver<String>) -> Self {
        Consumer { id, receiver }
    }

    pub fn start(&self) {
        let receiver = self.receiver.clone();
        let id = self.id;
        thread::spawn(move || {
            while let Ok(message) = receiver.recv() {
                println!("Consumer {} received: {}", id, message);
            }
        });
    }
}
