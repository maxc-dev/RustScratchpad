use std::{thread, time::Duration};

// Great tutorial: https://www.youtube.com/watch?v=06WcsNPUNC8

pub fn main() {
    moving_into_thread()
}

pub fn keeping_thread_alive() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread spawn: {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 1..10 {
        println!("Thread main: {}", i);
        thread::sleep(Duration::from_millis(10));
    }

    handle.join().unwrap();
}

pub fn moving_into_thread() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Vector: {:?}", v)
    });

    handle.join().unwrap();
}