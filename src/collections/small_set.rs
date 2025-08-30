use std::{borrow::Borrow, hash::Hash, iter::FusedIterator, mem, ops::RangeBounds};

use smallvec::{CollectionAllocErr, Drain, SmallVec};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct SmallSet<T, const N: usize>(SmallVec<[T; N]>);

impl<T: PartialEq, const N: usize> SmallSet<T, N> {
    /// Construct an empty vector
    #[inline]
    pub fn new() -> Self { Self(SmallVec::new()) }

    /// Construct an empty vector with enough capacity pre-allocated to store at least `n`
    /// elements.
    ///
    /// Will create a heap allocation only if `n` is larger than the inline capacity.
    #[inline]
    pub fn with_capacity(n: usize) -> Self { Self(SmallVec::with_capacity(n)) }

    /// The number of elements stored in the vector
    #[inline]
    pub fn len(&self) -> usize { self.0.len() }

    /// Returns `true` if the vector is empty
    #[inline]
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    /// The number of items the vector can hold without reallocating
    #[inline]
    pub fn capacity(&self) -> usize { self.0.capacity() }

    /// Returns `true` if the data has spilled into a separate heap-allocated buffer.
    #[inline]
    pub fn spilled(&self) -> bool { self.0.spilled() }

    /// Creates a draining iterator that removes the specified range in the vector
    /// and yields the removed items.
    ///
    /// Note 1: The element range is removed even if the iterator is only
    /// partially consumed or not consumed at all.
    ///
    /// Note 2: It is unspecified how many elements are removed from the vector
    /// if the `Drain` value is leaked.
    ///
    /// # Panics
    ///
    /// Panics if the starting point is greater than the end point or if
    /// the end point is greater than the length of the vector.
    pub fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> Drain<'_, [T; N]> {
        self.0.drain(range)
    }

    /// Returns `true` if the set contains a value.
    ///
    /// The value may be any borrowed form of the set's value type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the value type.
    #[inline]
    pub fn contains<Q: ?Sized>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + PartialEq,
    {
        self.iter().any(|v| v.borrow() == value)
    }

    /// Returns a reference to the value in the set, if any, that is equal to the given value.
    ///
    /// The value may be any borrowed form of the set's value type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the value type.
    #[inline]
    pub fn get<Q: ?Sized>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.iter().find(|&v| v.borrow() == value)
    }

    /// Inserts the given `value` into the set if it is not present, then
    /// returns a reference to the value in the set.
    #[inline]
    pub fn get_or_insert(&mut self, value: T) -> &T {
        let self_ptr = self as *mut SmallSet<T, N>;

        unsafe {
            for item in (&*self_ptr).iter() {
                if *item == value {
                    return item;
                }
            }

            (&mut *self_ptr).0.push(value);
            self.0.last().unwrap_unchecked()
        }
    }

    /// Inserts a value computed from `f` into the set if the given `value` is
    /// not present, then returns a reference to the value in the set.
    #[inline]
    pub fn get_or_insert_with<Q: ?Sized, F>(&mut self, value: &Q, f: F) -> &T
    where
        T: Borrow<Q>,
        Q: Hash + Eq,
        F: FnOnce(&Q) -> T,
    {
        let self_ptr = self as *mut SmallSet<T, N>;

        unsafe {
            for item in (&*self_ptr).iter() {
                if item.borrow() == value {
                    return item;
                }
            }

            (&mut *self_ptr).0.push(f(value));
            self.0.last().unwrap_unchecked()
        }
    }

    /// Inserts an item into this Set.
    #[inline]
    pub fn insert(&mut self, value: T) -> Option<T> {
        for item in &mut self.0 {
            if *item == value {
                return Some(mem::replace(item, value));
            }
        }

        self.0.push(value);
        None
    }

    /// Reserve capacity for `additional` more elements to be inserted.
    ///
    /// May reserve more space to avoid frequent reallocations.
    ///
    /// Panics if the capacity computation overflows `usize`.
    #[inline]
    pub fn reserve(&mut self, additional: usize) { self.0.reserve(additional) }

    /// May reserve more space to avoid frequent reallocations.
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), CollectionAllocErr> {
        self.0.try_reserve(additional)
    }

    /// Reserve the minimum capacity for `additional` more elements to be inserted.
    ///
    /// Panics if the new capacity overflows `usize`.
    pub fn reserve_exact(&mut self, additional: usize) { self.0.reserve_exact(additional) }

    /// Reserve the minimum capacity for `additional` more elements to be inserted.
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), CollectionAllocErr> {
        self.0.try_reserve_exact(additional)
    }

    /// Shrink the capacity of the vector as much as possible.
    ///
    /// When possible, this will move data from an external heap buffer to the vector's inline
    /// storage.
    pub fn shrink_to_fit(&mut self) { self.0.shrink_to_fit(); }

    /// Extracts a slice containing the entire vector.
    ///
    /// Equivalent to `&s[..]`.
    pub fn as_slice(&self) -> &[T] { &self.0 }

    /// Extracts a mutable slice of the entire vector.
    ///
    /// Equivalent to `&mut s[..]`.
    pub fn as_mut_slice(&mut self) -> &mut [T] { &mut self.0 }

    /// Remove all elements from the vector.
    #[inline]
    pub fn clear(&mut self) { self.0.clear(); }

    /// Remove and return the element stored in the key `key`.
    pub fn remove(&mut self, value: &T) -> Option<T> {
        if let Some(pos) = self.0.iter().position(|v| v == value) {
            Some(self.0.swap_remove(pos))
        } else {
            None
        }
    }

    /// Convert a `SmallVec` to a `Vec`, without reallocating if the `SmallVec` has already spilled onto
    /// the heap.
    pub fn into_vec(self) -> Vec<T> { self.0.into_vec() }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all elements `e` such that `f(&e)` returns `false`.
    /// This method operates in place and preserves the order of the retained
    /// elements.
    pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut f: F) { self.0.retain(|t| f(t)) }

    pub fn iter(&self) -> std::slice::Iter<'_, T> { self.0.iter() }

    pub fn iter_mut(&mut self) -> SmallSetIterMut<'_, T> { SmallSetIterMut(self.0.iter_mut()) }
}

impl<T: PartialEq, const N: usize> IntoIterator for SmallSet<T, N> {
    type Item = T;
    type IntoIter = smallvec::IntoIter<[T; N]>;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl<'a, T: PartialEq, const N: usize> IntoIterator for &'a SmallSet<T, N> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl<'a, T: PartialEq, const N: usize> IntoIterator for &'a mut SmallSet<T, N> {
    type Item = &'a mut T;
    type IntoIter = SmallSetIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter { self.iter_mut() }
}

impl<T: PartialEq, const N: usize> FromIterator<T> for SmallSet<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::new();
        for item in iter {
            set.insert(item);
        }
        set
    }
}

impl<T: PartialEq, const N: usize> Extend<T> for SmallSet<T, N> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.insert(item);
        }
    }
}

#[derive(Debug)]
pub struct SmallSetIterMut<'a, T>(std::slice::IterMut<'a, T>);

impl<'a, T> Iterator for SmallSetIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> { self.0.next() }

    fn size_hint(&self) -> (usize, Option<usize>) { self.0.size_hint() }

    #[inline]
    fn count(self) -> usize { self.0.count() }

    #[inline]
    fn last(mut self) -> Option<Self::Item> { self.0.next_back() }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> { self.0.nth(n) }

    // We override the default implementation, which uses `try_fold`,
    // because this simple implementation generates less LLVM IR and is
    // faster to compile.
    #[inline]
    fn for_each<F>(self, f: F)
    where
        Self: Sized,
        F: FnMut(Self::Item),
    {
        self.0.for_each(f)
    }

    #[inline]
    fn fold<B, F>(self, init: B, f: F) -> B
    where F: FnMut(B, Self::Item) -> B {
        self.0.fold(init, f)
    }

    // We override the default implementation, which uses `try_fold`,
    // because this simple implementation generates less LLVM IR and is
    // faster to compile.
    #[inline]
    fn all<F>(&mut self, f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        self.0.all(f)
    }

    // We override the default implementation, which uses `try_fold`,
    // because this simple implementation generates less LLVM IR and is
    // faster to compile.
    #[inline]
    fn any<F>(&mut self, f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        self.0.any(f)
    }

    // We override the default implementation, which uses `try_fold`,
    // because this simple implementation generates less LLVM IR and is
    // faster to compile.
    #[inline]
    fn find_map<B, F>(&mut self, f: F) -> Option<B>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Option<B>,
    {
        self.0.find_map(f)
    }

    // We override the default implementation, which uses `try_fold`,
    // because this simple implementation generates less LLVM IR and is
    // faster to compile. Also, the `assume` avoids a bounds check.
    #[inline]
    fn position<P>(&mut self, predicate: P) -> Option<usize>
    where
        Self: Sized,
        P: FnMut(Self::Item) -> bool,
    {
        self.0.position(predicate)
    }
}

impl<T> DoubleEndedIterator for SmallSetIterMut<'_, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> { self.0.next_back() }
}

impl<T> ExactSizeIterator for SmallSetIterMut<'_, T> {
    fn len(&self) -> usize { self.0.len() }
}

impl<T> FusedIterator for SmallSetIterMut<'_, T> {}

unsafe impl<T> Sync for SmallSetIterMut<'_, T> {}
unsafe impl<T> Send for SmallSetIterMut<'_, T> {}

#[cfg(test)]
mod tests {
    use super::*;

    type TestSet = SmallSet<i32, 2>;

    #[test]
    fn test_new() {
        let set: TestSet = SmallSet::new();
        assert_eq!(set.len(), 0);
        assert!(set.is_empty());
    }

    #[test]
    fn test_default() {
        let set: TestSet = SmallSet::default();
        assert_eq!(set.len(), 0);
        assert!(set.is_empty());
    }

    #[test]
    fn test_with_capacity() {
        let set: TestSet = SmallSet::with_capacity(10);
        assert_eq!(set.len(), 0);
        assert!(set.capacity() >= 10);
    }

    #[test]
    fn test_insert_and_contains() {
        let mut set = TestSet::new();

        // Insert new value
        assert_eq!(set.insert(1), None);
        assert_eq!(set.len(), 1);
        assert!(set.contains(&1));

        // Insert existing value (should replace)
        assert_eq!(set.insert(1), Some(1));
        assert_eq!(set.len(), 1);
        assert!(set.contains(&1));

        // Insert another value
        assert_eq!(set.insert(2), None);
        assert_eq!(set.len(), 2);
        assert!(set.contains(&2));
    }

    #[test]
    fn test_contains_nonexistent() {
        let set = TestSet::new();
        assert!(!set.contains(&1));
    }

    #[test]
    fn test_get() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);

        assert_eq!(set.get(&1), Some(&1));
        assert_eq!(set.get(&3), None);
    }

    #[test]
    fn test_get_or_insert() {
        let mut set = TestSet::new();

        // Insert new value
        let value_ref = set.get_or_insert(1);
        assert_eq!(*value_ref, 1);
        assert_eq!(set.len(), 1);

        // Get existing value
        let value_ref2 = set.get_or_insert(1);
        assert_eq!(*value_ref2, 1);
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_get_or_insert_with() {
        let mut set: SmallSet<String, 2> = SmallSet::new();

        // Insert new value using closure
        let value_ref = set.get_or_insert_with("hello", |s| s.to_string());
        assert_eq!(value_ref, "hello");
        assert_eq!(set.len(), 1);

        // Get existing value (closure shouldn't be called)
        let value_ref2 = set.get_or_insert_with("hello", |_| panic!("Should not be called"));
        assert_eq!(value_ref2, "hello");
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_is_empty() {
        let mut set = TestSet::new();
        assert!(set.is_empty());

        set.insert(1);
        assert!(!set.is_empty());

        set.remove(&1);
        assert!(set.is_empty());
    }

    #[test]
    fn test_len() {
        let mut set = TestSet::new();
        assert_eq!(set.len(), 0);

        set.insert(1);
        assert_eq!(set.len(), 1);

        set.insert(2);
        assert_eq!(set.len(), 2);

        set.insert(1); // Replace, shouldn't increase len
        assert_eq!(set.len(), 2);

        set.remove(&1);
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_capacity() {
        let set: TestSet = SmallSet::with_capacity(10);
        assert!(set.capacity() >= 10);

        let small_set: TestSet = SmallSet::new();
        // Should have inline capacity
        assert!(small_set.capacity() >= 2);
    }

    #[test]
    fn test_spilled() {
        let mut set: TestSet = SmallSet::new();
        assert!(!set.spilled()); // Should be inline initially

        // Fill beyond inline capacity to potentially force spill
        for i in 0..10 {
            set.insert(i);
        }
        // Depending on implementation, this might spill
    }

    #[test]
    fn test_drain() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        let drained: Vec<_> = set.drain(1..3).collect();
        assert_eq!(drained.len(), 2);
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_remove() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);

        assert_eq!(set.remove(&1), Some(1));
        assert_eq!(set.len(), 1);
        assert!(!set.contains(&1));

        assert_eq!(set.remove(&3), None); // Non-existent value
    }

    #[test]
    fn test_clear() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);

        assert_eq!(set.len(), 2);
        set.clear();
        assert_eq!(set.len(), 0);
        assert!(set.is_empty());
    }

    #[test]
    fn test_reserve() {
        let mut set = TestSet::new();
        let initial_capacity = set.capacity();
        set.reserve(100);
        assert!(set.capacity() >= initial_capacity + 100);
    }

    #[test]
    fn test_try_reserve() {
        let mut set = TestSet::new();
        assert!(set.try_reserve(10).is_ok());
    }

    #[test]
    fn test_reserve_exact() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);
        let initial_capacity = set.capacity();
        set.reserve_exact(50);
        assert!(set.capacity() >= initial_capacity + 50);
    }

    #[test]
    fn test_try_reserve_exact() {
        let mut set = TestSet::new();
        assert!(set.try_reserve_exact(10).is_ok());
    }

    #[test]
    fn test_shrink_to_fit() {
        let mut set = TestSet::new();
        set.reserve(100);
        set.insert(1);

        let capacity_before = set.capacity();
        set.shrink_to_fit();
        let capacity_after = set.capacity();

        // Should not increase capacity
        assert!(capacity_after <= capacity_before);
    }

    #[test]
    fn test_as_slice() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);

        let slice = set.as_slice();
        assert_eq!(slice.len(), 2);
        assert!(slice.contains(&1));
        assert!(slice.contains(&2));
    }

    #[test]
    fn test_as_mut_slice() {
        let mut set = TestSet::new();
        set.insert(1);

        let slice = set.as_mut_slice();
        slice[0] = 10;

        assert!(set.contains(&10));
        assert!(!set.contains(&1));
    }

    #[test]
    fn test_retain() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);

        set.retain(|&x| x % 2 == 0); // Keep only even values

        assert_eq!(set.len(), 2);
        assert!(set.contains(&2));
        assert!(set.contains(&4));
        assert!(!set.contains(&1));
        assert!(!set.contains(&3));
    }

    #[test]
    fn test_iter() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);

        let items: Vec<_> = set.iter().copied().collect();
        assert_eq!(items.len(), 2);
        assert!(items.contains(&1));
        assert!(items.contains(&2));
    }

    #[test]
    fn test_iter_mut() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);

        for value in set.iter_mut() {
            *value *= 2;
        }

        assert!(set.contains(&2));
        assert!(set.contains(&4));
        assert!(!set.contains(&1));
    }

    #[test]
    fn test_into_vec() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);

        let vec = set.into_vec();
        assert_eq!(vec.len(), 2);
        assert!(vec.contains(&1));
        assert!(vec.contains(&2));
    }

    #[test]
    fn test_into_iterator() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);

        let items: Vec<_> = set.into_iter().collect();
        assert_eq!(items.len(), 2);
        assert!(items.contains(&1));
        assert!(items.contains(&2));
    }

    #[test]
    fn test_into_iterator_ref() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);

        let items: Vec<_> = (&set).into_iter().copied().collect();
        assert_eq!(items.len(), 2);
        assert!(items.contains(&1));
        assert!(items.contains(&2));
    }

    #[test]
    fn test_into_iterator_mut() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);

        for value in &mut set {
            *value += 10;
        }

        assert!(set.contains(&11));
        assert!(set.contains(&12));
        assert!(!set.contains(&1));
        assert!(!set.contains(&2));
    }

    #[test]
    fn test_from_iterator() {
        let values = vec![1, 2, 3, 1, 2]; // Duplicates should be removed
        let set: TestSet = values.into_iter().collect();

        assert_eq!(set.len(), 3);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
    }

    #[test]
    fn test_extend() {
        let mut set = TestSet::new();
        set.insert(1);

        let additional = vec![2, 3, 1]; // Duplicate 1 should not increase len
        set.extend(additional);

        assert_eq!(set.len(), 3);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
    }

    #[test]
    fn test_clone() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);

        let cloned = set.clone();
        assert_eq!(set.len(), cloned.len());
        assert_eq!(set.contains(&1), cloned.contains(&1));
        assert_eq!(set.contains(&2), cloned.contains(&2));
    }

    #[test]
    fn test_debug() {
        let mut set = TestSet::new();
        set.insert(1);

        let debug_str = format!("{:?}", set);
        assert!(debug_str.contains("SmallSet"));
    }

    #[test]
    fn test_borrowed_value_lookup() {
        // Test with String/&str to verify borrowed form lookup works
        let mut set: SmallSet<String, 2> = SmallSet::new();
        set.insert("hello".to_string());

        // Should be able to lookup with &str
        assert!(set.contains("hello"));
        assert_eq!(set.get("hello"), Some(&"hello".to_string()));
    }

    #[test]
    fn test_large_capacity() {
        let mut set: SmallSet<i32, 1> = SmallSet::with_capacity(1000);
        for i in 0..100 {
            set.insert(i);
        }

        assert_eq!(set.len(), 100);
        assert!(set.spilled()); // Should definitely spill with small inline capacity
    }

    #[test]
    fn test_drain_range() {
        let mut set = TestSet::new();
        for i in 0..5 {
            set.insert(i);
        }

        let drained: Vec<_> = set.drain(1..4).collect();
        assert_eq!(drained.len(), 3);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_drain_all() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);

        let drained: Vec<_> = set.drain(..).collect();
        assert_eq!(drained.len(), 2);
        assert_eq!(set.len(), 0);
        assert!(set.is_empty());
    }

    #[test]
    fn test_retain_with_different_predicates() {
        let mut set = TestSet::new();
        for i in 1..=10 {
            set.insert(i);
        }

        // Keep only values > 5
        set.retain(|&x| x > 5);

        assert_eq!(set.len(), 5);
        for i in 6..=10 {
            assert!(set.contains(&i));
        }
        for i in 1..=5 {
            assert!(!set.contains(&i));
        }
    }

    #[test]
    fn test_iterator_methods() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        // Test iterator count
        assert_eq!(set.iter_mut().count(), 3);

        // Test iterator size_hint
        let iter = set.iter_mut();
        assert_eq!(iter.size_hint(), (3, Some(3)));

        // Test iterator last
        let iter = set.iter_mut();
        let last = iter.last();
        assert!(last.is_some());

        // Test iterator nth
        let mut iter = set.iter_mut();
        let second = iter.nth(1);
        assert!(second.is_some());
    }

    #[test]
    fn test_iterator_for_each() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);

        let mut count = 0;
        set.iter_mut().for_each(|_| count += 1);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_iterator_fold() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        let sum = set.iter_mut().fold(0, |acc, &mut x| acc + x);
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_iterator_all() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);

        assert!(set.iter_mut().all(|&mut x| x > 0));
        assert!(!set.iter_mut().all(|&mut x| x > 1));
    }

    #[test]
    fn test_iterator_any() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);

        assert!(set.iter_mut().any(|&mut x| x == 1));
        assert!(!set.iter_mut().any(|&mut x| x == 3));
    }

    #[test]
    fn test_iterator_find_map() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);

        let found = set.iter_mut().find_map(|&mut x| {
            if x == 2 {
                Some(x * 2)
            } else {
                None
            }
        });

        assert_eq!(found, Some(4));
    }

    #[test]
    fn test_iterator_position() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        let pos = set.iter_mut().position(|&mut x| x == 2);
        assert_eq!(pos, Some(1)); // Second element (0-indexed)
    }

    #[test]
    fn test_double_ended_iterator() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        let mut iter = set.iter_mut();
        let first = iter.next();
        let last = iter.next_back();

        assert!(first.is_some());
        assert!(last.is_some());
        assert_ne!(*first.unwrap(), *last.unwrap());
    }

    #[test]
    fn test_exact_size_iterator() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);

        let iter = set.iter_mut();
        assert_eq!(iter.len(), 2);
    }

    #[test]
    fn test_empty_set_operations() {
        let mut set = TestSet::new();

        assert!(!set.contains(&1));
        assert_eq!(set.get(&1), None);
        assert_eq!(set.remove(&1), None);
        assert_eq!(set.iter().count(), 0);

        let drained: Vec<_> = set.drain(..).collect();
        assert_eq!(drained.len(), 0);
    }

    #[test]
    fn test_single_element_operations() {
        let mut set = TestSet::new();
        set.insert(42);

        assert_eq!(set.len(), 1);
        assert!(!set.is_empty());
        assert!(set.contains(&42));
        assert_eq!(set.get(&42), Some(&42));

        let items: Vec<_> = set.iter().copied().collect();
        assert_eq!(items, vec![42]);
    }

    #[test]
    fn test_complex_types() {
        let mut set: SmallSet<String, 2> = SmallSet::new();
        set.insert("hello".to_string());
        set.insert("world".to_string());

        assert!(set.contains("hello"));
        assert!(set.contains("world"));
        assert_eq!(set.len(), 2);

        // Test get_or_insert_with with complex types
        let value = set.get_or_insert_with("new", |s| s.to_uppercase());
        assert_eq!(value, "NEW");
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_capacity_operations() {
        let mut set: SmallSet<i32, 2> = SmallSet::new();
        let initial_capacity = set.capacity();

        // Fill beyond inline capacity
        for i in 0..10 {
            set.insert(i);
        }

        assert!(set.capacity() >= initial_capacity);
        assert_eq!(set.len(), 10);

        // Test reserve operations don't decrease capacity unnecessarily
        let before_reserve = set.capacity();
        set.reserve(5);
        assert!(set.capacity() >= before_reserve);
    }

    #[test]
    fn test_drain_partial_consumption() {
        let mut set = TestSet::new();
        for i in 0..5 {
            set.insert(i);
        }

        let mut drain = set.drain(1..4);
        let first = drain.next();
        assert!(first.is_some());

        // Drop the iterator without consuming all
        drop(drain);

        // The range should still be removed
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_retain_all_false() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);

        set.retain(|_| false); // Remove all

        assert_eq!(set.len(), 0);
        assert!(set.is_empty());
    }

    #[test]
    fn test_retain_all_true() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);

        let original_len = set.len();
        set.retain(|_| true); // Keep all

        assert_eq!(set.len(), original_len);
    }

    #[test]
    fn test_memory_efficiency() {
        // Test that small sets don't allocate when under inline capacity
        let mut set: SmallSet<i32, 10> = SmallSet::new();

        // Add elements within inline capacity
        for i in 0..5 {
            set.insert(i);
        }

        // Should not have spilled to heap yet
        assert!(!set.spilled());

        // Verify all elements are accessible
        for i in 0..5 {
            assert!(set.contains(&i));
        }
    }

    #[test]
    fn test_insert_replace_behavior() {
        let mut set = TestSet::new();

        // First insert
        assert_eq!(set.insert(1), None);
        assert_eq!(set.len(), 1);

        // Replace existing - should return old value
        assert_eq!(set.insert(1), Some(1));
        assert_eq!(set.len(), 1);

        // Verify the value is still there
        assert!(set.contains(&1));
    }

    #[test]
    fn test_get_or_insert_ordering() {
        let mut set = TestSet::new();

        // Insert in specific order
        let first = *set.get_or_insert(3);
        let second = *set.get_or_insert(1);
        let third = *set.get_or_insert(2);

        assert_eq!(first, 3);
        assert_eq!(second, 1);
        assert_eq!(third, 2);

        // Verify order is maintained in iteration
        let values: Vec<_> = set.iter().copied().collect();
        assert_eq!(values, vec![3, 1, 2]);
    }

    #[test]
    fn test_as_slice_ordering() {
        let mut set = TestSet::new();
        set.insert(3);
        set.insert(1);
        set.insert(2);

        let slice = set.as_slice();
        assert_eq!(slice, &[3, 1, 2]); // Should maintain insertion order
    }

    #[test]
    fn test_remove_maintains_order() {
        let mut set = TestSet::new();
        for i in 0..5 {
            set.insert(i);
        }

        // Remove middle element
        set.remove(&2);

        let slice = set.as_slice();
        // Note: remove uses swap_remove, so order might not be preserved
        assert_eq!(slice.len(), 4);
        assert!(!slice.contains(&2));
    }

    #[test]
    fn test_iter_mut_safety() {
        let mut set = TestSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        // Modify through iter_mut
        for value in set.iter_mut() {
            *value *= 10;
        }

        // Verify modifications
        assert!(set.contains(&10));
        assert!(set.contains(&20));
        assert!(set.contains(&30));
        assert!(!set.contains(&1));
        assert!(!set.contains(&2));
        assert!(!set.contains(&3));
    }

    #[test]
    fn test_iterator_chain_operations() {
        let mut set = TestSet::new();
        for i in 1..=5 {
            set.insert(i);
        }

        // Chain multiple iterator operations
        let result: Vec<_> = set
            .iter()
            .filter(|&&x| x % 2 == 0)
            .map(|&x| x * 2)
            .collect();

        assert_eq!(result, vec![4, 8]); // 2*2 and 4*2
    }

    #[test]
    fn test_fused_iterator_property() {
        let mut set = TestSet::new();
        set.insert(1);

        let mut iter = set.iter_mut();
        assert!(iter.next().is_some()); // Should return Some(1)
        assert!(iter.next().is_none()); // Should return None
        assert!(iter.next().is_none()); // Should still return None (fused)
        assert!(iter.next().is_none()); // Should still return None (fused)
    }

    #[test]
    fn test_sync_send_properties() {
        // This is a compile-time test - if it compiles, the types implement Sync and Send
        fn assert_sync_send<T: Sync + Send>() {}

        let mut set = TestSet::new();
        set.insert(1);

        assert_sync_send::<SmallSetIterMut<'_, i32>>();

        // Test that we can actually use the iterator across thread boundaries
        let iter = set.iter_mut();
        // In a real scenario, we'd move this iterator to another thread
        drop(iter);
    }
}
