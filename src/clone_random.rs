use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

use rand::Rng;

pub trait CloneRandom<T> {
	fn clone_random(&self, rng: &mut impl Rng) -> Option<T>;
}

impl<T: Clone> CloneRandom<T> for Vec<T> {
	fn clone_random(&self, rng: &mut impl Rng) -> Option<T> {
		if self.is_empty() {
			return None;
		}

		let index = rng.gen_range(0..self.len());
		Some(self[index].clone())
	}
}

impl<T: Clone> CloneRandom<T> for &[T] {
	fn clone_random(&self, rng: &mut impl Rng) -> Option<T> {
		if self.is_empty() {
			return None;
		}

		let index = rng.gen_range(0..self.len());
		Some(self[index].clone())
	}
}

impl<T: Clone, const SIZE: usize> CloneRandom<T> for [T; SIZE] {
	fn clone_random(&self, rng: &mut impl Rng) -> Option<T> {
		if self.is_empty() {
			return None;
		}

		let index = rng.gen_range(0..SIZE);
		Some(self[index].clone())
	}
}

impl<T: Clone> CloneRandom<T> for VecDeque<T> {
	fn clone_random(&self, rng: &mut impl Rng) -> Option<T> {
		if self.is_empty() {
			return None;
		}

		let index = rng.gen_range(0..self.len());
		Some(self[index].clone())
	}
}

impl<T: Clone + PartialEq + Eq + Hash> CloneRandom<T> for HashSet<T> {
	fn clone_random(&self, rng: &mut impl Rng) -> Option<T> {
		if self.is_empty() {
			return None;
		}

		let index = rng.gen_range(0..self.len());
		self.iter().nth(index).cloned()
	}
}

impl<TKey: PartialEq + Eq + Hash + Clone, TVal: Clone> CloneRandom<(TKey, TVal)> for HashMap<TKey, TVal> {
	fn clone_random(&self, rng: &mut impl Rng) -> Option<(TKey, TVal)> {
		if self.is_empty() {
			return None;
		}

		let index = rng.gen_range(0..self.len());
		self.iter().nth(index).map(|(key, val)| (key.clone(), val.clone()))
	}
}
