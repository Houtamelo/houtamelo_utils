use std::collections::{HashMap, HashSet};

pub trait RemoveMany<T> {
	fn remove_many(&mut self, to_remove: impl Iterator<Item = T>);
}

impl<'a, T> RemoveMany<&'a T> for Vec<T> 
	where T: PartialEq {
	fn remove_many(&mut self, to_remove: impl Iterator<Item = &'a T>) {
		to_remove.for_each(|remove_me| {
			self.retain(|item| item != remove_me);
		});
	}
}

impl<'a, T> RemoveMany<&'a T> for HashSet<T> 
	where T: Eq + std::hash::Hash {
	fn remove_many(&mut self, to_remove: impl Iterator<Item = &'a T>) {
		to_remove.for_each(|remove_me| {
			self.remove(remove_me);
		});
	}
}

impl<'a, TKey, TValue> RemoveMany<&'a TKey> for HashMap<TKey, TValue>
	where TKey: Eq + std::hash::Hash {
	fn remove_many(&mut self, to_remove: impl Iterator<Item = &'a TKey>) {
		to_remove.for_each(|remove_me| {
			self.remove(remove_me);
		});
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_vec() {
		let mut vec = vec![1, 2, 3, 4, 5];
		vec.remove_many(vec![1, 3, 5].iter());
		assert_eq!(vec, vec![2, 4]);
	}

	#[test]
	fn test_hash_set() {
		let mut set = [1, 2, 3, 4, 5].iter().cloned().collect::<HashSet<_>>();
		set.remove_many(vec![&1, &3, &5].into_iter());
		assert_eq!(set, [2, 4].iter().cloned().collect::<HashSet<_>>());
	}

	#[test]
	fn test_hash_map() {
		let mut map = [(1, "one"), (2, "two"), (3, "three"), (4, "four"), (5, "five")]
			.iter().cloned().collect::<HashMap<_, _>>();
		map.remove_many(vec![&1, &3, &5].into_iter());
		assert_eq!(map, [(2, "two"), (4, "four")].iter().cloned().collect::<HashMap<_, _>>());
	}
}