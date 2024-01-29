use std::hash::Hash;
use std::ops::{Index, IndexMut};
use bimap::BiHashMap;
use serde::{Deserialize, Serialize};
use crate::iter_generator::IterGenerator;

type InnerIndex = usize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	inner: Vec<TValue>,
	indexer: BiHashMap<TKey, InnerIndex>,
}

impl<TKey, TValue> IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	pub fn new() -> IndexedHashMap<TKey, TValue> {
		return IndexedHashMap {
			inner: Vec::new(),
			indexer: BiHashMap::new(),
		};
	}
	
	/// Returns the old value if the key already exists.
	pub fn insert(&mut self, key: TKey, value: TValue) -> Option<TValue> {
		if let Some(index) = self.indexer.get_by_left(&key) {
			let old_value = std::mem::replace(&mut self.inner[*index], value);
			return Some(old_value);
		} else {
			let index = self.inner.len();
			self.inner.push(value);
			self.indexer.insert(key, index);
			return None;
		}
	}
	
	pub fn get(&self, key: &TKey) -> Option<&TValue> {
		if let Some(index) = self.indexer.get_by_left(key) {
			return Some(&self.inner[*index]);
		} else {
			return None;
		}
	}
	
	pub fn get_mut(&mut self, key: &TKey) -> Option<&mut TValue> {
		if let Some(index) = self.indexer.get_by_left(key) {
			return Some(&mut self.inner[*index]);
		} else {
			return None;
		}
	}

	pub fn try_index(&self, index: usize) -> Option<&TValue> {
		return self.inner.get(index);
	}
	
	pub fn try_index_mut(&mut self, index: usize) -> Option<&mut TValue> {
		return self.inner.get_mut(index);
	}
	
	pub fn key_to_index(&self, key: &TKey) -> Option<&InnerIndex> {
		return self.indexer.get_by_left(key);
	}
	
	pub fn index_to_key(&self, index: &InnerIndex) -> Option<&TKey> {
		return self.indexer.get_by_right(index);
	}
	
	pub fn remove(&mut self, key: &TKey) -> Option<TValue> {
		let Some((_, index)) = self.indexer.remove_by_left(key)
			else {
				return None;
			};
		
		let value = self.inner.swap_remove(index);
		let len = self.inner.len();
		if len == 0 {
			return Some(value);
		}
		
		let moved_index = len - 1;
		
		let Some((moved_key, _)) = self.indexer.remove_by_right(&moved_index)
			else {
				eprintln!("IndexedHashMap::remove: Failed to remove old moved index: {moved_index}");
				return Some(value);
			};
		
		self.indexer.insert(moved_key, index);
		return Some(value);
	}
	
	pub fn contains_key(&self, key: &TKey) -> bool {
		return self.indexer.contains_left(key);
	}

	pub fn contains_index(&self, index: &usize) -> bool {
		return self.indexer.contains_right(index);
	}
	
	pub fn len(&self) -> usize {
		return self.indexer.len();
	}
	
	pub fn is_empty(&self) -> bool {
		return self.indexer.is_empty();
	}
	
	pub fn clear(&mut self) {
		self.inner.clear();
		self.indexer.clear();
	}
	
	pub fn keys(&self) -> impl ExactSizeIterator<Item = &TKey> {
		return self.indexer.left_values();
	}

	pub fn values(&self) -> impl ExactSizeIterator<Item = &TValue> {
		return self.inner.iter();
	}
	
	pub fn values_mut(&mut self) -> impl ExactSizeIterator<Item = &mut TValue> {
		return self.inner.iter_mut();
	}

	pub fn into_iter(self) -> impl IntoIterator<Item = (TKey, TValue)> {
		let mut indexer = self.indexer;
		return self.inner.into_iter().enumerate().map(move |(index, value)| (indexer.remove_by_right(&index).unwrap().0, value));
	}

	pub fn extend_from_slice(&mut self, other: &[(TKey, TValue)]) -> Vec<TValue>
		where TKey: Clone, TValue: Clone {
		return other.iter()
			.filter_map(|(key, value)| 
				self.insert(key.clone(), value.clone()))
			.collect();
	}
	
	pub fn consume_from(&mut self, other: impl IntoIterator<Item = (TKey, TValue)>) -> Vec<TValue> {
		return other.into_iter()
			.filter_map(|(key, value)| 
				self.insert(key, value))
			.collect();
	}
	
	pub fn iter(&self) -> impl Iterator<Item = (usize, &TKey, &TValue)> {
		return self.indexer.iter()
			.map(|(key, index)| 
				(*index, key, &self.inner[*index]));
	}
}

impl<TKey, TValue> Default for IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	fn default() -> Self {
		return IndexedHashMap::new();
	}
}

impl<TKey, TValue> Index<usize> for IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	type Output = TValue;
	
	fn index(&self, index: usize) -> &Self::Output {
		return &self.inner[index];
	}
}

impl<TKey, TValue> Index<usize> for &IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	type Output = TValue;
	
	fn index(&self, index: usize) -> &Self::Output {
		return &self.inner[index];
	}
}

impl<TKey, TValue> Index<usize> for &mut IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	type Output = TValue;
	
	fn index(&self, index: usize) -> &Self::Output {
		return &self.inner[index];
	}
}

impl<TKey, TValue> Index<&TKey> for IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	type Output = TValue;
	
	fn index(&self, key: &TKey) -> &Self::Output {
		return &self.inner[*self.indexer.get_by_left(key).unwrap()];
	}
}

impl<TKey, TValue> Index<&TKey> for &IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	type Output = TValue;
	
	fn index(&self, key: &TKey) -> &Self::Output {
		return &self.inner[*self.indexer.get_by_left(key).unwrap()];
	}
}

impl<TKey, TValue> Index<&TKey> for &mut IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	type Output = TValue;
	
	fn index(&self, key: &TKey) -> &Self::Output {
		return &self.inner[*self.indexer.get_by_left(key).unwrap()];
	}
}

impl<TKey, TValue> IndexMut<usize> for IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		return &mut self.inner[index];
	}
}

impl<TKey, TValue> IndexMut<usize> for &mut IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		return &mut self.inner[index];
	}
}

impl<TKey, TValue> IndexMut<&TKey> for IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	fn index_mut(&mut self, key: &TKey) -> &mut Self::Output {
		return &mut self.inner[*self.indexer.get_by_left(key).unwrap()];
	}
}

impl<TKey, TValue> IndexMut<&TKey> for &mut IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	fn index_mut(&mut self, key: &TKey) -> &mut Self::Output {
		return &mut self.inner[*self.indexer.get_by_left(key).unwrap()];
	}
}

impl<TKey, TValue> FromIterator<(TKey, TValue)> for IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	fn from_iter<T: IntoIterator<Item = (TKey, TValue)>>(iter: T) -> Self {
		return iter.into_iter().collect();
	}
}

impl<TKey, TValue> Extend<(TKey, TValue)> for IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	fn extend<T: IntoIterator<Item = (TKey, TValue)>>(&mut self, iter: T) {
		iter.into_iter()
			.for_each(|(key, value)| {
				self.insert(key, value); });
	}
}

impl<'a, TKey, TValue> IterGenerator<'a, (&'a InnerIndex, &'a TKey, &'a TValue)> for IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	fn iterate(&'a self) -> impl ExactSizeIterator<Item = (&'a InnerIndex, &'a TKey, &'a TValue)> {
		return self.indexer.iter()
			.map(|(key, index)| 
				(index, key, &self.inner[*index]));
	}
}

impl<'a, TKey, TValue> IterGenerator<'a, (&'a InnerIndex, &'a TKey, &'a TValue)> for &'a IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	fn iterate(&'a self) -> impl ExactSizeIterator<Item = (&'a InnerIndex, &'a TKey, &'a TValue)> {
		return self.indexer.iter()
			.map(|(key, index)|
				(index, key, &self.inner[*index]));
	}
}

impl<'a, TKey, TValue> IterGenerator<'a, (&'a InnerIndex, &'a TKey, &'a TValue)> for &'a mut IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	fn iterate(&'a self) -> impl ExactSizeIterator<Item = (&'a InnerIndex, &'a TKey, &'a TValue)> {
		return self.indexer.iter()
			.map(|(key, index)| 
				(index, key, &self.inner[*index]));
	}
}


// Inserting a new key-value pair should add it to the IndexedHashMap
#[test]
fn test_insert_new_key_value_pair() {
	let mut map = IndexedHashMap::new();
	map.insert("key1", "value1");
	assert_eq!(map.get(&"key1"), Some(&"value1"));
}

// Inserting an existing key-value pair should replace the old value with the new one
#[test]
fn test_insert_existing_key_value_pair() {
	let mut map = IndexedHashMap::new();
	map.insert("key1", "value1");
	map.insert("key1", "value2");
	assert_eq!(map.get(&"key1"), Some(&"value2"));
}

// Getting a value by an existing key should return Some(value)
#[test]
fn test_get_existing_key() {
	let mut map = IndexedHashMap::new();
	map.insert("key1", "value1");
	assert_eq!(map.get(&"key1"), Some(&"value1"));
}

// Getting a value by a non-existing key should return None
#[test]
fn test_get_non_existing_key() {
	let map = IndexedHashMap::<&str, i32>::new();
	assert_eq!(map.get(&"key1"), None);
}

// Removing a key-value pair by an existing key should remove it from the IndexedHashMap and return Some(value)
#[test]
fn test_remove_existing_key() {
	let mut map = IndexedHashMap::new();
	map.insert("key1", "value1");
	assert_eq!(map.remove(&"key1"), Some("value1"));
	assert_eq!(map.get(&"key1"), None);
}

// Removing a key-value pair by a non-existing key should return None
#[test]
fn test_remove_non_existing_key() {
	let mut map = IndexedHashMap::<&str, i32>::new();
	assert_eq!(map.remove(&"key1"), None);
}

// Inserting a large number of key-value pairs should not cause a panic or memory issues
#[test]
fn test_insert_large_number_of_key_value_pairs() {
	let mut map = IndexedHashMap::new();
	for i in 0..100000 {
		map.insert(i, i);
	}
	assert_eq!(map.len(), 100000);
}

// Removing a key-value pair from a non-empty IndexedHashMap should not cause a panic or memory issues
#[test]
fn test_remove_from_non_empty_map() {
	let mut map = IndexedHashMap::new();
	map.insert("key1", "value1");
	map.insert("key2", "value2");
	map.insert("key3", "value3");
	assert_eq!(map.remove(&"key2"), Some("value2"));
	assert_eq!(map.len(), 2);
}

// Getting a value by an index that is out of bounds should return None
#[test]
fn test_get_value_by_out_of_bounds_index() {
	let map = IndexedHashMap::<i32, i32>::new();
	assert_eq!(map.try_index(0), None);
}

// Indexing a non-existing key should panic
#[test]
#[should_panic]
fn test_index_non_existing_key() {
	let map = IndexedHashMap::<&str, i32>::new();
	let _ = map[&"key1"];
}


#[test]
fn test_iterate_over_map() {
	let mut map = IndexedHashMap::new();
	map.insert("key1", "value1");
	map.insert("key2", "value2");
	map.insert("key3", "value3");

	let mut expect = vec![
		(&"key1", &"value1"),
		(&"key2", &"value2"),
		(&"key3", &"value3"),
	];

	let mut iter = map.iterate();
	let next: (&InnerIndex, &&str, &&str) = iter.next().unwrap();
	let next_index = expect.iter().enumerate().find(|(_, (key, value))| *key == next.1 && *value == next.2).unwrap().0;
	expect.remove(next_index);

	let next: (&InnerIndex, &&str, &&str) = iter.next().unwrap();
	let next_index = expect.iter().enumerate().find(|(_, (key, value))| *key == next.1 && *value == next.2).unwrap().0;
	expect.remove(next_index);

	let next: (&InnerIndex, &&str, &&str) = iter.next().unwrap();
	let next_index = expect.iter().enumerate().find(|(_, (key, value))| *key == next.1 && *value == next.2).unwrap().0;
	expect.remove(next_index);
	
	assert_eq!(expect.len(), 0);
	assert_eq!(iter.next(), None);
}

