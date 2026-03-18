use std::mem::{needs_drop, MaybeUninit};

#[derive(Copy, Clone)]
pub struct Handle(u32);

pub struct Arena<T> {
    buffer: Vec<MaybeUninit<T>>,
    free_list: Vec<Handle>,
    occupied: Vec<bool>,
}

impl<T> Drop for Arena<T> {
    fn drop(&mut self) {
        if needs_drop::<T>() {
            for (i, occupied) in self.occupied.iter().enumerate() {
                if *occupied {
                    unsafe {
                        self.buffer[i].assume_init_drop();
                    }
                }
            }
        }
    }
}

impl<T> Arena<T> {
    pub fn with_capacity(cap: usize) -> Self {
        Arena {
            buffer: (0..cap).map(|_| MaybeUninit::uninit()).collect(),
            free_list: (0..cap).map(|i| Handle(i as u32)).collect(),
            occupied: vec![false; cap],
        }
    }

    pub fn alloc(&mut self, value: T) -> Option<Handle> {
        if let Some(handle) = self.free_list.pop() {
            let index = handle.0 as usize;
            self.buffer[index] = MaybeUninit::new(value);
            self.occupied[index] = true;
            return Some(handle);
        }
        None
    }

    pub fn get(&self, handle: Handle) -> &T {
        unsafe {
            self.buffer[handle.0 as usize].assume_init_ref()
        }
    }

    pub fn get_mut(&mut self, handle: Handle) -> &mut T {
        unsafe {
            self.buffer[handle.0 as usize].assume_init_mut()
        }
    }

    pub fn free(&mut self, handle: Handle) -> T {
        let index = handle.0 as usize;
        assert!(self.occupied[index], "double drop detected");

        self.occupied[index] = false;
        self.free_list.push(handle);

        unsafe {
            let ptr = self.buffer.as_ptr().add(index) as *const T;
            read(ptr)
        }
    }
}

use std::hint::black_box;
use std::ptr::read;
use std::time::Instant;

const N: usize = 10_000_000;
const CAP: usize = 1_000_000;

#[derive(Debug)]
struct Order {
    id: u64,
    price: u64,
    qty: u64,
}

fn main() {
    let mut arena = Arena::<Order>::with_capacity(CAP);

    // Pre-allocate some handles for reuse testing
    let mut handles = Vec::with_capacity(CAP);

    // Warmup
    for i in 0..CAP {
        let h = arena.alloc(Order {
            id: i as u64,
            price: 100,
            qty: 10,
        }).unwrap();
        handles.push(h);
    }

    for h in handles.drain(..) {
        arena.free(h);
    }

    let start = Instant::now();

    for i in 0..N {
        let h = arena.alloc(Order {
            id: i as u64,
            price: 100,
            qty: 10,
        }).unwrap();

        let o = arena.get_mut(h);
        o.price += 1;

        arena.free(h);
    }

    let elapsed = start.elapsed();

    println!("Elapsed: {:?}", elapsed);
    println!(
        "Throughput: {:.2} M alloc/free per sec",
        N as f64 / elapsed.as_secs_f64() / 1_000_000.0
    );

    black_box(arena);
}