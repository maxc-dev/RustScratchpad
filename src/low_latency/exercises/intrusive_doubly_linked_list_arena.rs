#[path = "fixed_capacity_arena_allocator_single_threaded.rs"]
mod fixed_capacity_arena_allocator_single_threaded;
use fixed_capacity_arena_allocator_single_threaded::{Arena, Handle};
use std::hint::black_box;
use std::time::Instant;

pub struct IntrusiveList<T> {
    arena: Arena<Node<T>>,
    head: Option<Handle>,
    tail: Option<Handle>,
}

struct Node<T> {
    value: T,
    prev: Option<Handle>,
    next: Option<Handle>,
}

impl<T> IntrusiveList<T> {
    pub fn with_capacity(cap: usize) -> Self {
        IntrusiveList {
            arena: Arena::with_capacity(cap),
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, value: T) -> Handle {
        if self.tail.is_none() && self.head.is_none() {
            let node = Node {
                value,
                prev: None,
                next: None,
            };
            let ptr = self.arena.alloc(node).unwrap();
            self.head = Some(ptr);
            self.tail = Some(ptr);
            return ptr;
        }
        let node = Node {
            value,
            prev: self.tail,
            next: None,
        };
        let ptr = self.arena.alloc(node).unwrap();
        self.arena.get_mut(self.tail.unwrap()).next = Some(ptr);
        self.tail = Some(ptr);
        ptr
    }

    pub fn remove(&mut self, handle: Handle) {
        let node = self.arena.get(handle);
        let lprev = node.prev;
        let lnext = node.next;
        if lprev.is_some() {
            self.arena.get_mut(lprev.unwrap()).next = lnext;
        } else {
            self.head = lnext;
        }
        if lnext.is_some() {
            self.arena.get_mut(lnext.unwrap()).prev = lprev;
        } else {
            self.tail = lprev;
        }
        self.arena.free(handle);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let h = self.head?;
        let next = self.arena.get(h).next;

        match next {
            None => {
                self.head = None;
                self.tail = None;
            }
            Some(n) => {
                self.head = Some(n);
                self.arena.get_mut(n).prev = None;
            }
        }

        Some(self.arena.free(h).value)
    }

    pub fn get(&self, handle: Handle) -> &T {
        &self.arena.get(handle).value
    }

    pub fn get_mut(&mut self, handle: Handle) -> &mut T {
        &mut self.arena.get_mut(handle).value
    }
}

const N: usize = 10_000_000;
const CAP: usize = 1_000_000;

#[derive(Debug)]
struct Order {
    id: u64,
    qty: u64,
}

fn main() {
    let mut list = IntrusiveList::<Order>::with_capacity(CAP);

    // Warmup
    for i in 0..CAP {
        list.push_back(Order {
            id: i as u64,
            qty: 10,
        });
    }

    // Clear it
    while list.pop_front().is_some() {}

    // Measured run
    let start = Instant::now();

    for i in 0..N {
        let h = list.push_back(Order {
            id: i as u64,
            qty: 10,
        });

        // Simulate cancel
        list.remove(h);
    }

    let elapsed = start.elapsed();

    println!("Elapsed: {:?}", elapsed);
    println!(
        "Throughput: {:.2} M ops/sec",
        N as f64 / elapsed.as_secs_f64() / 1_000_000.0
    );

    black_box(list);
}
