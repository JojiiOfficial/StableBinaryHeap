pub mod item;

use item::HeapItem;
use std::{collections::BinaryHeap, vec::IntoIter};

/// Normal Binary (Max) heap from std::collections::BinaryHeap but returns
/// equal items in inserted order
pub struct StableBinaryHeap<T> {
    heap: BinaryHeap<HeapItem<T>>,
    counter: usize,
}

impl<T: Ord> StableBinaryHeap<T> {
    /// Creates a new stable binary heap
    #[inline]
    pub fn new() -> Self {
        let heap = BinaryHeap::new();
        Self { heap, counter: 0 }
    }

    /// Creates a new stable binary heap with a given capacity
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        let heap = BinaryHeap::with_capacity(capacity);
        Self { heap, counter: 0 }
    }

    /// Pushes a new element on the heap
    #[inline]
    pub fn push(&mut self, item: T) {
        let heap_item = self.new_item(item);
        self.counter += 1;
        self.heap.push(heap_item);
    }

    #[inline]
    fn push_raw(&mut self, item: HeapItem<T>) {
        self.counter = self.counter.max(item.counter);
        self.heap.push(item);
    }

    /// Returns a new HeapItem based wrapping around `inner`.
    /// The StableBinaryHeap's `counter` has to be manually increased after each call
    #[inline]
    fn new_item(&self, inner: T) -> HeapItem<T> {
        let id = self.counter;
        HeapItem::new(inner, id)
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.heap.capacity()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.heap.clear();
        self.counter = 0;
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.heap.iter().map(|i| i.inner())
    }

    #[inline]
    pub fn peek_mut(&mut self) -> Option<std::collections::binary_heap::PeekMut<'_, HeapItem<T>>> {
        self.heap.peek_mut()
    }

    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.heap.reserve(additional)
    }

    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.heap.shrink_to(min_capacity)
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.heap.shrink_to_fit()
    }

    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        self.heap
            .into_vec()
            .into_iter()
            .map(|i| i.into_inner())
            .collect()
    }

    #[inline]
    pub fn into_sorted_vec(self) -> Vec<T> {
        self.into_iter_sorted().collect()
    }

    #[inline]
    pub fn into_iter_sorted(self) -> IntoIterSorted<T> {
        IntoIterSorted { inner: self.heap }
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|i| i.into_inner())
    }

    #[inline]
    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|i| i.inner())
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: Fn(&T) -> bool,
    {
        let tmp: Vec<_> = self.heap.drain().filter(|i| f(&i)).collect();

        assert!(self.is_empty());

        for i in tmp {
            self.push_raw(i);
        }
    }

    /// Get the stable binary heap's counter.
    pub fn counter(&self) -> usize {
        self.counter
    }
}

pub struct Drain<'a, T> {
    iter: std::collections::binary_heap::Drain<'a, HeapItem<T>>,
}

impl<'a, T: Ord> Iterator for Drain<'a, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|i| i.into_inner())
    }
}

impl<T: Ord> IntoIterator for StableBinaryHeap<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.into_vec().into_iter()
    }
}

impl<T: Ord> Extend<T> for StableBinaryHeap<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for i in iter {
            self.push(i);
        }
    }
}

impl<T: Ord> Default for StableBinaryHeap<T> {
    #[inline]
    fn default() -> Self {
        StableBinaryHeap::new()
    }
}

pub struct IntoIterSorted<T> {
    inner: BinaryHeap<HeapItem<T>>,
}

impl<T: Ord> Iterator for IntoIterSorted<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.inner.pop().map(|i| i.into_inner())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.inner.len();
        (exact, Some(exact))
    }
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};

    use crate::*;

    fn generate_data(inp_len: usize) -> Vec<usize> {
        let mut input = vec![0usize; inp_len];
        thread_rng().try_fill(&mut input[..]).unwrap();
        input
    }

    fn make_test(inp_len: usize) {
        let input = generate_data(inp_len);

        let mut expected = input.clone();
        expected.sort_by(|a, b| a.cmp(&b).reverse());

        let mut stable_heap = StableBinaryHeap::new();
        stable_heap.extend(input);

        let out = stable_heap.into_iter_sorted().collect::<Vec<_>>();

        assert_eq!(out, expected);
    }

    #[test]
    fn test_heap_functionality() {
        for inp_len in (1..9000).step_by(51) {
            make_test(inp_len);
        }
    }

    #[test]
    fn test_stability_same() {
        let mut heap = StableBinaryHeap::new();

        for i in 0..1000 {
            heap.push(UniqueItem::new(i, 0));
        }

        let vec = heap.into_sorted_vec();
        for i in 0..1000 {
            assert_eq!(vec[i].item, i as usize);
        }
    }

    #[test]
    fn test_stability_simple() {
        let mut heap = StableBinaryHeap::new();

        heap.push(UniqueItem::new("9", 3));
        heap.push(UniqueItem::new("8", 2));
        heap.push(UniqueItem::new("7", 2));
        heap.push(UniqueItem::new("a", 1));
        heap.push(UniqueItem::new("b", 1));
        heap.push(UniqueItem::new("c", 1));
        heap.push(UniqueItem::new("d", 1));
        heap.push(UniqueItem::new("e", 0));

        let out: Vec<_> = heap.into_iter_sorted().map(|i| i.item).collect();
        assert_eq!(out, vec!["9", "8", "7", "a", "b", "c", "d", "e"]);
    }

    #[test]
    fn test_stability_full() {
        for inp_len in (1..10000).step_by(71) {
            new_stability_test(inp_len);
        }
    }

    #[test]
    fn test_retain() {
        let mut heap = StableBinaryHeap::new();
        for i in 0..=5 {
            heap.push(i);
        }

        heap.retain(|i| *i != 2);

        assert_eq!(heap.into_sorted_vec(), vec![5, 4, 3, 1, 0]);
    }

    fn new_stability_test(inp_len: usize) {
        if inp_len == 0 {
            return;
        }

        let mut heap = StableBinaryHeap::new();

        let data_iter: Vec<_> = generate_data(inp_len)
            .into_iter()
            .map(|i| UniqueItem::new(i.to_string(), i as u32))
            .collect();
        heap.extend(data_iter.clone());

        let mut dups = vec![];

        for i in (0..inp_len / 11).step_by(11) {
            let item = &data_iter[i];
            let name = format!("{}_", item.item);
            let new_item = UniqueItem::new(name, item.val);
            dups.push(new_item.clone());
            heap.push(new_item);
        }

        let sorted = heap.into_sorted_vec();
        let mut last = &sorted[0];
        for i in sorted.iter().skip(1) {
            if i.item.ends_with('_') {
                let mut prev_namae = i.item.clone();
                prev_namae.pop();
                assert_eq!(last.item, prev_namae);
            }

            last = &i;
        }
    }

    struct UniqueItem<T> {
        item: T,
        val: u32,
    }

    impl<T> UniqueItem<T> {
        fn new(item: T, val: u32) -> Self {
            Self { item, val }
        }
    }

    impl<T> PartialEq for UniqueItem<T> {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.val == other.val
        }
    }

    impl<T> Eq for UniqueItem<T> {}

    impl<T> PartialOrd for UniqueItem<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.val.partial_cmp(&other.val)
        }
    }

    impl<T> Ord for UniqueItem<T> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.val.cmp(&other.val)
        }
    }

    impl<T: Clone> Clone for UniqueItem<T> {
        #[inline]
        fn clone(&self) -> Self {
            Self {
                item: self.item.clone(),
                val: self.val,
            }
        }
    }
}
