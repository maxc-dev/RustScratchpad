use std::sync::{Arc, Mutex};
use std::thread;

/*
    https://www.youtube.com/watch?v=mupwF9jbVZ4
    Mutex = Mutual Exclusion
    Means only one thread can access the data at a time
    Mutex uses a locking system, when another thread tries to access the data,
     it signals that it wants that data and acquires the Mutex's lock.

    The lock is a DS that keeps track of who has access to the data.
    When the thread is done with the data, it releases the lock.
 */

pub fn main() {
    mutex_thread()
}

fn mutex_intro() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap(); // lock() is a blocking call
        // MutexGuard is a smart pointer, its deref trait points to the inner data of the mutex
        // Also has the drop trait which releases the lock when it goes out of scope
        *num = 6;
    }

    println!("m = {:?}", m);
}

/*
    Rc (Reference Counted) is a single-threaded reference-counting pointer used for
     shared ownership of data. It is not thread-safe.
    Arc (Atomic Reference Counted) is a thread-safe version of Rc that uses atomic
     operations to manage the reference count, making it suitable for concurrent programming.
 */

fn mutex_thread() {
    // Need to use Arc to share the Mutex between threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            /*
                counter is defined as immutable, but we can still modify the data inside it
                Because Mutex uses interior mutability

                Mutex allows you to modify the value inside an Arc smart pointer
             */

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}