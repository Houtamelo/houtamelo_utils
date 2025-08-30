use std::{
	cmp::{Ordering, Ord},
	collections::TryReserveError,
	fmt::Debug,
	ops::{Index, RangeBounds},
	vec::Drain,
};

use crate::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndexedSet<T> {
	vec: Vec<T>,
}

impl<T> Default for IndexedSet<T> {
	fn default() -> Self { IndexedSet { vec: Vec::new() } }
}

impl<T> Deref for IndexedSet<T> {
	type Target = Vec<T>;
	fn deref(&self) -> &Self::Target { &self.vec }
}

impl<T> IndexedSet<T> {
	pub fn new() -> Self { Self::default() }
	pub fn with_capacity(capacity: usize) -> Self { Self { vec: Vec::with_capacity(capacity) } }
	pub fn reserve(&mut self, additional: usize) { self.vec.reserve(additional) }
	pub fn reserve_exact(&mut self, additional: usize) { self.vec.reserve_exact(additional) }
	pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> { self.vec.try_reserve(additional) }
	pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> { self.vec.try_reserve_exact(additional) }
	pub fn shrink_to_fit(&mut self) { self.vec.shrink_to_fit() }
	pub fn shrink_to(&mut self, min_capacity: usize) { self.vec.shrink_to(min_capacity) }
	pub fn truncate(&mut self, len: usize) { self.vec.truncate(len) }
	pub fn swap_remove(&mut self, index: usize) -> T { self.vec.swap_remove(index) }
	pub fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> Drain<'_, T> { self.vec.drain(range) }
	pub fn clear(&mut self) { self.vec.clear(); }
	pub fn pop(&mut self) -> Option<T> { self.vec.pop() }

	pub fn retain(&mut self, f: impl FnMut(&T) -> bool) {
		self.vec.retain(f)
	}
	
	pub fn remove_at(&mut self, index: usize) -> Option<T> {
		if index >= self.len() {
			None
		} else {
			Some(self.vec.remove(index))
		}
	}

	pub fn swap(&mut self, idx_a: usize, idx_b: usize) {
		self.vec.swap(idx_a, idx_b);
	}

	pub fn sort_by(&mut self, mut compare: impl FnMut(&T, &T) -> Ordering) {
		self.vec.sort_by(|a, b| compare(a, b));
	}
}

impl<T: PartialEq> IndexedSet<T> {
	pub fn insert(&mut self, value: T, index: usize) -> bool {
		let old_value = self.remove(&value);
		let index =
			if old_value.is_some() {
				index - 1
			} else {
				index
			};

		self.vec.insert(index, value);
		old_value.is_some()
	}

	pub fn push(&mut self, value: T) -> bool {
		let old_value = self.remove(&value);
		self.vec.push(value);
		old_value.is_some()
	}

	pub fn remove(&mut self, value: &T) -> Option<usize> {
		if let Some(index) = self.index_of(value) {
			self.vec.remove(index);
			Some(index)
		} else {
			None
		}
	}

	pub fn index_of(&self, value: &T) -> Option<usize> {
		self.vec.iter().position(|t| t == value)
	}

	/// Updates the value at the given index with the result of the provided function.
	///
	/// If the "hash" of the new value clashes with an existing value, the existing(old) value is returned.
	/// "hash" is on quotes because this uses PartialEq, not Hash.
	///
	/// # Panics
	/// If `index` is out of bounds.
	pub fn update_at(&mut self, index: usize, f: impl FnOnce(T) -> T) {
		let old_value = self.vec.remove(index);
		let new_value = f(old_value);
		self.insert(new_value, index);
	}

	pub fn append(&mut self, other: &mut Self) {
		self.extend(other.vec.drain(..))
	}
}

impl<T: Ord> IndexedSet<T> {
	pub fn sort(&mut self) {
		self.vec.sort();
	}
}

impl<T: PartialEq> Extend<T> for IndexedSet<T> {
	fn extend<Iter: IntoIterator<Item = T>>(&mut self, iter: Iter) {
		for value in iter {
			self.push(value);
		}
	}
}

impl<T> IntoIterator for IndexedSet<T> {
	type Item = T;
	type IntoIter = impl Iterator<Item = T>;

	fn into_iter(self) -> Self::IntoIter { self.vec.into_iter() }
}

impl<'a, T> IntoIterator for &'a IndexedSet<T> {
	type Item = &'a T;
	type IntoIter = impl Iterator<Item = &'a T>;

	fn into_iter(self) -> Self::IntoIter { self.vec.iter() }
}

impl<'a, T> IntoIterator for &'a mut IndexedSet<T> {
	type Item = &'a T;
	type IntoIter = impl Iterator<Item = &'a T>;

	fn into_iter(self) -> Self::IntoIter { self.vec.iter() }
}

impl<T> Index<usize> for IndexedSet<T> {
	type Output = T;
	fn index(&self, index: usize) -> &Self::Output { self.get(index).unwrap() }
}

impl<T: PartialEq> FromIterator<T> for IndexedSet<T> {
	fn from_iter<I: IntoIterator<Item = T>>(into_iter: I) -> Self {
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

// Inserting a new element into an empty IndexedSet returns None and increases
// the length of the set by 1
#[test]
fn test_insert_new_element_into_empty_set() {
	let mut set = IndexedSet::new();
	assert_eq!(set.push(1), false);
	assert_eq!(set.len(), 1);
}

// Inserting an existing element into an IndexedSet returns Some(value) and does
// not increase the length of the set
#[test]
fn test_insert_existing_element_into_set() {
	let mut set = IndexedSet::new();
	set.push(1);
	assert_eq!(set.push(1), true);
	assert_eq!(set.len(), 1);
}

// Removing an existing element from an IndexedSet returns Some(index) and
// decreases the length of the set by 1
#[test]
fn test_remove_existing_element_from_set() {
	let mut set = IndexedSet::new();
	set.push(1);
	assert_eq!(set.remove(&1), Some(0));
	assert_eq!(set.len(), 0);
}

// Removing a non-existing element from an IndexedSet returns None and does not
// change the length of the set
#[test]
fn test_remove_non_existing_element_from_set() {
	let mut set = IndexedSet::new();
	assert_eq!(set.remove(&1), None);
	assert_eq!(set.len(), 0);
}

// Iterating over an IndexedSet using iter() returns a std::slice::Iter<T> with
// the same elements as the set
#[test]
fn test_iterate_over_set_using_iter() {
	let mut set = IndexedSet::new();
	set.push(1);
	set.push(2);
	set.push(3);
	let mut iter = set.iter();
	assert_eq!(iter.next(), Some(&1));
	assert_eq!(iter.next(), Some(&2));
	assert_eq!(iter.next(), Some(&3));
	assert_eq!(iter.next(), None);
}

#[test]
fn test_from_iter() {
	let iter: Vec<&str> = vec!["one", "two", "three"];
	let _: IndexedSet<&str> = iter.into_iter().collect();
}
