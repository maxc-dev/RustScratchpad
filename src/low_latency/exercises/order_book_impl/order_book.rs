use std::hint::black_box;
use std::iter::repeat_with;
use std::time::{Duration, Instant};
use rustscratchpad::low_latency::model::arena::{Arena, Handle, Node};
use rustscratchpad::low_latency::model::intrusive_list::IntrusiveList;
use rustscratchpad::low_latency::model::order_slab::{OrderId, OrderSlab};

// Tune these depending on your machine and how heavy matching is.
const MIN_PRICE: u64 = 10_000;
const MAX_PRICE: u64 = 20_000;
const TICK_SIZE: u64 = 1;

// Warmup: enough to “touch” code paths and populate caches/branch predictors.
const WARMUP_OPS: usize = 2_000_000;

// Benchmark: do multiple runs, report best/avg-ish.
const RUNS: usize = 5;
const OPS_PER_RUN: usize = 5_000_000;

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Side {
    Buy = 1,
    Sell = 2,
}

#[derive(Copy, Clone)]
pub struct ClientOrderId(u64);

pub struct NewOrder {
    pub client_order_id: ClientOrderId,
    internal_id: OrderId,
    pub price: u64,
    pub qty: u64,
    pub side: Side,
}

pub enum Event {
    Accepted(ClientOrderId, OrderId),
    Filled { maker: ClientOrderId, taker: ClientOrderId, qty: u64 },
    Cancelled(OrderId),
    RejectCancel(OrderId),
    Rejected(ClientOrderId),
}

#[derive(Copy, Clone)]
struct OrderRef {
    handle: Handle,
    level_idx: usize,
    order_side: Side,
}

pub enum CancelResult {
    Success = 1,
    OrderNotFound = 2,
    SuccessWithPartialFailures = 3,
}

enum RemoveOrderResult {
    Success = 1,
    Failure = 2,
}

pub struct OrderBook {
    order_arena: Arena<Node<NewOrder>>,
    price_levels_buy: Vec<IntrusiveList>,
    price_levels_sell: Vec<IntrusiveList>,
    price_level_buy_has_values: Vec<u8>,
    price_level_sell_has_values: Vec<u8>,
    order_index: OrderSlab<OrderRef>,
    min_price: u64,
    max_price: u64,
    tick_size: u64,
    lowest_sell_idx: u32,
    highest_buy_idx: u32,
}

impl OrderBook {
    pub fn new(min_price: u64, max_price: u64, tick_size: u64) -> Self {
        assert!(tick_size.is_power_of_two());
        assert!(tick_size > 0, "tick_size must be > 0");
        assert!(max_price >= min_price, "max_price must be >= min_price");

        let levels = ((max_price - min_price) / tick_size + 1) as usize;
        assert!(levels <= u32::MAX as usize, "levels exceeds u32::MAX");
        let capacity = OPS_PER_RUN;

        Self {
            order_arena: Arena::with_capacity(capacity),
            price_levels_buy: repeat_with(IntrusiveList::new).take(levels).collect(),
            price_levels_sell: repeat_with(IntrusiveList::new).take(levels).collect(),
            order_index: OrderSlab::with_capacity(capacity),
            price_level_buy_has_values: vec![0; levels],
            price_level_sell_has_values: vec![0; levels],
            min_price,
            max_price,
            tick_size,
            lowest_sell_idx: (levels as u32).saturating_sub(1),
            highest_buy_idx: 0,
        }
    }

    #[inline(always)]
    fn price_to_level_idx(&self, price: u64) -> Option<usize> {
        if price < self.min_price || price > self.max_price { return None; }
        let delta = price - self.min_price;
        let mask = self.tick_size - 1;
        if (delta & mask) != 0 { return None; }
        let idx = (delta >> self.tick_size.trailing_zeros()) as usize;
        if idx >= self.price_levels_buy.len() { return None; }
        Some(idx)
    }

    #[inline(always)]
    fn remove_sell(&mut self, level_idx: usize, handle: Handle) -> RemoveOrderResult {
        if self.price_levels_sell[level_idx].remove_fast(&mut self.order_arena, handle).is_none() {
            return RemoveOrderResult::Failure;
        }
        if self.price_levels_sell[level_idx].is_empty() {
            self.price_level_sell_has_values[level_idx] = 0;
            if level_idx as u32 == self.lowest_sell_idx {
                self.align_lowest_sell_idx();
            }
        }
        RemoveOrderResult::Success
    }

    #[inline(always)]
    fn remove_buy(&mut self, level_idx: usize, handle: Handle) -> RemoveOrderResult {
        if self.price_levels_buy[level_idx].remove_fast(&mut self.order_arena, handle).is_none() {
            return RemoveOrderResult::Failure;
        }
        if self.price_levels_buy[level_idx].is_empty() {
            self.price_level_buy_has_values[level_idx] = 0;
            if level_idx as u32 == self.highest_buy_idx {
                self.align_highest_buy_idx();
            }
        }
        RemoveOrderResult::Success
    }

    #[inline(always)]
    fn align_lowest_sell_idx(&mut self) {
        let levels = self.price_levels_sell.len() as u32;
        while self.lowest_sell_idx < levels
            && self.price_level_sell_has_values[self.lowest_sell_idx as usize] == 0
        {
            self.lowest_sell_idx = self.lowest_sell_idx.saturating_add(1);
        }
        if self.lowest_sell_idx >= levels {
            self.lowest_sell_idx = levels.saturating_sub(1);
        }
    }

    #[inline(always)]
    fn align_highest_buy_idx(&mut self) {
        while self.highest_buy_idx > 0
            && self.price_level_buy_has_values[self.highest_buy_idx as usize] == 0
        {
            self.highest_buy_idx = self.highest_buy_idx.saturating_sub(1);
        }
    }

    #[inline(always)]
    fn is_full(&self) -> bool {
        self.order_arena.is_full() || self.order_index.is_full()
    }

    #[inline(always)]
    fn match_buy_order_ret_qty_remainder(&mut self, client_order_id: ClientOrderId, order_price: u64, mut order_qty: u64, out: &mut Vec<Event>) -> u64 {
        loop {
            let level_idx = self.lowest_sell_idx as usize;
            let head = match self.price_levels_sell[level_idx].head() {
                Some(h) => h,
                None => break,
            };

            let (best_price, best_qty, maker_id, best_internal_id) = {
                let best = self.order_arena.get(head).unwrap();
                (best.value.price, best.value.qty, best.value.client_order_id, best.value.internal_id)
            };
            if order_price < best_price { break; }

            if order_qty >= best_qty {
                order_qty -= best_qty;
                let _ = self.price_levels_sell[level_idx].pop_front(&mut self.order_arena);

                self.order_index.free(best_internal_id);
                if self.price_levels_sell[level_idx].is_empty() {
                    self.price_level_sell_has_values[level_idx] = 0;
                    self.align_lowest_sell_idx();
                }
                //out.push(Event::Filled { maker: maker_id, taker: client_order_id, qty: best_qty });
            } else {
                let fill_qty = order_qty;
                unsafe {
                    let best = self.order_arena.get_unchecked_mut(head);
                    best.value.qty -= fill_qty;
                }
                order_qty = 0;
                //out.push(Event::Filled { maker: maker_id, taker: client_order_id, qty: fill_qty });
                break;
            }
        }
        order_qty
    }

    #[inline(always)]
    fn match_sell_order_ret_qty_remainder(&mut self, client_order_id: ClientOrderId, order_price: u64, order_qty: u64, out: &mut Vec<Event>) -> u64 {
        let mut order_qty = order_qty;
        loop {
            let level_idx = self.highest_buy_idx as usize;
            let head = match self.price_levels_buy[level_idx].head() {
                Some(h) => h,
                None => break,
            };

            let (best_price, best_qty, maker_id, best_internal_id) = {
                let best = self.order_arena.get(head).unwrap();
                (best.value.price, best.value.qty, best.value.client_order_id, best.value.internal_id)
            };
            if order_price > best_price { break; }

            if order_qty >= best_qty {
                order_qty -= best_qty;

                let _ = self.price_levels_buy[level_idx].pop_front(&mut self.order_arena);
                self.order_index.free(best_internal_id);

                if self.price_levels_buy[level_idx].is_empty() {
                    self.price_level_buy_has_values[level_idx] = 0;
                    self.align_highest_buy_idx();
                }
                //out.push(Event::Filled { maker: maker_id, taker: client_order_id, qty: best_qty });
            } else {
                let fill_qty = order_qty;
                unsafe {
                    let best = self.order_arena.get_unchecked_mut(head);
                    best.value.qty -= fill_qty;
                }
                order_qty = 0;
                //out.push(Event::Filled { maker: maker_id, taker: client_order_id, qty: fill_qty });
                break;
            }
        }
        order_qty
    }

    #[inline]
    fn persist_buy_order(&mut self, client_order_id: ClientOrderId, order_price: u64, order_qty: u64, level_idx: usize) -> Option<Handle> {
        let order = NewOrder {
            client_order_id,
            internal_id: OrderId(0),
            price: order_price,
            qty: order_qty,
            side: Side::Buy
        };
        let Some(handle) = self.price_levels_buy[level_idx].push_back(&mut self.order_arena, order) else {
            return None;
        };
        self.price_level_buy_has_values[level_idx] = 1;
        if (level_idx as u32) > self.highest_buy_idx  {
            self.highest_buy_idx = level_idx as u32;
        }
        Some(handle)
    }

    #[inline]
    fn persist_sell_order(&mut self, client_order_id: ClientOrderId, order_price: u64, order_qty: u64, level_idx: usize) -> Option<Handle> {
        let order = NewOrder {
            client_order_id,
            internal_id: OrderId(0),
            price: order_price,
            qty: order_qty,
            side: Side::Sell
        };
        let Some(handle) = self.price_levels_sell[level_idx].push_back(&mut self.order_arena, order) else {
            return None;
        };
        self.price_level_sell_has_values[level_idx] = 1;
        if (level_idx as u32) < self.lowest_sell_idx {
            self.lowest_sell_idx = level_idx as u32;
        }
        Some(handle)
    }

    pub fn submit(&mut self, client_order_id: ClientOrderId, order_price: u64, order_qty: u64, order_side: Side, out: &mut Vec<Event>) -> Option<OrderId> {
        let Some(level_idx) = self.price_to_level_idx(order_price) else {
            //out.push(Event::Rejected(client_order_id));
            return None;
        };

        let order_qty: u64 = match order_side {
            Side::Buy => self.match_buy_order_ret_qty_remainder(client_order_id, order_price, order_qty, out),
            Side::Sell => self.match_sell_order_ret_qty_remainder(client_order_id, order_price, order_qty, out),
        };

        if order_qty == 0 {
            return None;
        }
        if self.is_full() {
            //out.push(Event::Rejected(client_order_id));
            return None;
        }

        let ref_placeholder = OrderRef { handle: Handle(0), level_idx, order_side };
        let Some(exchange_id) = self.order_index.alloc(ref_placeholder) else { return None; };
        let Some(handle) = (match order_side {
            Side::Buy => self.persist_buy_order(client_order_id, order_price, order_qty, level_idx),
            Side::Sell => self.persist_sell_order(client_order_id, order_price, order_qty, level_idx)
        }) else {
            self.order_index.free(exchange_id);
            return None;
        };
        *self.order_index.get_mut(exchange_id).unwrap() = OrderRef { handle, level_idx, order_side };
        self.order_arena.get_mut(handle).unwrap().value.internal_id = exchange_id;
        Some(exchange_id)
    }

    pub fn cancel(&mut self, order_id: OrderId, out: &mut Vec<Event>) -> CancelResult {
        let Some(order_ref) = self.order_index.get(order_id) else {
            //out.push(Event::RejectCancel(order_id));
            return CancelResult::OrderNotFound;
        };
        let result = match order_ref.order_side {
            Side::Buy => self.remove_buy(order_ref.level_idx, order_ref.handle),
            Side::Sell => self.remove_sell(order_ref.level_idx, order_ref.handle),
        };
        match result {
            RemoveOrderResult::Success => {
                self.order_index.free(order_id);
                //out.push(Event::Cancelled(order_id));
                CancelResult::Success
            }
            RemoveOrderResult::Failure => {
                debug_assert!(false, "remove_* failed: list corruption or stale handle");
                //out.push(Event::RejectCancel(order_id));
                CancelResult::SuccessWithPartialFailures
            }
        }
    }
}


#[inline(always)]
fn lcg_next(state: &mut u64) -> u64 {
    // Simple deterministic PRNG (no heap, stable, fast).
    // Constants are common LCG parameters.
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
    *state
}

#[inline(always)]
fn gen_price(seed: &mut u64, min_price: u64, ticks: u64, tick_size: u64) -> u64 {
    // Uniform-ish tick selection.
    let r = lcg_next(seed);
    let idx = (r % ticks) as u64;
    min_price + idx * tick_size
}

#[inline(always)]
fn gen_qty(seed: &mut u64) -> u64 {
    // Keep qty small-ish to encourage partial fills.
    (lcg_next(seed) % 50) + 1
}

fn run_workload(
    ob: &mut OrderBook,
    out: &mut Vec<Event>,
    seed: &mut u64,
    ids: &mut Vec<u64>,
    min_price: u64,
    max_price: u64,
    tick_size: u64,
    ops: usize,
) {
    let ticks = ((max_price - min_price) / tick_size) + 1;

    // Workload mix:
    // - 70% submits (buys/sells alternating-ish)
    // - 30% cancels (random existing)
    // This stresses: match loop + O(1) cancel path + id lookup.
    for i in 0..ops {
        let r = lcg_next(seed);

        if (r % 10) < 7 || ids.is_empty() {
            // Submit
            let side = if (r & 1) == 0 { Side::Buy } else { Side::Sell };
            let price = gen_price(seed, min_price, ticks, tick_size);
            let qty = gen_qty(seed);
            let order_id = lcg_next(seed);

            // Avoid realloc in the hot path by reusing `out`.
            out.clear();
            if let Some(id) = ob.submit(ClientOrderId(order_id), price, qty, side, out) {
                ids.push(id.0);
            }

            // Make sure the compiler can't throw away the work.
            black_box(&*out);
        } else {
            // Cancel
            let victim_idx = (r as usize) % ids.len();
            let victim_id = ids.swap_remove(victim_idx);

            out.clear();
            ob.cancel(OrderId(victim_id), out);
            black_box(&*out);
        }

        // Keep ids bounded to reduce unbounded growth effects.
        if ids.len() > 1_000_000 {
            // Cancel some to shrink (still deterministic).
            for _ in 0..1000 {
                if ids.is_empty() {
                    break;
                }
                let rr = lcg_next(seed);
                let victim_idx = (rr as usize) % ids.len();
                let victim_id = ids.swap_remove(victim_idx);
                out.clear();
                ob.cancel(OrderId(victim_id), out);
                black_box(&*out);
            }
        }

        black_box(i); // also prevent loop from being “too obvious”
    }
}

fn timed_run(
    ob: &mut OrderBook,
    out: &mut Vec<Event>,
    seed: &mut u64,
    ids: &mut Vec<u64>,
    min_price: u64,
    max_price: u64,
    tick_size: u64,
    ops: usize,
) -> Duration {
    let t0 = Instant::now();
    run_workload(
        ob, out, seed, ids, min_price, max_price, tick_size, ops,
    );
    t0.elapsed()
}

fn main() {

    // Keep output/event buffers preallocated to avoid realloc noise.
    let mut out: Vec<Event> = Vec::with_capacity(256);

    // Track live ids for cancels; reserve to avoid growth reallocs.
    let mut ids: Vec<u64> = Vec::with_capacity(1_000_000);

    // Build book.
    let mut ob = OrderBook::new(MIN_PRICE, MAX_PRICE, TICK_SIZE);

    // Deterministic seed so comparisons are stable.
    let mut seed: u64 = 0xC0FFEE_u64;

    // -------- Warmup --------
    // Note: We intentionally reuse the same book/ids across warmup and bench to
    // reflect “steady state” behaviour. If you want clean-slate per run, rebuild.
    let warm = timed_run(
        &mut ob,
        &mut out,
        &mut seed,
        &mut ids,
        MIN_PRICE,
        MAX_PRICE,
        TICK_SIZE,
        WARMUP_OPS,
    );
    black_box(&mut ob);
    black_box(&mut ids);
    println!("warmup: {:?} ({:.2} Mops/s)", warm, (WARMUP_OPS as f64) / warm.as_secs_f64() / 1e6);

    // -------- Timed runs --------
    let mut best = Duration::MAX;
    let mut total_secs = 0.0;

    for r in 0..RUNS {
        let d = timed_run(
            &mut ob,
            &mut out,
            &mut seed,
            &mut ids,
            MIN_PRICE,
            MAX_PRICE,
            TICK_SIZE,
            OPS_PER_RUN,
        );
        if d < best {
            best = d;
        }
        total_secs += d.as_secs_f64();
        black_box(r);

        println!(
            "run {}: {:?} ({:.2} Mops/s)",
            r + 1,
            d,
            (OPS_PER_RUN as f64) / d.as_secs_f64() / 1e6
        );
    }

    let avg = total_secs / (RUNS as f64);
    println!(
        "best: {:?} ({:.2} Mops/s)",
        best,
        (OPS_PER_RUN as f64) / best.as_secs_f64() / 1e6
    );
    println!(
        "avg:  {:.3} s ({:.2} Mops/s)",
        avg,
        (OPS_PER_RUN as f64) / avg / 1e6
    );

    // Prevent whole program from being optimized as “no observable effect”.
    black_box(ob);
    black_box(ids);
    black_box(out);
}



/*
Attempt 1:
    warmup: 228.4103ms (8.76 Mops/s)
    run 1: 871.6142ms (5.74 Mops/s)
    run 2: 985.247ms (5.07 Mops/s)
    run 3: 946.1001ms (5.28 Mops/s)
    run 4: 945.1742ms (5.29 Mops/s)
    run 5: 947.8106ms (5.28 Mops/s)
    best: 871.6142ms (5.74 Mops/s)
    avg:  0.939 s (5.32 Mops/s)



*/
