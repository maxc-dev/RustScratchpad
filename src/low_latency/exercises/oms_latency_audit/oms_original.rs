use std::collections::HashMap;
use std::time::Instant;

type OrderId = u64;

#[derive(Clone)]
pub struct Order {
    pub id: OrderId,
    pub symbol: String,
    pub qty: u64,
    pub price: u64,
}

pub enum Command {
    New(Order),
    Amend { id: OrderId, new_qty: u64 },
    Cancel { id: OrderId },
}

pub struct OMS {
    orders: HashMap<OrderId, Order>,
    audit_log: Vec<String>,
    total_processed: u64,
}

impl OMS {
    pub fn new() -> Self {
        Self {
            orders: HashMap::new(),
            audit_log: Vec::new(),
            total_processed: 0,
        }
    }

    #[inline(always)]
    pub fn handle(&mut self, cmd: Command) {
        match cmd {
            Command::New(order) => {
                self.audit_log.push(format!("NEW {}", order.id));
                self.orders.insert(order.id, order);
            }
            Command::Amend { id, new_qty } => {
                if let Some(order) = self.orders.get_mut(&id) {
                    self.audit_log.push(format!("AMEND {}", id));
                    order.qty = new_qty;
                }
            }
            Command::Cancel { id } => {
                if let Some(order) = self.orders.remove(&id) {
                    self.audit_log.push(format!("CANCEL {}", order.id));
                }
            }
        }

        self.total_processed += 1;
    }
}

const N: usize = 5_000_000;

fn main() {
    let mut oms = OMS::new();

    let start = Instant::now();

    for i in 0..N {
        oms.handle(Command::New(Order {
            id: i as u64,
            symbol: "BTCUSD".to_string(),
            qty: 10,
            price: 50_000,
        }));

        oms.handle(Command::Amend {
            id: i as u64,
            new_qty: 20,
        });

        oms.handle(Command::Cancel { id: i as u64 });
    }

    let elapsed = start.elapsed();
    println!("Elapsed: {:?}", elapsed);
    println!(
        "Throughput: {:.2} M ops/sec",
        N as f64 / elapsed.as_secs_f64() / 1_000_000.0
    );
}