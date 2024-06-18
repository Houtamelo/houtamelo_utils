use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(bound(serialize = "T: Serialize, [T; COUNT]: Serialize"))]
#[serde(bound(deserialize = "T: DeserializeOwned, [T; COUNT]: DeserializeOwned"))]
pub struct CountOrMore<const COUNT: usize, T> {
	fixed: [T; COUNT],
	dynamic: Vec<T>,
}

impl<const COUNT: usize, T> CountOrMore<COUNT, T> {
	#[must_use]
	pub fn new(fixed: [T; COUNT], dynamic: Vec<T>) -> Self {
		return CountOrMore {
			fixed,
			dynamic,
		};
	}
	
	pub fn iter(&self) -> impl Iterator<Item = &T> {
		return self.fixed
		           .iter()
		           .chain(self.dynamic.iter());
	}
	
	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
		return self.fixed
		           .iter_mut()
		           .chain(self.dynamic.iter_mut());
	}

	#[must_use]
	pub fn get(&self, index: usize) -> Option<&T> {
		if index < COUNT {
			return Some(&self.fixed[index]);
		} else {
			return self.dynamic.get(index - COUNT);
		}
	}

	#[must_use]
	pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
		if index < COUNT {
			return Some(&mut self.fixed[index]);
		} else {
			return self.dynamic.get_mut(index - COUNT);
		}
	}
	
	pub fn clear_extras(&mut self) {
		self.dynamic.clear();
	}

	pub fn push(&mut self, value: T) {
		self.dynamic.push(value);
	}

	pub fn append(&mut self, other: &mut Vec<T>) {
		self.dynamic.append(other);
	}

	pub fn extend(&mut self, other: impl IntoIterator<Item = T>) {
		self.dynamic.extend(other);
	}
	
	pub fn pop(&mut self) -> Option<T> {
		return self.dynamic.pop();
	}

	/// ### Returns
	/// The value trying to be inserted, if it was not inserted. (Due to being out of bounds)
	#[must_use]
	pub fn insert(&mut self, index: usize, value: T) -> Option<T> {
		if index >= self.len() {
			return Some(value);
		}
		
		if index < COUNT { 
			for fixed_index in (index..(COUNT - 1)).rev() {
				self.fixed.swap(fixed_index, fixed_index + 1);
			}
			
			let fixed_to_move_to_dyn = std::mem::replace(&mut self.fixed[index], value);
			self.dynamic.insert(0, fixed_to_move_to_dyn);
		} else {
			self.dynamic.insert(index - COUNT, value);
		}
		
		return None;
	}
	
	pub fn remove(&mut self, index: usize) -> Option<T> {
		if index >= self.len() || self.dynamic.len() == 0 {
			return None;
		}
		
		return if index < COUNT {
			for fixed_index in index..(COUNT - 1) {
				self.fixed.swap(fixed_index, fixed_index + 1);
			}

			let first_dyn = self.dynamic.remove(0);
			let result = std::mem::replace(&mut self.fixed[COUNT - 1], first_dyn);
			Some(result)
		} else {
			Some(self.dynamic.remove(index - COUNT))
		};
	}
	
	pub fn swap_remove(&mut self, index: usize) -> Option<T> {
		if index >= self.len() || self.dynamic.len() == 0 {
			return None;
		}
		
		return if index < COUNT {
			let result = std::mem::replace(&mut self.fixed[index], self.dynamic.remove(self.dynamic.len() - 1));
			Some(result)
		} else {
			Some(self.dynamic.swap_remove(index - COUNT))
		};
	}

	pub fn shrink_to_fit(&mut self) {
		self.dynamic.shrink_to_fit();
	}

	#[must_use]
	pub fn len(&self) -> usize {
		return COUNT + self.dynamic.len();
	}

	#[must_use]
	pub fn capacity(&self) -> usize {
		return COUNT + self.dynamic.capacity();
	}
	
	pub fn push_within_capacity(&mut self, value: T) -> Result<(), T> {
		return self.dynamic.push_within_capacity(value);
	}

	#[must_use]
	pub fn contains(&self, value: &T) -> bool where T: Ord {
		return self.fixed.contains(value) || self.dynamic.contains(value);
	}
	
	pub fn reserve(&mut self, additional: usize) {
		return self.dynamic.reserve(additional);
	}

	pub fn reserve_exact(&mut self, additional: usize) {
		return self.dynamic.reserve_exact(additional);
	}

	pub fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> {
		return self.dynamic.try_reserve(additional);
	}

	pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> {
		return self.dynamic.try_reserve_exact(additional);
	}

	pub fn sort_by_key<K, F>(&mut self, mut f: F)
	                         where F: FnMut(&T) -> K, 
	                               K: Ord {
		let mut sorted_idx_var: Vec<_> = 
			self.iter()
				.enumerate()
				.collect();
		
		sorted_idx_var.sort_by_key(|(_, x)| f(x));
		
		let mut old_idxs: Vec<usize> =
			sorted_idx_var.into_iter()
			              .map(|(old_index, _)| old_index)
			              .collect();

		#[allow(unused_must_use)] 
		for i in 0..old_idxs.len() {
			let idx = old_idxs[i];
			self.swap(i, idx);
			
			for old in old_idxs.iter_mut().skip(i) {
				if *old == i {
					*old = idx;
					break;
				}
			}
		}
	}
	
	#[must_use]
	pub fn into_sorted_by_key<K, F>(self, mut f: F) 
		-> Self where F: FnMut(&T) -> K, 
		              K: Ord {
		let mut combined: Vec<T> = 
			self.into_iter()
				.collect();
		
		combined.sort_by_key(|x| f(x));
		
		let fixed: [T; COUNT] = [(); COUNT].map(|_| combined.remove(0));
		let dynamic = combined;
		return CountOrMore::new(fixed, dynamic);
	}

	pub fn shrink_to(&mut self, min_capacity: usize) {
		if min_capacity > COUNT {
			self.dynamic.shrink_to(min_capacity - COUNT);
		}
	}

	/// If false, no alterations were done.
	#[must_use]
	pub fn try_truncate(&mut self, len: usize) -> bool {
		if len >= COUNT {
			self.dynamic.truncate(len - COUNT);
			return true;
		} else {
			return false;
		}
	}
	
	pub fn swap(&mut self, a_idx: usize, b_idx: usize) {
		if a_idx == b_idx {
			return;
		}
		
		let len = self.len();
		if a_idx >= len || b_idx >= len {
			return;
		}
		
		if a_idx < COUNT && b_idx < COUNT {
			self.fixed.swap(a_idx, b_idx);
		} else if a_idx < COUNT && b_idx >= COUNT {
			let b_idx_dyn = b_idx - COUNT;
			let a_var = std::mem::replace(&mut self.fixed[a_idx], self.dynamic.remove(b_idx_dyn));
			self.dynamic.insert(b_idx_dyn, a_var);
		} else if a_idx >= COUNT && b_idx < COUNT {
			let a_idx_dyn = a_idx - COUNT;
			let b_var = b_idx;
			let b = std::mem::replace(&mut self.fixed[b_var], self.dynamic.remove(a_idx_dyn));
			self.dynamic.insert(a_idx_dyn, b);
		} else {
			let a_idx_dyn = a_idx - COUNT;
			let b_idx_dyn = b_idx - COUNT;
			self.dynamic.swap(a_idx_dyn, b_idx_dyn);
		}
	}
	
	/// If false, no alterations were done.
	#[must_use]
	pub fn try_retain<F>(&mut self, mut f: F) -> bool 
		where F: FnMut(&T) -> bool {
		let retained =
			self.iter()
				.enumerate()
			    .filter_map(|(index, x)| f(x).then_some(index)) 
			    .collect::<Vec<_>>();

		let retained_len = retained.len();
		
		if retained_len < COUNT || retained_len == self.len() {
			return false;
		}  
		
		let truncate_count = retained_len - COUNT;
		
		retained.into_iter()
		        .enumerate()
		        .for_each(|(origin_idx, retained_idx)| self.swap(origin_idx, retained_idx));
		
		self.dynamic.truncate(truncate_count);
		return true;
	}
}

impl<const COUNT: usize, T> IntoIterator for CountOrMore<COUNT, T> {
	type Item = T;
	type IntoIter = std::iter::Chain<std::array::IntoIter<T, COUNT>, std::vec::IntoIter<T>>;

	fn into_iter(self) -> Self::IntoIter {
		return self.fixed
		           .into_iter()
		           .chain(self.dynamic.into_iter());
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new() {
		let fixed = [1, 2, 3, 4, 5];
		let dynamic = vec![6, 7, 8, 9, 10];
		let count_or_more = CountOrMore::new(fixed.clone(), dynamic.clone());

		assert_eq!(count_or_more.fixed, fixed);
		assert_eq!(count_or_more.dynamic, dynamic);
	}

	#[test]
	fn test_iter() {
		let fixed = [10, 20, 30, 40, 50];
		let dynamic = vec![60, 70, 80, 90, 100];
		let count_or_more = CountOrMore::new(fixed.clone(), dynamic.clone());
		
		let expect = fixed.iter().chain(dynamic.iter());

		for (i, j) in count_or_more.iter().zip(expect) {
			assert_eq!(i, j);
		}
	}

	#[test]
	fn test_iter_mut() {
		let fixed = [100, 200, 300, 400, 500];
		let dynamic = vec![600, 700, 800, 900, 1000];
		let mut count_or_more = CountOrMore::new(fixed, dynamic.clone());

		for i in count_or_more.iter_mut() {
			*i += 7;
		}
		
		let expect = vec![107, 207, 307, 407, 507, 607, 707, 807, 907, 1007];
		for (i, j) in count_or_more.iter().zip(expect) {
			assert_eq!(i, &j);
		}
	}

	#[test]
	fn test_get() {
		let fixed = [1, 2, 3, 4, 5];
		let dynamic = vec![6, 7, 8, 9, 10];
		let count_or_more = CountOrMore::new(fixed, dynamic.clone());

		assert_eq!(count_or_more.get(0), Some(&1));
		assert_eq!(count_or_more.get(4), Some(&5));
		assert_eq!(count_or_more.get(5), Some(&6));
		assert_eq!(count_or_more.get(9), Some(&10));
		assert_eq!(count_or_more.get(10), None);
	}

	#[test]
	fn test_get_mut() {
		let fixed = [1, 2, 3, 4, 5];
		let dynamic = vec![6, 7, 8, 9, 10];
		let mut count_or_more = CountOrMore::new(fixed, dynamic.clone());

		if let Some(value) = count_or_more.get_mut(0) {
			*value += 1;
		}
		assert_eq!(count_or_more.get(0), Some(&2));

		if let Some(value) = count_or_more.get_mut(9) {
			*value += 1;
		}
		assert_eq!(count_or_more.get(9), Some(&11));
	}

	#[test]
	fn test_clear_extras() {
		let fixed = [1, 2, 3, 4, 5];
		let dynamic = vec![6, 7, 8, 9, 10];
		let mut count_or_more = CountOrMore::new(fixed, dynamic.clone());

		count_or_more.clear_extras();
		assert_eq!(count_or_more.dynamic, Vec::new());
	}

	#[test]
	fn test_push() {
		let fixed = [1, 2, 3, 4, 5];
		let dynamic = vec![6, 7, 8, 9, 10];
		let mut count_or_more = CountOrMore::new(fixed, dynamic.clone());

		count_or_more.push(11);
		assert_eq!(count_or_more.get(10), Some(&11));
		assert_eq!(count_or_more.dynamic, vec![6, 7, 8, 9, 10, 11]);
		assert_eq!(count_or_more.fixed, [1, 2, 3, 4, 5]);
	}

	#[test]
	fn test_append() {
		let fixed = [1, 2, 3, 4, 5];
		let dynamic = vec![6, 7, 8, 9, 10];
		let mut count_or_more = CountOrMore::new(fixed, dynamic.clone());

		let mut other = vec![11, 12, 13];
		count_or_more.append(&mut other);
		assert_eq!(count_or_more.dynamic, vec![6, 7, 8, 9, 10, 11, 12, 13]);
	}

	#[test]
	fn test_extend() {
		let fixed = [1, 2, 3, 4, 5];
		let dynamic = vec![6, 7, 8, 9, 10];
		let mut count_or_more = CountOrMore::new(fixed, dynamic.clone());

		let other = vec![11, 12, 13];
		count_or_more.extend(other);
		assert_eq!(count_or_more.get(10), Some(&11));
		assert_eq!(count_or_more.get(11), Some(&12));
		assert_eq!(count_or_more.get(12), Some(&13));
	}

	#[test]
	fn test_pop() {
		let fixed = [5, 3, 1, 4, 2];
		let dynamic = vec![10, 8, 6, 9, 7];
		let mut count_or_more = CountOrMore::new(fixed, dynamic.clone());

		assert_eq!(count_or_more.pop(), Some(7));
		assert_eq!(count_or_more.pop(), Some(9));
		assert_eq!(count_or_more.pop(), Some(6));
		assert_eq!(count_or_more.pop(), Some(8));
		assert_eq!(count_or_more.pop(), Some(10));
		assert_eq!(count_or_more.pop(), None);
	}

	#[test]
	fn test_insert() {
		let fixed = [5, 3, 1, 4, 2];
		let dynamic = vec![10, 8, 6, 9, 7];
		let mut count_or_more = CountOrMore::new(fixed, dynamic.clone());

		assert_eq!(count_or_more.insert(3, 11), None);
		assert_eq!(count_or_more.get(3), Some(&11));
		assert_eq!(count_or_more.fixed, [5, 3, 1, 11, 4]);
		assert_eq!(count_or_more.dynamic, vec![2, 10, 8, 6, 9, 7]);

		assert_eq!(count_or_more.insert(5, 12), None);
		assert_eq!(count_or_more.get(5), Some(&12));
		
		assert_eq!(count_or_more.insert(20, 13), Some(13));
	}

	#[test]
	fn test_remove() {
		let fixed = [5, 3, 1, 4, 2];
		let dynamic = vec![10, 8, 6, 9, 7];
		let mut count_or_more = CountOrMore::new(fixed, dynamic.clone());

		assert_eq!(count_or_more.remove(2), Some(1));
		assert_eq!(count_or_more.fixed, [5, 3, 4, 2, 10]);
		assert_eq!(count_or_more.dynamic, vec![8, 6, 9, 7]);
		
		assert_eq!(count_or_more.remove(4), Some(10));
		assert_eq!(count_or_more.remove(10), None);
		
		assert_eq!(count_or_more.remove(0), Some(5));
		assert_eq!(count_or_more.fixed, [3, 4, 2, 8, 6]);
		
		assert_eq!(count_or_more.remove(4), Some(6));
		assert_eq!(count_or_more.remove(1), Some(4));
		assert_eq!(count_or_more.remove(1), None);
	}

	#[test]
	fn test_swap_remove() {
		let fixed = [5, 3, 1, 4, 2];
		let dynamic = vec![10, 8, 6, 9, 7];
		let mut count_or_more = CountOrMore::new(fixed, dynamic.clone());

		assert_eq!(count_or_more.swap_remove(2), Some(1));
		assert_eq!(count_or_more.get(2), Some(&7));
	}

	#[test]
	fn test_shrink_to_fit() {
		let fixed = [1, 2, 3, 4, 5];
		let mut dynamic = Vec::with_capacity(20);
		dynamic.append(&mut vec![6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
		
		let mut count_or_more = CountOrMore::new(fixed, dynamic.clone());
		count_or_more.shrink_to_fit();
		assert_eq!(count_or_more.capacity(), 15);
	}

	#[test]
	fn test_contains() {
		let fixed = [5, 3, 1, 4, 2];
		let dynamic = vec![10, 8, 6, 9, 7];
		let count_or_more = CountOrMore::new(fixed, dynamic.clone());

		assert!(count_or_more.contains(&1));
		assert!(!count_or_more.contains(&11));
	}

	#[test]
	fn test_reserve() {
		let fixed = [1, 2, 3, 4, 5];
		let dynamic = vec![6, 7, 8, 9, 10];
		let mut count_or_more = CountOrMore::new(fixed, dynamic.clone());

		count_or_more.reserve(10);
		assert!(count_or_more.capacity() >= 15);
	}

	#[test]
	fn test_reserve_exact() {
		let fixed = [1, 2, 3, 4, 5];
		let dynamic = vec![6, 7, 8, 9, 10];
		let mut count_or_more = CountOrMore::new(fixed, dynamic.clone());

		count_or_more.reserve_exact(10);
		assert_eq!(count_or_more.capacity(), 20);
	}

	#[test]
	fn test_try_reserve() {
		let fixed = [1, 2, 3, 4, 5];
		let dynamic = vec![6, 7, 8, 9, 10];
		let mut count_or_more = CountOrMore::new(fixed, dynamic.clone());

		assert!(count_or_more.try_reserve(10).is_ok());
	}

	#[test]
	fn test_try_reserve_exact() {
		let fixed = [1, 2, 3, 4, 5];
		let dynamic = vec![6, 7, 8, 9, 10];
		let mut count_or_more = CountOrMore::new(fixed, dynamic.clone());

		assert!(count_or_more.try_reserve_exact(10).is_ok());
	}

	#[test]
	fn test_sort_by_key() {
		let fixed = [5, 3, 1];
		let dynamic = vec![2, 4];
		let mut count_or_more = CountOrMore::new(fixed, dynamic.clone());

		count_or_more.sort_by_key(|&x| x);
		assert_eq!(count_or_more.fixed, [1, 2, 3]);
		assert_eq!(count_or_more.dynamic, vec![4, 5]);
		assert_eq!(count_or_more.into_iter().collect::<Vec<_>>(), vec![1, 2, 3, 4, 5]);

		let fixed = [5, 3, 1, 20, 10, 25];
		let dynamic = vec![2, 40, 80, 36, 0];
		let mut count_or_more = CountOrMore::new(fixed, dynamic.clone());

		count_or_more.sort_by_key(|&x| x);
		assert_eq!(count_or_more.fixed, [0, 1, 2, 3, 5, 10]);
		assert_eq!(count_or_more.dynamic, vec![20, 25, 36, 40, 80]);
		assert_eq!(count_or_more.into_iter().collect::<Vec<_>>(), vec![0, 1, 2, 3, 5, 10, 20, 25, 36, 40, 80]);

		let fixed = [5, 3, 1, 50, 6, 8];
		let dynamic = vec![2, 50, 8, 5, 6];
		let mut count_or_more = CountOrMore::new(fixed, dynamic.clone());

		count_or_more.sort_by_key(|&x| x);
		assert_eq!(count_or_more.fixed, [1, 2, 3, 5, 5, 6]);
		assert_eq!(count_or_more.dynamic, vec![6, 8, 8, 50, 50]);
		assert_eq!(count_or_more.into_iter().collect::<Vec<_>>(), vec![1, 2, 3, 5, 5, 6, 6, 8, 8, 50, 50]);
	}

	#[test]
	fn test_into_sorted_by_key() {
		let fixed = [5, 3, 1, 4, 2];
		let dynamic = vec![10, 8, 6, 9, 7];
		let count_or_more = CountOrMore::new(fixed, dynamic.clone());

		let sorted = count_or_more.into_sorted_by_key(|&x| x);
		assert_eq!(sorted.iter().collect::<Vec<_>>(), vec![&1, &2, &3, &4, &5, &6, &7, &8, &9, &10]);
	}
	
	#[test]
	fn test_swap() {
		let fixed = [5, 3, 1, 4, 2];
		let dynamic = vec![10, 8, 6, 9, 7];
		let mut count_or_more = CountOrMore::new(fixed, dynamic.clone());
		
		count_or_more.swap(0, 9);
		assert_eq!(count_or_more.get(0), Some(&7));
		assert_eq!(count_or_more.get(9), Some(&5));
		
		count_or_more.swap(4, 5);
		assert_eq!(count_or_more.get(4), Some(&10));
		assert_eq!(count_or_more.get(5), Some(&2));
	}
	
	#[test]
	fn test_try_retain() {
		let fixed = [5, 3, 12, 4, 9];
		let dynamic = vec![10, 2, 6, 5, 7];
		let mut count_or_more = CountOrMore::new(fixed, dynamic.clone());
		
		assert_eq!(true, count_or_more.try_retain(|x| *x < 6));
		assert_eq!(count_or_more.fixed, [5, 3, 4, 2, 5]);
		assert_eq!(count_or_more.dynamic, vec![]);
		
		assert_eq!(false, count_or_more.try_retain(|x| *x < 6));
	}
}