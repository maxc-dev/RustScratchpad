use crossbeam::channel::Sender;
use std::thread;
use std::time::Duration;

pub struct Publisher {
    id: usize,
    sender: Sender<String>,
}

impl Publisher {
    pub fn new(id: usize, sender: Sender<String>) -> Self {
        Publisher { id, sender }
    }

    pub fn start(&self) {
        let sender = self.sender.clone();
        let id = self.id;
        thread::spawn(move || {
            for i in 0..10 {
                let message = format!("Publisher #{}, message: {}", id, i);
                sender.send(message).expect("Failed to send message");
                thread::sleep(Duration::from_millis(100));
            }
        });
    }
}
