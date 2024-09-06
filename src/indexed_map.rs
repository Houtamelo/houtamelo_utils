use std::{
	cmp::{Ordering, Ord},
	collections::TryReserveError,
	fmt::Debug,
	ops::{Index, IndexMut, RangeBounds},
	vec::Drain,
};

use crate::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndexedMap<Key, Val> {
	vec: Vec<(Key, Val)>,
}

impl<Key, Val> Default for IndexedMap<Key, Val> {
	fn default() -> Self { IndexedMap { vec: Vec::new() } }
}

impl<Key, Val> Deref for IndexedMap<Key, Val> {
	type Target = Vec<(Key, Val)>;
	fn deref(&self) -> &Self::Target { &self.vec }
}

impl<Key, Val> IndexedMap<Key, Val> {
	pub fn new() -> Self { Self::default() }
	pub fn with_capacity(capacity: usize) -> Self { Self { vec: Vec::with_capacity(capacity) } }
	pub fn reserve(&mut self, additional: usize) { self.vec.reserve(additional) }
	pub fn reserve_exact(&mut self, additional: usize) { self.vec.reserve_exact(additional) }
	pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> { self.vec.try_reserve(additional) }
	pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> { self.vec.try_reserve_exact(additional) }
	pub fn shrink_to_fit(&mut self) { self.vec.shrink_to_fit() }
	pub fn shrink_to(&mut self, min_capacity: usize) { self.vec.shrink_to(min_capacity) }
	pub fn truncate(&mut self, len: usize) { self.vec.truncate(len) }
	pub fn swap_remove(&mut self, index: usize) -> (Key, Val) { self.vec.swap_remove(index) }
	pub fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> Drain<'_, (Key, Val)> { self.vec.drain(range) }
	pub fn clear(&mut self) { self.vec.clear(); }
	pub fn pop(&mut self) -> Option<(Key, Val)> { self.vec.pop() }
	
	pub fn retain(&mut self, mut f: impl FnMut(&Key, &Val) -> bool) {
		self.vec.retain(|(k, v)| f(k, v))
	}

	pub fn retain_mut(&mut self, mut f: impl FnMut(&Key, &mut Val) -> bool) {
		self.vec.retain_mut(|(k, v)| f(k, v))
	}
	
	pub fn get(&self, index: usize) -> Option<(&Key, &Val)> {
		self.vec.get(index).map(|(k, v)| (k, v))
	}

	pub fn value_at(&self, index: usize) -> Option<&Val> {
		self.vec.get(index).map(|(_, v)| v)
	}

	pub fn value_at_mut(&mut self, index: usize) -> Option<&mut Val> {
		self.vec.get_mut(index).map(|(_, v)| v)
	}

	pub fn key_at(&self, index: usize) -> Option<&Key> {
		self.vec.get(index).map(|(k, _)| k)
	}

	pub fn remove_at(&mut self, index: usize) -> Option<(Key, Val)> {
		if index >= self.len() {
			None
		} else {
			Some(self.vec.remove(index))
		}
	}

	pub fn swap(&mut self, idx_a: usize, idx_b: usize) {
		self.vec.swap(idx_a, idx_b);
	}

	pub fn keys(&self) -> impl ExactSizeIterator<Item = &Key> {
		self.vec.iter().map(|(k, _)| k)
	}

	pub fn values(&self) -> impl ExactSizeIterator<Item = &Val> {
		self.vec.iter().map(|(_, v)| v)
	}

	pub fn values_mut(&mut self) -> impl ExactSizeIterator<Item = &mut Val> {
		self.vec.iter_mut().map(|(_, v)| v)
	}

	pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Key, &mut Val)> {
		self.vec.iter_mut().map(|(k, v)| (&*k, v))
	}
	
	pub fn sort_by(&mut self, mut compare: impl FnMut(&Key, &Key) -> Ordering) {
		self.vec.sort_by(|(a, _), (b, _)| compare(a, b));
	}
}

impl<Key: PartialEq, Val> IndexedMap<Key, Val> {
	/// Returns the old value if the key already exists.
	///
	/// # Panics
	/// If `index` > `len`
	pub fn insert(&mut self, key: Key, value: Val, index: usize) -> Option<Val> {
		let old_value = self.remove(&key);
		let index =
			if old_value.is_some() {
				index - 1
			} else {
				index
			};

		self.vec.insert(index, (key, value));
		old_value
	}

	/// Returns the old value if the key already exists.
	pub fn push(&mut self, key: Key, value: Val) -> Option<Val> {
		let old_value = self.remove(&key);
		self.vec.push((key, value));
		old_value
	}
	
	pub fn get_mut(&mut self, index: usize) -> Option<(&Key, &mut Val)> {
		self.vec.get_mut(index).map(|(k, v)| (&*k, v))
	}

	pub fn get_value(&self, key: &Key) -> Option<&Val> {
		self.vec.iter().find(|(k, _)| k == key).map(|(_, v)| v)
	}

	pub fn get_value_mut(&mut self, key: &Key) -> Option<&mut Val> {
		self.vec.iter_mut().find(|(k, _)| k == key).map(|(_, v)| v)
	}

	pub fn key_index(&self, key: &Key) -> Option<usize> {
		self.vec.iter().position(|(k, _)| k == key)
	}

	pub fn remove(&mut self, key: &Key) -> Option<Val> {
		self.key_index(key).map(|idx| self.vec.remove(idx).1)
	}

	pub fn contains_key(&self, key: &Key) -> bool {
		self.vec.iter().any(|(k, _)| k == key)
	}

	pub fn append(&mut self, other: &mut Self) {
		self.extend(other.vec.drain(..))
	}
}

impl<Key: Ord, Val> IndexedMap<Key, Val> { 
	pub fn sort(&mut self) {
		self.vec.sort_by(|(a, _), (b, _)| a.cmp(b));
	}
}

impl<Key, Val: Ord> IndexedMap<Key, Val> {
	pub fn sort_by_value(&mut self) {
		self.vec.sort_by(|(_, a), (_, b)| a.cmp(b));
	}
}

impl<Key: PartialEq, Val> Extend<(Key, Val)> for IndexedMap<Key, Val> {
	fn extend<Iter: IntoIterator<Item = (Key, Val)>>(&mut self, iter: Iter) {
		for (key, value) in iter {
			self.push(key, value);
		}
	}
}

impl<Key: PartialEq, Val> IntoIterator for IndexedMap<Key, Val> {
	type Item = (Key, Val);
	type IntoIter = impl Iterator<Item = (Key, Val)>;
	fn into_iter(self) -> Self::IntoIter { self.vec.into_iter() }
}

impl<'a, Key, Val> IntoIterator for &'a IndexedMap<Key, Val> {
	type Item = (&'a Key, &'a Val);
	type IntoIter = impl Iterator<Item = (&'a Key, &'a Val)>;
	fn into_iter(self) -> Self::IntoIter { self.iter().map(|(k, v)| (k, v)) }
}

impl<'a, Key, Val> IntoIterator for &'a mut IndexedMap<Key, Val> {
	type Item = (&'a Key, &'a mut Val);
	type IntoIter = impl Iterator<Item = (&'a Key, &'a mut Val)>;
	fn into_iter(self) -> Self::IntoIter { self.iter_mut() }
}

impl<Key, Val> Index<usize> for IndexedMap<Key, Val> {
	type Output = Val;
	fn index(&self, index: usize) -> &Self::Output { self.value_at(index).unwrap() }
}

impl<Key: PartialEq, Val> Index<&Key> for IndexedMap<Key, Val> {
	type Output = Val;
	fn index(&self, key: &Key) -> &Self::Output { self.get_value(key).unwrap() }
}

impl<Key, Val> IndexMut<usize> for IndexedMap<Key, Val> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output { self.value_at_mut(index).unwrap() }
}

impl<Key: PartialEq, Val> IndexMut<&Key> for IndexedMap<Key, Val> {
	fn index_mut(&mut self, key: &Key) -> &mut Self::Output { self.get_value_mut(key).unwrap() }
}

impl<Key: PartialEq, Val> FromIterator<(Key, Val)> for IndexedMap<Key, Val> {
	fn from_iter<T: IntoIterator<Item = (Key, Val)>>(into_iter: T) -> Self {
		let iter = into_iter.into_iter();
		let size = {
			let (lower, upper) = iter.size_hint();
			usize::max(lower, upper.unwrap_or(0))
		};

		let mut result = Self { vec: Vec::with_capacity(size) };
		result.extend(iter);
		result.vec.shrink_to_fit();
		result
	}
}

// Inserting a new key-value pair should add it to the IndexedHashMap
#[test]
fn test_insert_new_key_value_pair() {
	let mut map = IndexedMap::new();
	map.push("key1", "value1");
	assert_eq!(
		map.get_value(&"key1"),
		Some(&"value1")
	);
}

// Inserting an existing key-value pair should replace the old value with the
// new one
#[test]
fn test_insert_existing_key_value_pair() {
	let mut map = IndexedMap::new();
	map.push("key1", "value1");
	map.push("key1", "value2");
	assert_eq!(
		map.get_value(&"key1"),
		Some(&"value2")
	);
}

// Getting a value by an existing key should return Some(value)
#[test]
fn test_get_existing_key() {
	let mut map = IndexedMap::new();
	map.push("key1", "value1");
	assert_eq!(
		map.get_value(&"key1"),
		Some(&"value1")
	);
}

// Getting a value by a non-existing key should return None
#[test]
fn test_get_non_existing_key() {
	let map = IndexedMap::<&str, i32>::new();
	assert_eq!(
		map.get_value(&"key1"),
		None
	);
}

// Removing a key-value pair by an existing key should remove it from the
// IndexedHashMap and return Some(value)
#[test]
fn test_remove_existing_key() {
	let mut map = IndexedMap::new();
	map.push("key1", "value1");
	assert_eq!(
		map.remove(&"key1"),
		Some("value1")
	);
	assert_eq!(
		map.get_value(&"key1"),
		None
	);
}

// Removing a key-value pair by a non-existing key should return None
#[test]
fn test_remove_non_existing_key() {
	let mut map = IndexedMap::<&str, i32>::new();
	assert_eq!(
		map.remove(&"key1"),
		None
	);
}

// Inserting a large number of key-value pairs should not cause a panic or
// memory issues
#[test]
fn test_insert_large_number_of_key_value_pairs() {
	let mut map = IndexedMap::new();
	for i in 0..100000 {
		map.push(i, i);
	}
	assert_eq!(map.len(), 100000);
}

// Removing a key-value pair from a non-empty IndexedHashMap should not cause a
// panic or memory issues
#[test]
fn test_remove_from_non_empty_map() {
	let mut map = IndexedMap::new();
	map.push("key1", "value1");
	map.push("key2", "value2");
	map.push("key3", "value3");
	assert_eq!(
		map.remove(&"key2"),
		Some("value2")
	);
	assert_eq!(map.len(), 2);
}

// Getting a value by an index that is out of bounds should return None
#[test]
fn test_get_value_by_out_of_bounds_index() {
	let map = IndexedMap::<i32, i32>::new();
	assert_eq!(
		map.value_at(0),
		None
	);
}

// Indexing a non-existing key should panic
#[test]
#[should_panic]
fn test_index_non_existing_key() {
	let map = IndexedMap::<&str, i32>::new();
	let _ = map[&"key1"];
}

#[test]
fn test_iterate_over_map() {
	let mut map = IndexedMap::new();
	map.push("key1", "value1");
	map.push("key2", "value2");
	map.push("key3", "value3");

	let mut expect = vec![
		(&"key1", &"value1"),
		(&"key2", &"value2"),
		(&"key3", &"value3"),
	];

	let mut iter = map.iter().enumerate();
	let next = iter.next().unwrap();

	let next_index = expect
		.iter()
		.enumerate()
		.find(|(_, (key, value))| **key == next.1.0 && **value == next.1.1)
		.unwrap()
		.0;
	
	expect.remove(next_index);

	let next = iter.next().unwrap();
	
	let next_index = expect
		.iter()
		.enumerate()
		.find(|(_, (key, value))| **key == next.1.0 && **value == next.1.1)
		.unwrap()
		.0;
	
	expect.remove(next_index);

	let next = iter.next().unwrap();
	let next_index = expect
		.iter()
		.enumerate()
		.find(|(_, (key, value))| **key == next.1.0 && **value == next.1.1)
		.unwrap()
		.0;
	expect.remove(next_index);

	assert_eq!(expect.len(), 0);
	assert_eq!(iter.next(), None);
}

#[test]
fn test_from_iter() {
	let iter: Vec<(i32, &str)> = vec![(1, "one"), (2, "two"), (3, "three")];
	let _: IndexedMap<i32, &str> = iter.into_iter().collect();
}
