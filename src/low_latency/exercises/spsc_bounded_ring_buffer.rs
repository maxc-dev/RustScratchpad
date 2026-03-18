use std::sync::Arc;
use std::thread;
use std::time::Instant;
use std::hint::black_box;
use std::mem::MaybeUninit;
use std::ptr::{drop_in_place, read};
use std::sync::atomic::{AtomicU64, Ordering};

#[repr(align(64))]
struct Padded64 {
    value: AtomicU64,
}

pub struct SpscRing<T> {
    buffer: Box<[MaybeUninit<T>]>,
    tail: Padded64,
    head: Padded64,
    cap: u64,
}

impl<T> SpscRing<T> {
    pub fn with_capacity(cap: usize) -> SpscRing<T> {
        assert!(cap.is_power_of_two(), "capacity must be power of two");
        SpscRing {
            buffer: Box::new_uninit_slice(cap),
            tail: Padded64 { value: AtomicU64::new(0) },
            head: Padded64 { value: AtomicU64::new(0) },
            cap: cap as u64,
        }
    }

    pub fn push(&self, value: T) -> Result<(), T> {
        let tail = self.tail.value.load(Ordering::Relaxed);
        let head = self.head.value.load(Ordering::Acquire);
        if tail - head == self.cap {
            return Err(value);
        }
        unsafe {
            let ptr = self.buffer.as_ptr() as *mut MaybeUninit<T>;
            let index = (tail & (self.cap - 1)) as usize;
            ptr.add(index).write(MaybeUninit::new(value));
        }
        self.tail.value.store(tail + 1, Ordering::Release);
        Ok(())
    }

    pub fn pop(&self) -> Option<T> {
        let head = self.head.value.load(Ordering::Relaxed);
        let tail = self.tail.value.load(Ordering::Acquire);
        if head == tail {
            return None;
        }
        let index = (head & (self.cap - 1)) as usize;
        let value = unsafe { read(self.buffer.as_ptr().add(index)).assume_init() };
        self.head.value.store(head + 1, Ordering::Release);
        Some(value)
    }
}

impl<T> Drop for SpscRing<T> {
    fn drop(&mut self) {
        let mut head = self.head.value.load(Ordering::Relaxed);
        let tail = self.tail.value.load(Ordering::Relaxed);
        let ptr = self.buffer.as_mut_ptr();

        while head != tail {
            let index = (head & (self.cap - 1)) as usize;
            unsafe {
                drop_in_place(ptr.add(index));
            }
            head += 1;
        }
    }
}

const N: usize = 10_000_000;
const CAPACITY: usize = 1024; // must be power of two

fn main() {
    let ring = Arc::new(SpscRing::<u64>::with_capacity(CAPACITY));

    let producer = {
        let ring = Arc::clone(&ring);
        thread::spawn(move || {
            for i in 0..N as u64 {
                loop {
                    if ring.push(i).is_ok() {
                        break;
                    }
                    std::hint::spin_loop();
                }
            }
        })
    };

    let consumer = {
        let ring = Arc::clone(&ring);
        thread::spawn(move || {
            let mut expected = 0u64;
            let mut received = 0usize;

            while received < N {
                if let Some(v) = ring.pop() {
                    // correctness check
                    if v != expected {
                        panic!("Out of order: expected {}, got {}", expected, v);
                    }
                    expected += 1;
                    received += 1;
                } else {
                    std::hint::spin_loop();
                }
            }

            black_box(expected);
        })
    };

    let start = Instant::now();

    producer.join().unwrap();
    consumer.join().unwrap();

    let elapsed = start.elapsed();

    println!("Elapsed: {:?}", elapsed);
    println!(
        "Throughput: {:.2} M msgs/sec",
        N as f64 / elapsed.as_secs_f64() / 1_000_000.0
    );
}