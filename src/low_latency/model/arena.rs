use std::iter::repeat_with;
use std::mem::{needs_drop, MaybeUninit};
use std::ptr::read;

#[derive(Copy, Clone)]
pub struct Handle(pub u64);

impl Handle {
    pub fn index(self) -> usize {
        (self.0 as u32) as usize
    }

    pub fn generation(self) -> u32 {
        (self.0 >> 32) as u32
    }

    pub fn with_inc_gen(mut self) -> Self {
        let index = self.index() as u64;
        let generation = self.generation().wrapping_add(1);
        self.0 = ((generation as u64) << 32) | index;
        self
    }
}

pub struct Arena<T> {
    buffer: Box<[MaybeUninit<T>]>,
    free_list: Vec<Handle>,
    occupied: Box<[bool]>,
    generation: Box<[u32]>,
    capacity: usize,
}

pub struct Node<T> {
    pub value: T,
    pub(crate) prev: Option<Handle>,
    pub(crate) next: Option<Handle>,
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
    pub fn with_capacity(capacity: usize) -> Self {
        assert!(capacity <= u32::MAX as usize);
        
        Arena {
            buffer: repeat_with(MaybeUninit::uninit)
                .take(capacity)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            free_list: (0..capacity).map(|i| Handle(i as u64)).collect(),
            occupied: vec![false; capacity].into_boxed_slice(),
            generation: vec![0; capacity].into_boxed_slice(),
            capacity,
        }
    }
    
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        self.free_list.is_empty()
    }
    
    #[inline(always)]
    fn get_index(&self, handle: Handle) -> Option<usize> {
        let index = handle.index();
        if index >= self.capacity {
            return None;
        }
        let generation = handle.generation();
        if generation != self.generation[index] {
            return None;
        }
        if !self.occupied[index] {
            return None;
        }
        Some(index)
    }

    pub fn alloc(&mut self, value: T) -> Option<Handle> {
        if let Some(handle) = self.free_list.pop() {
            let index = handle.index();
            self.buffer[index] = MaybeUninit::new(value);
            self.occupied[index] = true;
            return Some(handle);
        }
        None
    }

    #[inline(always)]
    pub fn get(&self, handle: Handle) -> Option<&T> {
        let index = self.get_index(handle)?;
        Some(unsafe { self.buffer[index].assume_init_ref() })
    }

    #[inline(always)]
    pub fn get_mut(&mut self, handle: Handle) -> Option<&mut T> {
        let index = self.get_index(handle)?;
        Some(unsafe { self.buffer[index].assume_init_mut() })
    }

    #[inline(always)]
    pub unsafe fn get_unchecked(&self, handle: Handle) -> &T {
        let index = handle.index();
        self.buffer.get_unchecked(index).assume_init_ref()
    }

    #[inline(always)]
    pub unsafe fn get_unchecked_mut(&mut self, handle: Handle) -> &mut T {
        let index = handle.index();
        self.buffer.get_unchecked_mut(index).assume_init_mut()
    }

    pub fn free(&mut self, handle: Handle) -> Option<T> {
        let index = self.get_index(handle)?;

        self.occupied[index] = false;
        self.free_list.push(handle.with_inc_gen());
        self.generation[index] = self.generation[index].wrapping_add(1);

        Some(unsafe {
            let ptr = self.buffer.as_ptr().add(index) as *const T;
            read(ptr)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Arena, Handle};
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    #[test]
    fn alloc_get_get_mut_free_happy_path() {
        let mut arena = Arena::with_capacity(2);

        let h = arena.alloc(String::from("abc")).expect("alloc should succeed");
        assert_eq!(arena.get(h).map(String::as_str), Some("abc"));

        if let Some(v) = arena.get_mut(h) {
            v.push('!');
        }
        assert_eq!(arena.get(h).map(String::as_str), Some("abc!"));

        let removed = arena.free(h).expect("free should succeed");
        assert_eq!(removed, "abc!");
        assert!(arena.get(h).is_none());
    }

    #[test]
    fn full_capacity_returns_none_on_alloc() {
        let mut arena = Arena::with_capacity(1);
        assert!(arena.alloc(10u64).is_some());
        assert!(arena.alloc(20u64).is_none());
    }

    #[test]
    fn invalid_or_unallocated_handles_return_none() {
        let mut arena: Arena<u32> = Arena::with_capacity(2);
        let invalid = Handle(10);
        assert!(arena.get(invalid).is_none());
        assert!(arena.get_mut(invalid).is_none());
        assert!(arena.free(invalid).is_none());

        let unallocated = Handle(0);
        assert!(arena.get(unallocated).is_none());
        assert!(arena.get_mut(unallocated).is_none());
        assert!(arena.free(unallocated).is_none());
    }

    #[test]
    fn double_free_is_handled_safely() {
        let mut arena = Arena::with_capacity(1);
        let h = arena.alloc(123u32).unwrap();

        assert_eq!(arena.free(h), Some(123));
        assert_eq!(arena.free(h), None);
    }

    #[test]
    fn reuse_slot_after_free() {
        let mut arena = Arena::with_capacity(1);
        let h1 = arena.alloc(1u32).unwrap();
        assert_eq!(arena.free(h1), Some(1));
        assert!(arena.get(h1).is_none());

        let h2 = arena.alloc(2u32).unwrap();
        assert_ne!(h1.0, h2.0);
        assert_eq!(arena.get(h2), Some(&2));
    }

    #[test]
    fn stale_handle_is_rejected_after_reuse() {
        let mut arena = Arena::with_capacity(1);
        let h1 = arena.alloc(10u32).unwrap();
        assert_eq!(arena.free(h1), Some(10));

        let h2 = arena.alloc(20u32).unwrap();
        assert_eq!(arena.get(h2), Some(&20));

        assert!(arena.get(h1).is_none());
        assert!(arena.get_mut(h1).is_none());
        assert!(arena.free(h1).is_none());
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
    fn drop_drops_live_elements_exactly_once() {
        let drops = Arc::new(AtomicUsize::new(0));

        {
            let mut arena = Arena::with_capacity(2);
            let a = arena.alloc(DropCounter::new(drops.clone())).unwrap();
            let _b = arena.alloc(DropCounter::new(drops.clone())).unwrap();

            let removed = arena.free(a).unwrap();
            drop(removed);

            assert_eq!(drops.load(Ordering::Relaxed), 1);
        }

        assert_eq!(drops.load(Ordering::Relaxed), 2);
    }
}
