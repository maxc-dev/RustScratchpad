use crate::low_latency::model::arena::{Arena, Handle, Node};

pub struct IntrusiveList {
    head: Option<Handle>,
    tail: Option<Handle>,
}

impl IntrusiveList {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push_back<T>(&mut self, arena: &mut Arena<Node<T>>, value: T) -> Option<Handle> {
        let handle = arena.alloc(Node {
            value,
            prev: self.tail,
            next: None,
        })?;

        match self.tail {
            None => {
                self.head = Some(handle);
                self.tail = Some(handle);
            }
            Some(old_tail) => {
                arena.get_mut(old_tail)?.next = Some(handle);
                self.tail = Some(handle);
            }
        }

        Some(handle)
    }

    pub fn remove<T>(&mut self, arena: &mut Arena<Node<T>>, handle: Handle) -> Option<T> {
        let node = arena.get(handle)?;
        let prev = node.prev;
        let next = node.next;

        // Validate handles are some to prevent failing early and silently
        if let Some(p) = prev { arena.get(p)?; }
        if let Some(n) = next { arena.get(n)?; }

        match prev {
            Some(p) => arena.get_mut(p)?.next = next,
            None => self.head = next,
        }
        match next {
            Some(n) => arena.get_mut(n)?.prev = prev,
            None => self.tail = prev,
        }
        Some(arena.free(handle)?.value)
    }
    
    #[inline(always)]
    pub fn remove_fast<T>(&mut self, arena: &mut Arena<Node<T>>, handle: Handle) -> Option<T> {
        let node = arena.get(handle)?;
        let prev = node.prev;
        let next = node.next;

        match prev {
            Some(p) => arena.get_mut(p)?.next = next,
            None => self.head = next,
        }
        match next {
            Some(n) => arena.get_mut(n)?.prev = prev,
            None => self.tail = prev,
        }
        Some(arena.free(handle)?.value)
    }

    pub fn pop_front<T>(&mut self, arena: &mut Arena<Node<T>>) -> Option<T> {
        let handle = self.head?;
        let next = arena.get(handle)?.next;
        
        // Validate handles are some to prevent failing early and silently
        if let Some(n) = next { arena.get(n)?; }

        match next {
            Some(n) => {
                arena.get_mut(n)?.prev = None;
                self.head = Some(n);
            }
            None => {
                self.head = None;
                self.tail = None;
            }
        }

        Some(arena.free(handle)?.value)
    }

    #[inline(always)]
    pub fn pop_front_fast<T>(&mut self, arena: &mut Arena<Node<T>>) -> Option<T> {
        let handle = self.head?;
        let next = arena.get(handle)?.next;

        match next {
            Some(n) => {
                arena.get_mut(n)?.prev = None;
                self.head = Some(n);
            }
            None => {
                self.head = None;
                self.tail = None;
            }
        }
        Some(arena.free(handle)?.value)
    }

    #[inline(always)]
    pub fn head(&self) -> Option<Handle> { self.head }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::IntrusiveList;
    use crate::low_latency::model::arena::{Arena, Node};

    fn collect_front<T>(list: &mut IntrusiveList, arena: &mut Arena<Node<T>>) -> Vec<T> {
        let mut out = Vec::new();
        while let Some(v) = list.pop_front(arena) {
            out.push(v);
        }
        out
    }

    #[test]
    fn empty_list_pop_front_is_none() {
        let mut arena = Arena::<Node<i32>>::with_capacity(4);
        let mut list = IntrusiveList::new();

        assert!(list.pop_front(&mut arena).is_none());
        assert!(list.is_empty());
    }

    #[test]
    fn push_back_and_pop_front_fifo_order() {
        let mut arena = Arena::<Node<i32>>::with_capacity(4);
        let mut list = IntrusiveList::new();

        list.push_back(&mut arena, 10).unwrap();
        list.push_back(&mut arena, 20).unwrap();
        list.push_back(&mut arena, 30).unwrap();

        assert_eq!(list.pop_front(&mut arena), Some(10));
        assert_eq!(list.pop_front(&mut arena), Some(20));
        assert_eq!(list.pop_front(&mut arena), Some(30));
        assert!(list.pop_front(&mut arena).is_none());
        assert!(list.is_empty());
    }

    #[test]
    fn remove_middle_updates_links() {
        let mut arena = Arena::<Node<i32>>::with_capacity(4);
        let mut list = IntrusiveList::new();

        let h1 = list.push_back(&mut arena, 1).unwrap();
        let h2 = list.push_back(&mut arena, 2).unwrap();
        let _h3 = list.push_back(&mut arena, 3).unwrap();

        assert_eq!(list.remove(&mut arena, h2), Some(2));
        assert_eq!(collect_front(&mut list, &mut arena), vec![1, 3]);
        assert!(list.is_empty());
        assert!(arena.get(h1).is_none());
    }

    #[test]
    fn remove_head_and_tail() {
        let mut arena = Arena::<Node<i32>>::with_capacity(4);
        let mut list = IntrusiveList::new();

        let h1 = list.push_back(&mut arena, 1).unwrap();
        let h2 = list.push_back(&mut arena, 2).unwrap();
        let h3 = list.push_back(&mut arena, 3).unwrap();

        assert_eq!(list.remove(&mut arena, h1), Some(1));
        assert_eq!(list.remove(&mut arena, h3), Some(3));
        assert_eq!(list.pop_front(&mut arena), Some(2));
        assert!(list.pop_front(&mut arena).is_none());

        assert!(arena.get(h1).is_none());
        assert!(arena.get(h2).is_none());
        assert!(arena.get(h3).is_none());
    }

    #[test]
    fn double_remove_returns_none_and_keeps_list_intact() {
        let mut arena = Arena::<Node<i32>>::with_capacity(4);
        let mut list = IntrusiveList::new();

        let h1 = list.push_back(&mut arena, 10).unwrap();
        list.push_back(&mut arena, 20).unwrap();

        assert_eq!(list.remove(&mut arena, h1), Some(10));
        assert!(list.remove(&mut arena, h1).is_none());
        assert_eq!(collect_front(&mut list, &mut arena), vec![20]);
    }

    #[test]
    fn stale_handle_is_rejected_after_reuse() {
        let mut arena = Arena::<Node<i32>>::with_capacity(1);
        let mut list = IntrusiveList::new();

        let h1 = list.push_back(&mut arena, 10).unwrap();
        assert_eq!(list.remove(&mut arena, h1), Some(10));

        let _h2 = list.push_back(&mut arena, 99).unwrap();
        assert!(list.remove(&mut arena, h1).is_none());
        assert_eq!(list.pop_front(&mut arena), Some(99));
        assert!(list.is_empty());
    }
}