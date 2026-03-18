use std::iter::repeat_with;
use std::mem::MaybeUninit;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct OrderId(pub u64);

impl OrderId {
    #[inline(always)]
    pub fn new(index: u32, generation: u32) -> Self {
        // Generation fills the leftmost 32 bits, index fills the rightmost 32 bits.
        Self(((generation as u64) << 32) | (index as u64))
    }

    #[inline(always)]
    pub fn index(self) -> u32 {
        // Index as the lower 32 bits of the 64-bit ID.
        self.0 as u32
    }

    #[inline(always)]
    pub fn generation(self) -> u32 {
        // Generation as the upper 32 bits of the 64-bit ID.
        // Shift the upper 32 bits to the right by 32 positions.
        // The upper 32 bits are now in the lower 32-bit positions.
        (self.0 >> 32) as u32
    }
}

/// OrderSlab is a fixed-capacity, low-latency storage pool for orders (or any `T`).
/// In this matching-engine context it provides stable, compact `OrderId` handles
/// for fast lookup/amend/cancel, while keeping allocations out of the hot path by
/// preallocating memory and recycling slots with a generation guard.
///
/// `OrderId` packs `generation` into the high 32 bits and `index` into the low 32
/// bits: `id = (generation << 32) | index`. `index()` reads the low 32 bits, while
/// `generation()` shifts right by 32 to read the high 32 bits. Each time a slot is
/// freed, its generation increments so old IDs are rejected after reuse.
pub struct OrderSlab<T> {
    // Fixed-capacity backing storage. Slots are manually initialized via MaybeUninit.
    storage: Box<[MaybeUninit<T>]>,
    // Per-slot generation counters. IDs store generation in the upper 32 bits.
    generations: Box<[u32]>,
    // Tracks which slots are currently initialized/occupied.
    occupied: Box<[bool]>,
    // Stack of free indices (LIFO) for O(1) allocation.
    free_list: Vec<u32>,
    // Number of live elements.
    len: usize,
}

impl<T> OrderSlab<T> {
    /// A fixed-capacity, index-addressed slab used for orders. It preallocates
    /// storage once, hands out compact `OrderId` handles, and recycles slots via
    /// a free list with a generation counter to prevent use-after-free.
    pub fn with_capacity(cap: usize) -> Self {
        assert!(cap <= u32::MAX as usize, "capacity exceeds u32::MAX");

        let storage = repeat_with(MaybeUninit::uninit)
            .take(cap)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let generations = vec![0u32; cap].into_boxed_slice();
        let occupied = vec![false; cap].into_boxed_slice();

        let mut free_list = Vec::with_capacity(cap);
        for i in (0..cap as u32).rev() {
            free_list.push(i);
        }

        Self {
            storage,
            generations,
            occupied,
            free_list,
            len: 0,
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.storage.len()
    }

    #[inline(always)]
    pub fn is_full(&self) -> bool {
        self.len == self.capacity()
    }

    #[inline(always)]
    pub fn alloc(&mut self, value: T) -> Option<OrderId> {
        let index = self.free_list.pop()? as usize;
        debug_assert!(!self.occupied[index]);

        self.storage[index].write(value);
        self.occupied[index] = true;
        self.len += 1;

        Some(OrderId::new(index as u32, self.generations[index]))
    }

    #[inline(always)]
    fn get_index(&self, id: OrderId) -> Option<usize> {
        let index = id.index() as usize;
        if index >= self.storage.len() {
            return None;
        }
        Some(index)
    }

    /// Check the generation from the id-arg is the same as what is stored in `generations`
    /// If there is no match, the generation is wrong and the Order ID is stale
    #[inline(always)]
    fn index_unused(&self, index: usize, generation: u32) -> bool {
        !self.occupied[index] || self.generations[index] != generation
    }

    #[inline(always)]
    pub fn get(&self, id: OrderId) -> Option<&T> {
        let index = self.get_index(id)?;
        if self.index_unused(index, id.generation()) {
            return None;
        }

        // Safety: slot is occupied and generation was validated.
        Some(unsafe { self.storage[index].assume_init_ref() })
    }

    #[inline(always)]
    pub fn get_mut(&mut self, id: OrderId) -> Option<&mut T> {
        let index = self.get_index(id)?;
        if self.index_unused(index, id.generation()) {
            return None;
        }

        // Safety: slot is occupied and generation was validated.
        Some(unsafe { self.storage[index].assume_init_mut() })
    }

    #[inline(always)]
    pub fn free(&mut self, id: OrderId) -> Option<T> {
        let index = self.get_index(id)?;
        if self.index_unused(index, id.generation()) {
            return None;
        }

        self.occupied[index] = false;
        self.len -= 1;

        // Safety: slot is occupied and generation was validated.
        let value = unsafe { self.storage[index].assume_init_read() };

        self.generations[index] = self.generations[index].wrapping_add(1);
        self.free_list.push(index as u32);

        Some(value)
    }
}

impl<T> Drop for OrderSlab<T> {
    fn drop(&mut self) {
        for i in 0..self.storage.len() {
            if self.occupied[i] {
                // Safety: occupied tracks initialized elements.
                unsafe {
                    self.storage[i].assume_init_drop();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{OrderId, OrderSlab};
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    #[test]
    fn basic_alloc_get_free_non_copy() {
        let mut slab = OrderSlab::with_capacity(2);

        let id = slab.alloc(String::from("abc")).expect("alloc should succeed");
        assert_eq!(slab.len(), 1);
        assert_eq!(slab.get(id).map(String::as_str), Some("abc"));

        if let Some(v) = slab.get_mut(id) {
            v.push('!');
        }
        assert_eq!(slab.get(id).map(String::as_str), Some("abc!"));

        let removed = slab.free(id).expect("free should succeed");
        assert_eq!(removed, "abc!");
        assert_eq!(slab.len(), 0);
        assert!(slab.get(id).is_none());
    }

    #[test]
    fn full_capacity_returns_none_on_alloc() {
        let mut slab = OrderSlab::with_capacity(2);
        assert!(slab.alloc(10u64).is_some());
        assert!(slab.alloc(20u64).is_some());
        assert!(slab.alloc(30u64).is_none());
        assert!(slab.is_full());
    }

    #[test]
    fn stale_order_id_rejected_after_slot_reuse() {
        let mut slab = OrderSlab::with_capacity(1);

        let old_id = slab.alloc(String::from("first")).unwrap();
        assert!(slab.free(old_id).is_some());

        let new_id = slab.alloc(String::from("second")).unwrap();
        assert_eq!(old_id.index(), new_id.index());
        assert_ne!(old_id.generation(), new_id.generation());

        assert!(slab.get(old_id).is_none());
        assert!(slab.free(old_id).is_none());
        assert_eq!(slab.get(new_id).map(String::as_str), Some("second"));
    }

    #[test]
    fn double_free_is_handled_safely() {
        let mut slab = OrderSlab::with_capacity(1);
        let id = slab.alloc(123u32).unwrap();

        assert_eq!(slab.free(id), Some(123));
        assert_eq!(slab.free(id), None);
    }

    struct DropCounter {
        drops: Arc<AtomicUsize>,
    }

    impl DropCounter {
        fn new(drops: Arc<AtomicUsize>) -> Self {
            Self { drops }
        }
    }

    impl Drop for DropCounter {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn drop_drops_remaining_live_elements_exactly_once() {
        let drops = Arc::new(AtomicUsize::new(0));

        {
            let mut slab = OrderSlab::with_capacity(3);
            let a = slab.alloc(DropCounter::new(drops.clone())).unwrap();
            let _b = slab.alloc(DropCounter::new(drops.clone())).unwrap();

            let removed = slab.free(a).unwrap();
            drop(removed);

            assert_eq!(drops.load(Ordering::Relaxed), 1);
        }

        assert_eq!(drops.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn stress_alloc_free_cycles() {
        let mut slab = OrderSlab::with_capacity(1024);
        let mut ids: Vec<OrderId> = Vec::with_capacity(1024);

        for round in 0..10_000u32 {
            ids.clear();

            for i in 0..1024u32 {
                let id = slab.alloc((round, i)).expect("slot should be available");
                ids.push(id);
            }

            assert!(slab.alloc((round, 999_999)).is_none());

            for &id in &ids {
                let v = slab.get_mut(id).expect("id should be valid");
                v.1 = v.1.wrapping_add(1);
            }

            for &id in &ids {
                let removed = slab.free(id).expect("free should succeed");
                assert_eq!(removed.0, round);
            }

            assert_eq!(slab.len(), 0);
        }
    }
}
