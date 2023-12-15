use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Index, IndexMut};
use serde::{Deserialize, Serialize};
use crate::iter_generator::IterGenerator;

type InnerIndex = usize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	inner: Vec<TValue>,
	indexer: HashMap<TKey, InnerIndex>,
}

impl<TKey, TValue> IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	pub fn new() -> IndexedHashMap<TKey, TValue> {
		return IndexedHashMap {
			inner: Vec::new(),
			indexer: HashMap::new(),
		};
	}
	
	/// Returns the old value if the key already exists.
	pub fn insert(&mut self, key: TKey, value: TValue) -> Option<TValue> {
		if let Some(index) = self.indexer.get(&key) {
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
		if let Some(index) = self.indexer.get(key) {
			return Some(&self.inner[*index]);
		} else {
			return None;
		}
	}
	
	pub fn get_mut(&mut self, key: &TKey) -> Option<&mut TValue> {
		if let Some(index) = self.indexer.get(key) {
			return Some(&mut self.inner[*index]);
		} else {
			return None;
		}
	}
	
	pub fn remove(&mut self, key: &TKey) -> Option<TValue> {
		if let Some(index) = self.indexer.remove(key) {
			return Some(self.inner.remove(index));
		} else {
			return None;
		}
	}
	
	pub fn contains_key(&self, key: &TKey) -> bool {
		return self.indexer.contains_key(key);
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
		return self.indexer.keys();
	}
	
	pub fn values(&self) -> impl ExactSizeIterator<Item = &TValue> {
		return self.inner.iter();
	}
	
	pub fn values_mut(&mut self) -> impl ExactSizeIterator<Item = &mut TValue> {
		return self.inner.iter_mut();
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
		return &self.inner[self.indexer[key]];
	}
}

impl<TKey, TValue> Index<&TKey> for &IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	type Output = TValue;
	
	fn index(&self, key: &TKey) -> &Self::Output {
		return &self.inner[self.indexer[key]];
	}
}

impl<TKey, TValue> Index<&TKey> for &mut IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	type Output = TValue;
	
	fn index(&self, key: &TKey) -> &Self::Output {
		return &self.inner[self.indexer[key]];
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
		return &mut self.inner[self.indexer[key]];
	}
}

impl<TKey, TValue> IndexMut<&TKey> for &mut IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	fn index_mut(&mut self, key: &TKey) -> &mut Self::Output {
		return &mut self.inner[self.indexer[key]];
	}
}

impl<TKey, TValue> FromIterator<(TKey, TValue)> for IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	fn from_iter<T: IntoIterator<Item = (TKey, TValue)>>(iter: T) -> Self {
		let mut map = IndexedHashMap::new();
		for (key, value) in iter {
			map.insert(key, value);
		}
		return map;
	}
}

impl<TKey, TValue> Extend<(TKey, TValue)> for IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	fn extend<T: IntoIterator<Item = (TKey, TValue)>>(&mut self, iter: T) {
		for (key, value) in iter {
			self.insert(key, value);
		}
	}
}

impl<'a, TKey, TValue> IterGenerator<'a, (&'a InnerIndex, &'a TKey, &'a TValue)> for IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	fn iterate(&'a self) -> impl ExactSizeIterator<Item = (&'a InnerIndex, &'a TKey, &'a TValue)> {
		return self.indexer.iter().map(|(key, index)| (index, key, &self.inner[*index]));
	}
}

impl<'a, TKey, TValue> IterGenerator<'a, (&'a InnerIndex, &'a TKey, &'a TValue)> for &'a IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	fn iterate(&'a self) -> impl ExactSizeIterator<Item = (&'a InnerIndex, &'a TKey, &'a TValue)> {
		return self.indexer.iter().map(|(key, index)| (index, key, &self.inner[*index]));
	}
}

impl<'a, TKey, TValue> IterGenerator<'a, (&'a InnerIndex, &'a TKey, &'a TValue)> for &'a mut IndexedHashMap<TKey, TValue> where TKey: Hash + PartialEq + Eq {
	fn iterate(&'a self) -> impl ExactSizeIterator<Item = (&'a InnerIndex, &'a TKey, &'a TValue)> {
		return self.indexer.iter().map(|(key, index)| (index, key, &self.inner[*index]));
	}
}



