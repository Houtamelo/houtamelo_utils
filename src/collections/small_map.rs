use std::{
    borrow::Borrow,
    fmt::Debug,
    iter::FusedIterator,
    ops::{Index, IndexMut, RangeBounds},
};

use smallvec::{CollectionAllocErr, Drain, SmallVec};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct SmallMap<K, V, const N: usize>(SmallVec<[(K, V); N]>);

impl<K: PartialEq, V, const N: usize> SmallMap<K, V, N> {
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

    /// An iterator visiting all keys in arbitrary order.
    /// The iterator element type is `&'a K`.
    ///
    /// # Performance
    ///
    /// In the current implementation, iterating over keys takes O(capacity) time
    /// instead of O(len) because it internally visits empty buckets too.
    pub fn keys(&self) -> impl Iterator<Item = &K> { self.0.iter().map(|(k, _)| k) }

    /// Creates a consuming iterator visiting all the keys in arbitrary order.
    /// The map cannot be used after calling this.
    /// The iterator element type is `K`.
    ///
    /// # Performance
    ///
    /// In the current implementation, iterating over keys takes O(capacity) time
    /// instead of O(len) because it internally visits empty buckets too.
    #[inline]
    pub fn into_keys(self) -> impl Iterator<Item = K> { self.0.into_iter().map(|(k, _)| k) }

    /// An iterator visiting all values in arbitrary order.
    /// The iterator element type is `&'a V`.
    ///
    /// # Performance
    ///
    /// In the current implementation, iterating over values takes O(capacity) time
    /// instead of O(len) because it internally visits empty buckets too.
    pub fn values(&self) -> impl Iterator<Item = &V> { self.0.iter().map(|(_, v)| v) }

    /// An iterator visiting all values mutably in arbitrary order.
    /// The iterator element type is `&'a mut V`
    ///
    /// # Performance
    ///
    /// In the current implementation, iterating over values takes O(capacity) time
    /// instead of O(len) because it internally visits empty buckets too.
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.0.iter_mut().map(|(_, v)| v)
    }

    /// Creates a consuming iterator visiting all the values in arbitrary order.
    /// The map cannot be used after calling this.
    /// The iterator element type is `V`.
    ///
    /// # Performance
    ///
    /// In the current implementation, iterating over values takes O(capacity) time
    /// instead of O(len) because it internally visits empty buckets too.
    #[inline]
    pub fn into_values(self) -> impl Iterator<Item = V> { self.0.into_iter().map(|(_, v)| v) }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    #[inline]
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: PartialEq,
    {
        for (key, value) in &self.0 {
            if key.borrow() == k {
                return Some(value);
            }
        }

        None
    }

    /// Returns the key-value pair corresponding to the supplied key. This is
    /// potentially useful:
    /// - for key types where non-identical keys can be considered equal;
    /// - for getting the `&K` stored key value from a borrowed `&Q` lookup key; or
    /// - for getting a reference to a key with the same lifetime as the collection.
    ///
    /// The supplied key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    #[inline]
    pub fn get_key_value<Q: ?Sized>(&self, k: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: PartialEq,
    {
        for (key, value) in &self.0 {
            if key.borrow() == k {
                return Some((key, value));
            }
        }

        None
    }

    /// Returns `true` if the map contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    #[inline]
    pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: PartialEq,
    {
        for (key, _) in &self.0 {
            if key.borrow() == k {
                return true;
            }
        }

        false
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    #[inline]
    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: PartialEq,
    {
        for (key, value) in &mut self.0 {
            if (&*key).borrow() == k {
                return Some(value);
            }
        }

        None
    }

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
    pub fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> Drain<'_, [(K, V); N]> {
        self.0.drain(range)
    }

    /// Inserts an item into this Map.
    #[inline]
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        for (k, v) in &mut self.0 {
            if *k == key {
                return Some(std::mem::replace(v, value));
            }
        }

        self.0.push((key, value));
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
    pub fn as_slice(&self) -> &[(K, V)] { &self.0 }

    /// Extracts a mutable slice of the entire vector.
    ///
    /// Equivalent to `&mut s[..]`.
    pub fn as_mut_slice(&mut self) -> &mut [(K, V)] { &mut self.0 }

    /// Remove all elements from the vector.
    #[inline]
    pub fn clear(&mut self) { self.0.clear(); }

    /// Remove and return the element stored in the key `key`.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(pos) = self.0.iter().position(|(k, _)| k == key) {
            Some(self.0.swap_remove(pos).1)
        } else {
            None
        }
    }

    /// Convert a `SmallVec` to a `Vec`, without reallocating if the `SmallVec` has already spilled onto
    /// the heap.
    pub fn into_vec(self) -> Vec<(K, V)> { self.0.into_vec() }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all elements `e` such that `f(&e)` returns `false`.
    /// This method operates in place and preserves the order of the retained
    /// elements.
    pub fn retain<F: FnMut(&K, &mut V) -> bool>(&mut self, mut f: F) {
        self.0.retain_mut(|(k, v)| f(k, v))
    }

    pub fn iter(&self) -> std::slice::Iter<'_, (K, V)> { self.0.iter() }

    pub fn iter_mut(&mut self) -> SmallMapIterMut<'_, K, V> { SmallMapIterMut(self.0.iter_mut()) }
}

impl<K: PartialEq, V, const N: usize> IntoIterator for SmallMap<K, V, N> {
    type Item = (K, V);
    type IntoIter = smallvec::IntoIter<[(K, V); N]>;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl<'a, K: PartialEq, V, const N: usize> IntoIterator for &'a SmallMap<K, V, N> {
    type Item = &'a (K, V);
    type IntoIter = std::slice::Iter<'a, (K, V)>;

    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl<'a, K: PartialEq, V, const N: usize> IntoIterator for &'a mut SmallMap<K, V, N> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = SmallMapIterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter { self.iter_mut() }
}

impl<K, V, Q, const N: usize> Index<&Q> for SmallMap<K, V, N>
where
    K: PartialEq + Borrow<Q>,
    Q: PartialEq + Debug,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index)
            .unwrap_or_else(|| panic!("No entry found for key {index:?}"))
    }
}

impl<K, V, Q, const N: usize> IndexMut<&Q> for SmallMap<K, V, N>
where
    K: PartialEq + Borrow<Q>,
    Q: PartialEq + Debug,
{
    fn index_mut(&mut self, index: &Q) -> &mut Self::Output {
        self.get_mut(&index)
            .unwrap_or_else(|| panic!("No entry found for key {index:?}"))
    }
}

impl<K: PartialEq, V, const N: usize> FromIterator<(K, V)> for SmallMap<K, V, N> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut set = Self::new();
        for (k, v) in iter {
            set.insert(k, v);
        }
        set
    }
}

impl<K: PartialEq, V, const N: usize> Extend<(K, V)> for SmallMap<K, V, N> {
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

#[derive(Debug)]
pub struct SmallMapIterMut<'a, K, V>(std::slice::IterMut<'a, (K, V)>);

impl<'a, K, V> Iterator for SmallMapIterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> { self.0.next().map(|(k, v)| (&*k, v)) }

    fn size_hint(&self) -> (usize, Option<usize>) { self.0.size_hint() }

    #[inline]
    fn count(self) -> usize { self.0.count() }

    #[inline]
    fn last(mut self) -> Option<Self::Item> { self.0.next_back().map(|(k, v)| (&*k, v)) }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> { self.0.nth(n).map(|(k, v)| (&*k, v)) }

    // We override the default implementation, which uses `try_fold`,
    // because this simple implementation generates less LLVM IR and is
    // faster to compile.
    #[inline]
    fn for_each<F>(self, mut f: F)
    where
        Self: Sized,
        F: FnMut(Self::Item),
    {
        self.0.for_each(|(k, v)| f((&*k, v)))
    }

    #[inline]
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where F: FnMut(B, Self::Item) -> B {
        self.0.fold(init, |acc, (k, v)| f(acc, (&*k, v)))
    }

    // We override the default implementation, which uses `try_fold`,
    // because this simple implementation generates less LLVM IR and is
    // faster to compile.
    #[inline]
    fn all<F>(&mut self, mut f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        self.0.all(|(k, v)| f((&*k, v)))
    }

    // We override the default implementation, which uses `try_fold`,
    // because this simple implementation generates less LLVM IR and is
    // faster to compile.
    #[inline]
    fn any<F>(&mut self, mut f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        self.0.any(|(k, v)| f((&*k, v)))
    }

    // We override the default implementation, which uses `try_fold`,
    // because this simple implementation generates less LLVM IR and is
    // faster to compile.
    #[inline]
    fn find_map<B, F>(&mut self, mut f: F) -> Option<B>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Option<B>,
    {
        self.0.find_map(|(k, v)| f((&*k, v)))
    }

    // We override the default implementation, which uses `try_fold`,
    // because this simple implementation generates less LLVM IR and is
    // faster to compile. Also, the `assume` avoids a bounds check.
    #[inline]
    fn position<P>(&mut self, mut predicate: P) -> Option<usize>
    where
        Self: Sized,
        P: FnMut(Self::Item) -> bool,
    {
        self.0.position(|(k, v)| predicate((&*k, v)))
    }
}

impl<'a, K, V> DoubleEndedIterator for SmallMapIterMut<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> { self.0.next_back().map(|(k, v)| (&*k, v)) }
}

impl<'a, K, V> ExactSizeIterator for SmallMapIterMut<'a, K, V> {
    fn len(&self) -> usize { self.0.len() }
}

impl<'a, K, V> FusedIterator for SmallMapIterMut<'a, K, V> {}

unsafe impl<'a, K, V> Sync for SmallMapIterMut<'a, K, V> {}
unsafe impl<'a, K, V> Send for SmallMapIterMut<'a, K, V> {}

#[cfg(test)]
mod tests {
    use super::*;

    type TestMap = SmallMap<i32, String, 2>;

    #[test]
    fn test_new() {
        let map: TestMap = SmallMap::new();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_default() {
        let map: TestMap = SmallMap::default();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_with_capacity() {
        let map: TestMap = SmallMap::with_capacity(10);
        assert_eq!(map.len(), 0);
        assert!(map.capacity() >= 10);
    }

    #[test]
    fn test_insert_and_get() {
        let mut map = TestMap::new();

        // Insert new key
        assert_eq!(map.insert(1, "one".to_string()), None);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&1), Some(&"one".to_string()));

        // Insert existing key (should replace)
        assert_eq!(map.insert(1, "ONE".to_string()), Some("one".to_string()));
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&1), Some(&"ONE".to_string()));

        // Insert another key
        assert_eq!(map.insert(2, "two".to_string()), None);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&2), Some(&"two".to_string()));
    }

    #[test]
    fn test_get_nonexistent() {
        let map = TestMap::new();
        assert_eq!(map.get(&1), None);
    }

    #[test]
    fn test_get_key_value() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        assert_eq!(map.get_key_value(&1), Some((&1, &"one".to_string())));
        assert_eq!(map.get_key_value(&3), None);
    }

    #[test]
    fn test_contains_key() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());

        assert!(map.contains_key(&1));
        assert!(!map.contains_key(&2));
    }

    #[test]
    fn test_get_mut() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());

        if let Some(value) = map.get_mut(&1) {
            *value = "ONE".to_string();
        }

        assert_eq!(map.get(&1), Some(&"ONE".to_string()));
        assert_eq!(map.get_mut(&2), None);
    }

    #[test]
    fn test_is_empty() {
        let mut map = TestMap::new();
        assert!(map.is_empty());

        map.insert(1, "one".to_string());
        assert!(!map.is_empty());

        map.remove(&1);
        assert!(map.is_empty());
    }

    #[test]
    fn test_len() {
        let mut map = TestMap::new();
        assert_eq!(map.len(), 0);

        map.insert(1, "one".to_string());
        assert_eq!(map.len(), 1);

        map.insert(2, "two".to_string());
        assert_eq!(map.len(), 2);

        map.insert(1, "ONE".to_string()); // Replace, shouldn't increase len
        assert_eq!(map.len(), 2);

        map.remove(&1);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_capacity() {
        let map: TestMap = SmallMap::with_capacity(10);
        assert!(map.capacity() >= 10);

        let small_map: TestMap = SmallMap::new();
        // Should have inline capacity
        assert!(small_map.capacity() >= 2);
    }

    #[test]
    fn test_spilled() {
        let mut map: TestMap = SmallMap::new();
        assert!(!map.spilled()); // Should be inline initially

        // Fill beyond inline capacity to force spill
        for i in 0..10 {
            map.insert(i, format!("value_{}", i));
        }
        // Depending on implementation, this might spill
    }

    #[test]
    fn test_keys() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string());

        let keys: Vec<_> = map.keys().copied().collect();
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&1));
        assert!(keys.contains(&2));
        assert!(keys.contains(&3));
    }

    #[test]
    fn test_into_keys() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        let keys: Vec<_> = map.into_keys().collect();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&1));
        assert!(keys.contains(&2));
    }

    #[test]
    fn test_values() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        let values: Vec<_> = map.values().cloned().collect();
        assert_eq!(values.len(), 2);
        assert!(values.contains(&"one".to_string()));
        assert!(values.contains(&"two".to_string()));
    }

    #[test]
    fn test_values_mut() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        for value in map.values_mut() {
            value.push_str("_modified");
        }

        assert_eq!(map.get(&1), Some(&"one_modified".to_string()));
        assert_eq!(map.get(&2), Some(&"two_modified".to_string()));
    }

    #[test]
    fn test_into_values() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        let values: Vec<_> = map.into_values().collect();
        assert_eq!(values.len(), 2);
        assert!(values.contains(&"one".to_string()));
        assert!(values.contains(&"two".to_string()));
    }

    #[test]
    fn test_drain() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string());

        let drained: Vec<_> = map.drain(1..3).collect();
        assert_eq!(drained.len(), 2);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_remove() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        assert_eq!(map.remove(&1), Some("one".to_string()));
        assert_eq!(map.len(), 1);
        assert!(!map.contains_key(&1));

        assert_eq!(map.remove(&3), None); // Non-existent key
    }

    #[test]
    fn test_clear() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        assert_eq!(map.len(), 2);
        map.clear();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_reserve() {
        let mut map = TestMap::new();
        let initial_capacity = map.capacity();
        map.reserve(100);
        assert!(map.capacity() >= initial_capacity + 100);
    }

    #[test]
    fn test_try_reserve() {
        let mut map = TestMap::new();
        assert!(map.try_reserve(10).is_ok());
    }

    #[test]
    fn test_reserve_exact() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        let initial_capacity = map.capacity();
        map.reserve_exact(50);
        let curr_cap = map.capacity();
        let expect_cap = initial_capacity + 50;
        assert!(curr_cap >= expect_cap, "curr_cap: {curr_cap}, expect_cap: {expect_cap}");
    }

    #[test]
    fn test_try_reserve_exact() {
        let mut map = TestMap::new();
        assert!(map.try_reserve_exact(10).is_ok());
    }

    #[test]
    fn test_shrink_to_fit() {
        let mut map = TestMap::new();
        map.reserve(100);
        map.insert(1, "one".to_string());

        let capacity_before = map.capacity();
        map.shrink_to_fit();
        let capacity_after = map.capacity();

        // Should not increase capacity
        assert!(capacity_after <= capacity_before);
    }

    #[test]
    fn test_as_slice() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        let slice = map.as_slice();
        assert_eq!(slice.len(), 2);
        assert!(slice.contains(&(1, "one".to_string())));
        assert!(slice.contains(&(2, "two".to_string())));
    }

    #[test]
    fn test_as_mut_slice() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());

        let slice = map.as_mut_slice();
        slice[0].1 = "ONE".to_string();

        assert_eq!(map.get(&1), Some(&"ONE".to_string()));
    }

    #[test]
    fn test_retain() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string());

        map.retain(|&k, _| k % 2 == 1); // Keep only odd keys

        assert_eq!(map.len(), 2);
        assert!(map.contains_key(&1));
        assert!(map.contains_key(&3));
        assert!(!map.contains_key(&2));
    }

    #[test]
    fn test_iter() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        let items: Vec<_> = map.iter().collect();
        assert_eq!(items.len(), 2);
        assert!(items.contains(&&(1, "one".to_string())));
        assert!(items.contains(&&(2, "two".to_string())));
    }

    #[test]
    fn test_iter_mut() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        for (_, value) in map.iter_mut() {
            value.push_str("_modified");
        }

        assert_eq!(map.get(&1), Some(&"one_modified".to_string()));
        assert_eq!(map.get(&2), Some(&"two_modified".to_string()));
    }

    #[test]
    fn test_into_vec() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        let vec = map.into_vec();
        assert_eq!(vec.len(), 2);
        assert!(vec.contains(&(1, "one".to_string())));
        assert!(vec.contains(&(2, "two".to_string())));
    }

    #[test]
    fn test_into_iterator() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        let items: Vec<_> = map.into_iter().collect();
        assert_eq!(items.len(), 2);
        assert!(items.contains(&(1, "one".to_string())));
        assert!(items.contains(&(2, "two".to_string())));
    }

    #[test]
    fn test_into_iterator_ref() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        let items: Vec<_> = (&map).into_iter().collect();
        assert_eq!(items.len(), 2);
        assert!(items.contains(&&(1, "one".to_string())));
        assert!(items.contains(&&(2, "two".to_string())));
    }

    #[test]
    fn test_into_iterator_mut() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        for (_, value) in &mut map {
            value.push_str("_mut");
        }

        assert_eq!(map.get(&1), Some(&"one_mut".to_string()));
        assert_eq!(map.get(&2), Some(&"two_mut".to_string()));
    }

    #[test]
    fn test_index() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());

        assert_eq!(map[&1], "one".to_string());
    }

    #[test]
    #[should_panic(expected = "No entry found for key")]
    fn test_index_panic() {
        let map = TestMap::new();
        let _ = &map[&1]; // Should panic
    }

    #[test]
    fn test_index_mut() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());

        map[&1] = "ONE".to_string();
        assert_eq!(map.get(&1), Some(&"ONE".to_string()));
    }

    #[test]
    #[should_panic(expected = "No entry found for key")]
    fn test_index_mut_panic() {
        let mut map = TestMap::new();
        map[&1] = "one".to_string(); // Should panic
    }

    #[test]
    fn test_from_iterator() {
        let pairs = vec![
            (1, "one".to_string()),
            (2, "two".to_string()),
            (1, "ONE".to_string()),
        ];
        let map: TestMap = pairs.into_iter().collect();

        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&1), Some(&"ONE".to_string())); // Last value for duplicate key
        assert_eq!(map.get(&2), Some(&"two".to_string()));
    }

    #[test]
    fn test_extend() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());

        let additional = vec![(2, "two".to_string()), (3, "three".to_string())];
        map.extend(additional);

        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&2), Some(&"two".to_string()));
        assert_eq!(map.get(&3), Some(&"three".to_string()));
    }

    #[test]
    fn test_extend_with_duplicates() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());

        let additional = vec![(1, "ONE".to_string()), (2, "two".to_string())];
        map.extend(additional);

        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&1), Some(&"ONE".to_string())); // Should be replaced
    }

    #[test]
    fn test_clone() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        let cloned = map.clone();
        assert_eq!(map.len(), cloned.len());
        assert_eq!(map.get(&1), cloned.get(&1));
        assert_eq!(map.get(&2), cloned.get(&2));
    }

    #[test]
    fn test_debug() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());

        let debug_str = format!("{:?}", map);
        assert!(debug_str.contains("SmallMap"));
    }

    #[test]
    fn test_borrowed_key_lookup() {
        // Test with String/&str to verify borrowed form lookup works
        let mut map: SmallMap<String, i32, 2> = SmallMap::new();
        map.insert("hello".to_string(), 42);

        // Should be able to lookup with &str
        assert_eq!(map.get("hello"), Some(&42));
        assert!(map.contains_key("hello"));
        assert_eq!(map.get_key_value("hello"), Some((&"hello".to_string(), &42)));
    }

    #[test]
    fn test_get_mut_borrowed_key() {
        let mut map: SmallMap<String, i32, 2> = SmallMap::new();
        map.insert("hello".to_string(), 42);

        if let Some(value) = map.get_mut("hello") {
            *value = 100;
        }

        assert_eq!(map.get("hello"), Some(&100));
    }

    #[test]
    fn test_large_capacity() {
        let mut map: SmallMap<i32, String, 1> = SmallMap::with_capacity(1000);
        for i in 0..100 {
            map.insert(i, format!("value_{}", i));
        }

        assert_eq!(map.len(), 100);
        assert!(map.spilled()); // Should definitely spill with small inline capacity
    }

    #[test]
    fn test_drain_range() {
        let mut map = TestMap::new();
        for i in 0..5 {
            map.insert(i, format!("value_{}", i));
        }

        let drained: Vec<_> = map.drain(1..4).collect();
        assert_eq!(drained.len(), 3);
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_drain_all() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        let drained: Vec<_> = map.drain(..).collect();
        assert_eq!(drained.len(), 2);
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_retain_with_mutation() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string());

        map.retain(|&k, v| {
            if k % 2 == 0 {
                v.push_str("_even");
                true
            } else {
                false
            }
        });

        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&2), Some(&"two_even".to_string()));
        assert!(!map.contains_key(&1));
        assert!(!map.contains_key(&3));
    }

    #[test]
    fn test_iterator_methods() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string());

        // Test iterator count
        assert_eq!(map.iter_mut().count(), 3);

        // Test iterator size_hint
        let iter = map.iter_mut();
        assert_eq!(iter.size_hint(), (3, Some(3)));

        // Test iterator last
        let iter = map.iter_mut();
        let last = iter.last();
        assert!(last.is_some());

        // Test iterator nth
        let mut iter = map.iter_mut();
        let second = iter.nth(1);
        assert!(second.is_some());
    }

    #[test]
    fn test_iterator_for_each() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        let mut count = 0;
        map.iter_mut().for_each(|(..)| count += 1);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_iterator_fold() {
        let mut map = TestMap::new();
        map.insert(1, "1".to_string());
        map.insert(2, "2".to_string());

        let sum = map.iter_mut().fold(0, |acc, (k, _)| acc + k);
        assert_eq!(sum, 3);
    }

    #[test]
    fn test_iterator_all() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        assert!(map.iter_mut().all(|(k, _)| *k > 0));
        assert!(!map.iter_mut().all(|(k, _)| *k > 1));
    }

    #[test]
    fn test_iterator_any() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        assert!(map.iter_mut().any(|(k, _)| *k == 1));
        assert!(!map.iter_mut().any(|(k, _)| *k == 3));
    }

    #[test]
    fn test_iterator_find_map() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        let found = map.iter_mut().find_map(|(k, v)| {
            if *k == 2 {
                Some(v.clone())
            } else {
                None
            }
        });

        assert_eq!(found, Some("two".to_string()));
    }

    #[test]
    fn test_iterator_position() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string());

        let pos = map.iter_mut().position(|(k, _)| *k == 2);
        assert_eq!(pos, Some(1)); // Second element (0-indexed)
    }

    #[test]
    fn test_double_ended_iterator() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string());

        let mut iter = map.iter_mut();
        let first = iter.next();
        let last = iter.next_back();

        assert!(first.is_some());
        assert!(last.is_some());
        assert_ne!(first.unwrap().0, last.unwrap().0);
    }

    #[test]
    fn test_exact_size_iterator() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        let iter = map.iter_mut();
        assert_eq!(iter.len(), 2);
    }

    #[test]
    fn test_empty_map_operations() {
        let mut map = TestMap::new();

        assert_eq!(map.get(&1), None);
        assert_eq!(map.get_mut(&1), None);
        assert_eq!(map.get_key_value(&1), None);
        assert!(!map.contains_key(&1));
        assert_eq!(map.remove(&1), None);
        assert_eq!(map.keys().count(), 0);
        assert_eq!(map.values().count(), 0);
        assert_eq!(map.iter().count(), 0);

        let drained: Vec<_> = map.drain(..).collect();
        assert_eq!(drained.len(), 0);
    }

    #[test]
    fn test_single_element_operations() {
        let mut map = TestMap::new();
        map.insert(42, "answer".to_string());

        assert_eq!(map.len(), 1);
        assert!(!map.is_empty());
        assert_eq!(map.get(&42), Some(&"answer".to_string()));
        assert!(map.contains_key(&42));

        let keys: Vec<_> = map.keys().copied().collect();
        assert_eq!(keys, vec![42]);

        let values: Vec<_> = map.values().cloned().collect();
        assert_eq!(values, vec!["answer".to_string()]);
    }

    #[test]
    fn test_complex_types() {
        let mut map: SmallMap<String, Vec<i32>, 2> = SmallMap::new();
        map.insert("numbers".to_string(), vec![1, 2, 3]);
        map.insert("more_numbers".to_string(), vec![4, 5, 6]);

        assert_eq!(map.get("numbers"), Some(&vec![1, 2, 3]));
        assert_eq!(map.len(), 2);

        if let Some(numbers) = map.get_mut("numbers") {
            numbers.push(4);
        }

        assert_eq!(map.get("numbers"), Some(&vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_capacity_operations() {
        let mut map: SmallMap<i32, String, 2> = SmallMap::new();
        let initial_capacity = map.capacity();

        // Fill beyond inline capacity
        for i in 0..10 {
            map.insert(i, format!("value_{}", i));
        }

        assert!(map.capacity() >= initial_capacity);
        assert_eq!(map.len(), 10);

        // Test reserve operations don't decrease capacity unnecessarily
        let before_reserve = map.capacity();
        map.reserve(5);
        assert!(map.capacity() >= before_reserve);
    }

    #[test]
    fn test_drain_partial_consumption() {
        let mut map = TestMap::new();
        for i in 0..5 {
            map.insert(i, format!("value_{}", i));
        }

        let mut drain = map.drain(1..4);
        let first = drain.next();
        assert!(first.is_some());

        // Drop the iterator without consuming all
        drop(drain);

        // The range should still be removed
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_retain_all_false() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        map.retain(|_, _| false); // Remove all

        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_retain_all_true() {
        let mut map = TestMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        let original_len = map.len();
        map.retain(|_, _| true); // Keep all

        assert_eq!(map.len(), original_len);
    }

    #[test]
    fn test_memory_efficiency() {
        // Test that small maps don't allocate when under inline capacity
        let mut map: SmallMap<i32, i32, 10> = SmallMap::new();

        // Add elements within inline capacity
        for i in 0..5 {
            map.insert(i, i * 2);
        }

        // Should not have spilled to heap yet
        assert!(!map.spilled());

        // Verify all elements are accessible
        for i in 0..5 {
            assert_eq!(map.get(&i), Some(&(i * 2)));
        }
    }
}
