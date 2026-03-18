use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;
use std::hint::black_box;

#[derive(Clone)]
pub struct Order {
    pub id: u64,
    pub price: u64,
    pub qty: u64,
    pub side: Side,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Side {
    Bid,
    Ask,
}

pub struct Engine {
    processed: AtomicU64,
    risk_rejects: AtomicU64,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            processed: AtomicU64::new(0),
            risk_rejects: AtomicU64::new(0),
        }
    }

    #[inline(always)]
    pub fn handle(&self, order: Order) -> Option<Order> {
        // Risk check
        if order.qty > 1_000_000 {
            self.risk_rejects.fetch_add(1, Ordering::Relaxed);
            return None;
        }

        // Side logic
        let adjusted_price = if order.side == Side::Bid {
            order.price + 1
        } else {
            order.price - 1
        };

        self.processed.fetch_add(1, Ordering::Relaxed);

        Some(Order {
            id: order.id,
            price: adjusted_price,
            qty: order.qty,
            side: order.side,
        })
    }
}


const N: usize = 20_000_000;

fn generate_orders() -> Vec<Order> {
    let mut orders = Vec::with_capacity(N);

    for i in 0..N {
        orders.push(Order {
            id: i as u64,
            price: 1000 + (i as u64 % 10),
            qty: 100, // below risk threshold (hot path)
            side: if i % 2 == 0 { Side::Bid } else { Side::Ask },
        });
    }

    orders
}

fn run_benchmark(engine: &Engine, orders: &[Order]) {
    let start = Instant::now();

    let mut acc = 0u64;

    for o in orders {
        if let Some(out) = engine.handle(o.clone()) {
            acc = acc.wrapping_add(out.price);
        }
    }

    let elapsed = start.elapsed();

    black_box(acc);

    println!("Elapsed: {:?}", elapsed);
    println!(
        "Throughput: {:.2} M ops/sec",
        N as f64 / elapsed.as_secs_f64() / 1_000_000.0
    );
}

fn main() {
    let orders = generate_orders();
    let engine = Engine::new();

    // Warmup
    run_benchmark(&engine, &orders);

    println!("--- Measured run ---");
    run_benchmark(&engine, &orders);
}