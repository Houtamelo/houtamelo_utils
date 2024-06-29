use std::{
	ops::{RangeBounds, RangeInclusive},
	vec::Drain,
};

use crate::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndexedSet<T>
where T: Hash + PartialEq + Eq
{
	inner:Vec<T>,
}

impl<T> IndexedSet<T>
where T: Hash + PartialEq + Eq
{
	pub fn new() -> IndexedSet<T> { IndexedSet { inner:Vec::new() } }

	pub fn insert(&mut self, value:T) -> Option<T> {
		if !self.inner.contains(&value) {
			self.inner.push(value);
			None
		} else {
			Some(value)
		}
	}

	pub fn remove(&mut self, value:&T) -> Option<usize> {
		let option = self
			.inner
			.iter()
			.enumerate()
			.find_map(|(index, element)| {
				(*element)
					.eq(value)
					.then_some(index)
			});

		if let Some(index) = option {
			self.inner.remove(index);
			Some(index)
		} else {
			None
		}
	}

	pub fn contains(&self, value:&T) -> bool { self.inner.contains(value) }

	pub fn len(&self) -> usize { self.inner.len() }

	pub fn is_empty(&self) -> bool { self.inner.is_empty() }

	pub fn iter(&self) -> std::slice::Iter<T> { self.inner.iter() }

	pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
		self.inner.iter_mut()
	}

	pub fn try_index(&self, index:usize) -> Option<&T> { self.inner.get(index) }

	pub fn try_index_mut(&mut self, index:usize) -> Option<&mut T> {
		self.inner.get_mut(index)
	}

	pub fn clear(&mut self) { self.inner.clear(); }

	pub fn as_slice(&self) -> &[T] { self.inner.as_slice() }

	/// Retains only the elements specified by the predicate.
	/// In other words, remove all elements e for which f(&e) returns false.
	/// This method operates in place, visiting each element exactly once in the
	/// original order, and preserves the order of the retained elements.
	pub fn retain<F>(&mut self, mut f:F)
	where F: FnMut(&T) -> bool {
		self.inner.retain(|x| f(x));
	}

	pub fn get(&self, index:usize) -> Option<&T> { self.inner.get(index) }

	pub fn contains_index(&self, index:usize) -> bool {
		index < self.inner.len()
	}

	pub fn contains_range(&self, range:&RangeInclusive<usize>) -> bool {
		*range.start() < self.inner.len() && *range.end() < self.inner.len()
	}

	/// Removes the specified range from the vector in bulk, returning all
	/// removed elements as an iterator. If the iterator is dropped before being
	/// fully consumed, it drops the remaining removed elements. The returned
	/// iterator keeps a mutable borrow on the vector to optimize its
	/// implementation.
	///
	/// # Panics
	/// If the starting point is greater than the end point or if the end
	/// point is greater than the length of the vector. Leaking
	///
	///
	/// If the returned iterator goes out of scope without being dropped (due to
	/// mem::forget, for example), the vector may have lost and leaked elements
	/// arbitrarily, including elements outside the range.
	pub fn drain<R>(&mut self, range:R) -> Drain<T>
	where R: RangeBounds<usize> {
		self.inner.drain(range)
	}

	pub fn get_range(&self, range:&RangeInclusive<usize>) -> Option<&[T]> {
		self.inner.get(range.clone())
	}

	pub fn with_capacity(capacity:usize) -> Self {
		Self {
			inner:Vec::with_capacity(capacity),
		}
	}

	pub fn capacity(&self) -> usize { self.inner.capacity() }

	/// Reserves capacity for at least additional more elements to be inserted
	/// in the given Vec<T>. The collection may reserve more space to
	/// speculatively avoid frequent reallocations. After calling reserve,
	/// capacity will be greater than or equal to self.len() + additional. Does
	/// nothing if capacity is already sufficient.
	///
	/// # Panics
	/// If the new capacity exceeds isize::MAX bytes.
	pub fn reserve(&mut self, additional:usize) {
		self.inner.reserve(additional);
	}

	/// Shrinks the capacity of the vector as much as possible.
	/// It will drop down as close as possible to the length but the allocator
	/// may still inform the vector that there is space for a few more elements.
	pub fn shrink_to_fit(&mut self) { self.inner.shrink_to_fit(); }

	/// Shortens the vector, keeping the first len elements and dropping the
	/// rest. If len is greater or equal to the vector's current length, this
	/// has no effect. The drain method can emulate truncate, but causes the
	/// excess elements to be returned instead of dropped. Note that this
	/// method has no effect on the allocated capacity of the vector.
	///
	/// # Examples
	/// Truncating a five element vector to two elements:
	/// let mut vec = vec![1, 2, 3, 4, 5];
	/// vec.truncate(2);
	/// assert_eq!(vec, [1, 2]);
	///
	/// No truncation occurs when len is greater than the vector's current
	/// length: let mut vec = vec![1, 2, 3];
	/// vec.truncate(8);
	/// assert_eq!(vec, [1, 2, 3]);
	///
	/// Truncating when len == 0 is equivalent to calling the clear method.
	/// let mut vec = vec![1, 2, 3];
	/// vec.truncate(0);
	/// assert_eq!(vec, []);
	pub fn truncate(&mut self, len:usize) { self.inner.truncate(len); }

	pub fn pop(&mut self) -> Option<T> { self.inner.pop() }

	/// Removes an element from the vector and returns it.
	/// The removed element is replaced by the last element of the vector.
	/// This does not preserve ordering, but is O(1). If you need to preserve
	/// the element order, use remove instead.
	///
	/// # Panics
	/// If index is out of bounds.
	///
	/// # Examples
	/// let mut v = vec!["foo", "bar", "baz", "qux"];
	///
	/// assert_eq!(v.swap_remove(1), "bar");
	/// assert_eq!(v, ["foo", "qux", "baz"]);
	///
	/// assert_eq!(v.swap_remove(0), "foo");
	/// assert_eq!(v, ["baz", "qux"]);
	pub fn swap_remove(&mut self, index:usize) -> T {
		self.inner.swap_remove(index)
	}

	/// Splits the collection into two at the given index.
	/// Returns a newly allocated vector containing the elements in the range
	/// [at, len). After the call, the original vector will be left containing
	/// the elements [0, at) with its previous capacity unchanged.
	///
	/// # Panics
	/// If at > len.
	///
	/// # Examples
	/// let mut vec = vec![1, 2, 3];
	/// let vec2 = vec.split_off(1);
	/// assert_eq!(vec, [1]);
	/// assert_eq!(vec2, [2, 3]);
	pub fn split_off(&mut self, at:usize) -> Self {
		Self {
			inner:self.inner.split_off(at),
		}
	}

	pub fn sort(&mut self)
	where T: Ord {
		self.inner.sort();
	}

	pub fn sort_by<F>(&mut self, compare:F)
	where F: FnMut(&T, &T) -> std::cmp::Ordering {
		self.inner.sort_by(compare);
	}

	pub fn sort_by_key<K, F>(&mut self, f:F)
	where
		F: FnMut(&T) -> K,
		K: Ord,
	{
		self.inner.sort_by_key(f);
	}

	/// Sorts the slice, but might not preserve the order of equal elements.
	/// This sort is unstable (i.e., may reorder equal elements), in-place
	/// (i.e., does not allocate), and O(n * log(n)) worst-case.
	/// Current implementation
	/// The current algorithm is based on pattern-defeating quicksort  by Orson
	/// Peters, which combines the fast average case of randomized quicksort
	/// with the fast worst case of heapsort, while achieving linear time on
	/// slices with certain patterns. It uses some randomization to avoid
	/// degenerate cases, but with a fixed seed to always provide deterministic
	/// behavior. It is typically faster than stable sorting, except in a few
	/// special cases, e.g., when the slice consists of several concatenated
	/// sorted sequences. Examples
	/// let mut v = [-5, 4, 1, -3, 2];
	///
	/// v.sort_unstable();
	/// assert!(v == [-5, -3, 1, 2, 4]);
	pub fn sort_unstable(&mut self)
	where T: Ord {
		self.inner.sort_unstable();
	}

	/// Sorts the slice with a comparator function, but might not preserve the
	/// order of equal elements. This sort is unstable (i.e., may reorder equal
	/// elements), in-place (i.e., does not allocate), and O(n * log(n))
	/// worst-case. 
	/// 
	/// 
	/// The comparator function must define a total ordering for
	/// the elements in the slice. If the ordering is not total, the order of
	/// the elements is unspecified. An order is a total order if it is (for all
	/// a, b and c): total and antisymmetric: exactly one of a < b, a == b or a
	/// > b is true, and transitive, a < b and b < c implies a < c. The same
	/// must hold for both == and >. For example, while f64 doesn't implement
	/// Ord because NaN != NaN, we can use partial_cmp as our sort function when
	/// we know the slice doesn't contain a NaN.
	///
	/// ```
	/// let mut floats = [5f64, 4.0, 1.0, 3.0, 2.0];
	/// floats.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
	/// assert_eq!(floats, [1.0, 2.0, 3.0, 4.0,
	/// 5.0]);
	/// ```
	///
	/// Current implementation
	/// The current algorithm is based on pattern-defeating quicksort  by Orson
	/// Peters, which combines the fast average case of randomized quicksort
	/// with the fast worst case of heapsort, while achieving linear time on
	/// slices with certain patterns. It uses some randomization to avoid
	/// degenerate cases, but with a fixed seed to always provide deterministic
	/// behavior. It is typically faster than stable sorting, except in a few
	/// special cases, e.g., when the slice consists of several concatenated
	/// sorted sequences. Examples
	/// let mut v = [5, 4, 1, 3, 2];
	/// v.sort_unstable_by(|a, b| a.cmp(b));
	/// assert!(v == [1, 2, 3, 4, 5]);
	///
	/// /// reverse sorting
	/// v.sort_unstable_by(|a, b| b.cmp(a));
	/// assert!(v == [5, 4, 3, 2, 1]);
	pub fn sort_unstable_by<F>(&mut self, compare:F)
	where F: FnMut(&T, &T) -> std::cmp::Ordering {
		self.inner
			.sort_unstable_by(compare);
	}

	/// Sorts the slice with a key extraction function, but might not preserve
	/// the order of equal elements. This sort is unstable (i.e., may reorder
	/// equal elements), in-place (i.e., does not allocate), and O(m * n *
	/// log(n)) worst-case, where the key function is O(m).
	/// Current implementation
	/// The current algorithm is based on pattern-defeating quicksort  by Orson
	/// Peters, which combines the fast average case of randomized quicksort
	/// with the fast worst case of heapsort, while achieving linear time on
	/// slices with certain patterns. It uses some randomization to avoid
	/// degenerate cases, but with a fixed seed to always provide deterministic
	/// behavior. Due to its key calling strategy, sort_unstable_by_key is
	/// likely to be slower than sort_by_cached_key in cases where the key
	/// function is expensive. Examples
	/// let mut v = [-5i32, 4, 1, -3, 2];
	///
	/// v.sort_unstable_by_key(|k| k.abs());
	/// assert!(v == [1, 2, -3, 4, -5]);
	pub fn sort_unstable_by_key<K, F>(&mut self, f:F)
	where
		F: FnMut(&T) -> K,
		K: Ord,
	{
		self.inner
			.sort_unstable_by_key(f);
	}

	pub fn append(&mut self, other:&mut Self)
	where T: Clone {
		for value in other.drain(0..other.len()) {
			self.insert(value);
		}
	}

	pub fn extend_from_slice(&mut self, other:&[T])
	where T: Clone {
		self.inner
			.extend_from_slice(other);
	}

	pub fn consume_from(
		&mut self,
		other:impl IntoIterator<Item = T>,
	) -> Vec<T> {
		other
			.into_iter()
			.filter_map(|value| self.insert(value))
			.collect()
	}
}

impl<T:Eq + Hash> IntoIterator for IndexedSet<T> {
	type Item = T;
	type IntoIter = std::vec::IntoIter<T>;

	fn into_iter(self) -> Self::IntoIter { self.inner.into_iter() }
}

impl<'a, T:Eq + Hash> IntoIterator for &'a IndexedSet<T> {
	type Item = &'a T;
	type IntoIter = std::slice::Iter<'a, T>;

	fn into_iter(self) -> Self::IntoIter { self.inner.iter() }
}

impl<'a, T:Eq + Hash> IntoIterator for &'a mut IndexedSet<T> {
	type Item = &'a mut T;
	type IntoIter = std::slice::IterMut<'a, T>;

	fn into_iter(self) -> Self::IntoIter { self.inner.iter_mut() }
}

impl<T> Default for IndexedSet<T>
where T: Hash + PartialEq + Eq
{
	fn default() -> Self { Self::new() }
}

impl<T> std::ops::Index<usize> for IndexedSet<T>
where T: Hash + PartialEq + Eq
{
	type Output = T;

	fn index(&self, index:usize) -> &Self::Output { &self.inner[index] }
}

impl<T> std::ops::Index<usize> for &IndexedSet<T>
where T: Hash + PartialEq + Eq
{
	type Output = T;

	fn index(&self, index:usize) -> &Self::Output { &self.inner[index] }
}

impl<T> std::ops::Index<usize> for &mut IndexedSet<T>
where T: Hash + PartialEq + Eq
{
	type Output = T;

	fn index(&self, index:usize) -> &Self::Output { &self.inner[index] }
}

impl<T> std::ops::Index<&T> for IndexedSet<T>
where T: Hash + PartialEq + Eq
{
	type Output = T;

	fn index(&self, key:&T) -> &Self::Output {
		return self
			.inner
			.iter()
			.find(|&value| value == key)
			.unwrap();
	}
}

impl<T> std::ops::Index<&T> for &IndexedSet<T>
where T: Hash + PartialEq + Eq
{
	type Output = T;

	fn index(&self, key:&T) -> &Self::Output {
		return self
			.inner
			.iter()
			.find(|&value| value == key)
			.unwrap();
	}
}

impl<T> std::ops::Index<&T> for &mut IndexedSet<T>
where T: Hash + PartialEq + Eq
{
	type Output = T;

	fn index(&self, key:&T) -> &Self::Output {
		return self
			.inner
			.iter()
			.find(|&value| value == key)
			.unwrap();
	}
}

impl<T> FromIterator<T> for IndexedSet<T>
where T: Hash + PartialEq + Eq
{
	fn from_iter<I:IntoIterator<Item = T>>(iter:I) -> Self {
		let mut result = IndexedSet::new();
		result.extend(iter);
		result
	}
}

impl<T> Extend<T> for IndexedSet<T>
where T: Hash + PartialEq + Eq
{
	fn extend<I:IntoIterator<Item = T>>(&mut self, iter:I) {
		for value in iter {
			self.insert(value);
		}
	}
}

impl<'a, T> IterGenerator<'a, &'a T> for IndexedSet<T>
where T: Hash + PartialEq + Eq
{
	fn iterate(&'a self) -> impl ExactSizeIterator<Item = &'a T> {
		self.inner.iter()
	}
}

impl<'a, T> IterGenerator<'a, &'a T> for &'a IndexedSet<T>
where T: Hash + PartialEq + Eq
{
	fn iterate(&'a self) -> impl ExactSizeIterator<Item = &'a T> {
		self.inner.iter()
	}
}

impl<'a, T> IterGenerator<'a, &'a T> for &'a mut IndexedSet<T>
where T: Hash + PartialEq + Eq
{
	fn iterate(&'a self) -> impl ExactSizeIterator<Item = &'a T> {
		self.inner.iter()
	}
}

// Inserting a new element into an empty IndexedSet returns None and increases
// the length of the set by 1
#[test]
fn test_insert_new_element_into_empty_set() {
	let mut set = IndexedSet::new();
	assert_eq!(set.insert(1), None);
	assert_eq!(set.len(), 1);
}

// Inserting an existing element into an IndexedSet returns Some(value) and does
// not increase the length of the set
#[test]
fn test_insert_existing_element_into_set() {
	let mut set = IndexedSet::new();
	set.insert(1);
	assert_eq!(
		set.insert(1),
		Some(1)
	);
	assert_eq!(set.len(), 1);
}

// Removing an existing element from an IndexedSet returns Some(index) and
// decreases the length of the set by 1
#[test]
fn test_remove_existing_element_from_set() {
	let mut set = IndexedSet::new();
	set.insert(1);
	assert_eq!(
		set.remove(&1),
		Some(0)
	);
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
	set.insert(1);
	set.insert(2);
	set.insert(3);
	let mut iter = set.iter();
	assert_eq!(
		iter.next(),
		Some(&1)
	);
	assert_eq!(
		iter.next(),
		Some(&2)
	);
	assert_eq!(
		iter.next(),
		Some(&3)
	);
	assert_eq!(iter.next(), None);
}

// Iterating over an IndexedSet using iter_mut() returns a
// std::slice::IterMut<T> with the same elements as the set
#[test]
fn test_iterate_over_set_using_iter_mut() {
	let mut set = IndexedSet::new();
	set.insert(1);
	set.insert(2);
	set.insert(3);
	let mut iter = set.iter_mut();
	assert_eq!(
		iter.next(),
		Some(&mut 1)
	);
	assert_eq!(
		iter.next(),
		Some(&mut 2)
	);
	assert_eq!(
		iter.next(),
		Some(&mut 3)
	);
	assert_eq!(iter.next(), None);
}

#[test]
fn test_from_iter() {
	let iter:Vec<&str> = vec!["one", "two", "three"];
	let _:IndexedSet<&str> = iter.into_iter().collect();
}
