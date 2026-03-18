use std::time::Instant;
use rustscratchpad::low_latency::model::arena::{Arena, Handle, Node};
use rustscratchpad::low_latency::model::intrusive_list::IntrusiveList;
use rustscratchpad::low_latency::model::order_slab::{OrderId, OrderSlab};

#[derive(Copy, Clone)]
pub struct Symbol(pub u32);

#[derive(Clone)]
pub struct Order {
    symbol_id: Symbol,
    qty: u64,
    price: u64,
}

pub enum AuditResult {
    New(OrderId),
    Amend,
    Cancel,
    Invalid,
}

pub struct OMS {
    orders: Arena<Node<Order>>,
    price_levels: Vec<IntrusiveList>,
    order_index: OrderSlab<OrderRef>,
    min_price: u64,
    tick_size: u64,
    total_processed: u64,
}

#[derive(Copy, Clone)]
struct OrderRef {
    handle: Handle,
    level_idx: u32,
}

impl OMS {
    pub fn new(capacity: usize, min_price: u64, max_price: u64, tick_size: u64) -> Self {
        assert!(capacity <= u32::MAX as usize, "capacity exceeds u32::MAX");
        assert!(tick_size > 0, "tick_size must be > 0");
        assert!(max_price >= min_price, "max_price must be >= min_price");

        let levels = ((max_price - min_price) / tick_size + 1) as usize;
        assert!(levels <= u32::MAX as usize, "levels exceeds u32::MAX");

        Self {
            orders: Arena::with_capacity(capacity),
            price_levels: std::iter::repeat_with(IntrusiveList::new)
                .take(levels)
                .collect(),
            order_index: OrderSlab::with_capacity(capacity),
            min_price,
            tick_size,
            total_processed: 0,
        }
    }

    #[inline(always)]
    fn price_to_level_idx(&self, price: u64) -> Option<usize> {
        if price < self.min_price {
            return None;
        }
        let delta = price - self.min_price;
        if delta % self.tick_size != 0 {
            return None;
        }
        let idx = (delta / self.tick_size) as usize;
        if idx >= self.price_levels.len() {
            return None;
        }
        Some(idx)
    }

    #[inline]
    pub fn handle_new(&mut self, symbol_id: Symbol, qty: u64, price: u64) -> AuditResult {
        self.total_processed += 1;

        if self.order_index.is_full() || self.orders.is_full() {
            return AuditResult::Invalid;
        }

        let level_idx = match self.price_to_level_idx(price) {
            Some(idx) => idx,
            None => return AuditResult::Invalid,
        };

        let order = Order {
            symbol_id,
            qty,
            price,
        };

        let Some(handle) = self.price_levels[level_idx].push_back(&mut self.orders, order) else {
            return AuditResult::Invalid;
        };

        let id = match self.order_index.alloc(OrderRef {
            handle,
            level_idx: level_idx as u32,
        }) {
            Some(id) => id,
            None => {
                let _ = self.price_levels[level_idx].remove(&mut self.orders, handle);
                return AuditResult::Invalid;
            }
        };
        AuditResult::New(id)
    }

    #[inline]
    pub fn handle_amend(&mut self, id: OrderId, new_qty: u64) -> AuditResult {
        self.total_processed += 1;

        let order_ref = match self.order_index.get(id) {
            Some(order_ref) => order_ref,
            None => return AuditResult::Invalid,
        };

        let Some(node) = self.orders.get_mut(order_ref.handle) else {
            return AuditResult::Invalid;
        };

        node.value.qty = new_qty;
        AuditResult::Amend
    }

    #[inline]
    pub fn handle_cancel(&mut self, id: OrderId) -> AuditResult {
        self.total_processed += 1;

        let order_ref = match self.order_index.get(id) {
            Some(order_ref) => order_ref,
            None => return AuditResult::Invalid,
        };

        let level_idx = order_ref.level_idx as usize;
        match self.price_levels[level_idx].remove(&mut self.orders, order_ref.handle) {
            Some(_) => {
                let _ =self.order_index.free(id);
                AuditResult::Cancel
            },
            None => AuditResult::Invalid
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AuditResult, OMS, OrderId, Symbol};

    const MIN_PRICE: u64 = 10;
    const MAX_PRICE: u64 = 20;
    const TICK_SIZE: u64 = 2;

    fn new_oms(capacity: usize) -> OMS {
        OMS::new(capacity, MIN_PRICE, MAX_PRICE, TICK_SIZE)
    }

    fn unwrap_new(result: AuditResult) -> OrderId {
        match result {
            AuditResult::New(id) => id,
            _ => panic!("expected new order"),
        }
    }

    #[test]
    fn new_amend_cancel_happy_path() {
        let mut oms = new_oms(4);
        let id = unwrap_new(oms.handle_new(Symbol(1), 10, 12));

        assert!(matches!(oms.handle_amend(id, 20), AuditResult::Amend));
        assert!(matches!(oms.handle_cancel(id), AuditResult::Cancel));
        assert!(oms.price_levels[1].is_empty());
        assert_eq!(oms.order_index.len(), 0);
    }

    #[test]
    fn new_rejects_price_below_min() {
        let mut oms = new_oms(2);
        assert!(matches!(oms.handle_new(Symbol(1), 10, MIN_PRICE - 1), AuditResult::Invalid));
        assert_eq!(oms.order_index.len(), 0);
    }

    #[test]
    fn new_rejects_price_not_on_tick() {
        let mut oms = new_oms(2);
        assert!(matches!(oms.handle_new(Symbol(1), 10, MIN_PRICE + 1), AuditResult::Invalid));
        assert_eq!(oms.order_index.len(), 0);
    }

    #[test]
    fn new_rejects_price_above_max() {
        let mut oms = new_oms(2);
        assert!(matches!(oms.handle_new(Symbol(1), 10, MAX_PRICE + TICK_SIZE), AuditResult::Invalid));
        assert_eq!(oms.order_index.len(), 0);
    }

    #[test]
    fn new_rejects_when_capacity_full() {
        let mut oms = new_oms(1);
        let _id = unwrap_new(oms.handle_new(Symbol(1), 10, 12));

        assert!(matches!(oms.handle_new(Symbol(1), 10, 12), AuditResult::Invalid));
        assert_eq!(oms.order_index.len(), 1);
    }

    #[test]
    fn amend_rejects_unknown_id() {
        let mut oms = new_oms(1);
        let unknown = OrderId(999);
        assert!(matches!(oms.handle_amend(unknown, 5), AuditResult::Invalid));
        assert_eq!(oms.order_index.len(), 0);
    }

    #[test]
    fn cancel_rejects_unknown_id() {
        let mut oms = new_oms(1);
        let unknown = OrderId(999);
        assert!(matches!(oms.handle_cancel(unknown), AuditResult::Invalid));
        assert_eq!(oms.order_index.len(), 0);
    }

    #[test]
    fn amend_rejects_after_cancel() {
        let mut oms = new_oms(1);
        let id = unwrap_new(oms.handle_new(Symbol(1), 10, 12));
        assert!(matches!(oms.handle_cancel(id), AuditResult::Cancel));
        assert!(matches!(oms.handle_amend(id, 25), AuditResult::Invalid));
    }

    #[test]
    fn double_cancel_is_invalid() {
        let mut oms = new_oms(1);
        let id = unwrap_new(oms.handle_new(Symbol(1), 10, 12));
        assert!(matches!(oms.handle_cancel(id), AuditResult::Cancel));
        assert!(matches!(oms.handle_cancel(id), AuditResult::Invalid));
        assert_eq!(oms.order_index.len(), 0);
    }

    #[test]
    fn id_reuse_after_cancel_changes_generation() {
        let mut oms = new_oms(1);
        let id1 = unwrap_new(oms.handle_new(Symbol(1), 10, 12));
        assert!(matches!(oms.handle_cancel(id1), AuditResult::Cancel));

        let id2 = unwrap_new(oms.handle_new(Symbol(1), 10, 12));
        assert_ne!(id1.0, id2.0);
        assert_eq!(oms.order_index.len(), 1);
    }

    #[test]
    fn cancel_releases_price_level_node() {
        let mut oms = new_oms(2);
        let id1 = unwrap_new(oms.handle_new(Symbol(1), 10, 12));
        let id2 = unwrap_new(oms.handle_new(Symbol(1), 10, 12));

        assert!(matches!(oms.handle_cancel(id1), AuditResult::Cancel));
        assert!(!oms.price_levels[1].is_empty());

        assert!(matches!(oms.handle_cancel(id2), AuditResult::Cancel));
        assert!(oms.price_levels[1].is_empty());
    }
}

const N: usize = 5_000_000;
const MIN_PRICE: u64 = 1;
const MAX_PRICE: u64 = 100_000;
const TICK_SIZE: u64 = 1;

fn main() {
    let mut oms = OMS::new(N, MIN_PRICE, MAX_PRICE, TICK_SIZE);

    use std::hint::black_box;

    // warmup
    for _ in 0..3 {
        for _ in 0..N {
            let id = match black_box(oms.handle_new(Symbol(1), 10, 50_000)) {
                AuditResult::New(id) => id,
                _ => continue,
            };
            black_box(oms.handle_amend(id, 20));
            black_box(oms.handle_cancel(id));
        }
    }

    let start = Instant::now();
    for _ in 0..N {
        let id = match black_box(oms.handle_new(Symbol(1), 10, 50_000)) {
            AuditResult::New(id) => id,
            _ => continue,
        };
        black_box(oms.handle_amend(id, 20));
        black_box(oms.handle_cancel(id));
    }

    let elapsed = start.elapsed();
    println!("Elapsed: {:?}", elapsed);
    let total_ops = (N as f64) * 3.0;
    println!("Throughput: {:.2} M ops/sec",
             total_ops / elapsed.as_secs_f64() / 1_000_000.0
    );
}
