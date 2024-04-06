use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

use rand::Rng;

pub trait GetRandom<'a, T> {
	fn get_random(&'a self, rng: &mut impl Rng) -> Option<T>;
}

impl<'a, T> GetRandom<'a, &'a T> for Vec<T> {
	fn get_random(&'a self, rng: &mut impl Rng) -> Option<&'a T> {
		if self.is_empty() {
			return None;
		}

		let index = rng.gen_range(0..self.len());
		self.get(index)
	}
}

impl<'a, T> GetRandom<'a, &'a T> for &[T] {
	fn get_random(&'a self, rng: &mut impl Rng) -> Option<&'a T> {
		if self.is_empty() {
			return None;
		}

		let index = rng.gen_range(0..self.len());
		Some(&self[index])
	}
}

impl<'a, T, const SIZE: usize> GetRandom<'a, &'a T> for [T; SIZE] {
	fn get_random(&'a self, rng: &mut impl Rng) -> Option<&'a T> {
		if self.is_empty() {
			return None;
		}

		let index = rng.gen_range(0..SIZE);
		Some(&self[index])
	}
}

impl<'a, T> GetRandom<'a, &'a T> for VecDeque<T> {
	fn get_random(&'a self, rng: &mut impl Rng) -> Option<&'a T> {
		if self.is_empty() {
			return None;
		}

		let index = rng.gen_range(0..self.len());
		self.get(index)
	}
}

impl<'a, T: PartialEq + Eq + Hash> GetRandom<'a, &'a T> for HashSet<T> {
	fn get_random(&'a self, rng: &mut impl Rng) -> Option<&'a T> {
		if self.is_empty() {
			return None;
		}

		let index = rng.gen_range(0..self.len());
		self.iter().nth(index)
	}
}

impl<'a, TKey: PartialEq + Eq + Hash, TVal> GetRandom<'a, (&'a TKey, &'a TVal)> for HashMap<TKey, TVal> {
	fn get_random(&'a self, rng: &mut impl Rng) -> Option<(&'a TKey, &'a TVal)> {
		if self.is_empty() {
			return None;
		}

		let index = rng.gen_range(0..self.len());
		self.iter().nth(index)
	}
}

pub trait GetRandomMut<'a, T> {
	fn get_random_mut(&'a mut self, rng: &mut impl Rng) -> Option<T>;
}

impl<'a, T> GetRandomMut<'a, &'a mut T> for Vec<T> {
	fn get_random_mut(&'a mut self, rng: &mut impl Rng) -> Option<&'a mut T> {
		if self.is_empty() {
			return None;
		}

		let index = rng.gen_range(0..self.len());
		self.get_mut(index)
	}
}

impl<'a, T> GetRandomMut<'a, &'a mut T> for &'a mut [T] {
	fn get_random_mut(&'a mut self, rng: &mut impl Rng) -> Option<&'a mut T> {
		if self.is_empty() {
			return None;
		}

		let index = rng.gen_range(0..self.len());
		Some(&mut self[index])
	}
}

impl<'a, T, const SIZE: usize> GetRandomMut<'a, &'a mut T> for [T; SIZE] {
	fn get_random_mut(&'a mut self, rng: &mut impl Rng) -> Option<&'a mut T> {
		if self.is_empty() {
			return None;
		}

		let index = rng.gen_range(0..SIZE);
		Some(&mut self[index])
	}
}

impl<'a, T> GetRandomMut<'a, &'a mut T> for VecDeque<T> {
	fn get_random_mut(&'a mut self, rng: &mut impl Rng) -> Option<&'a mut T> {
		if self.is_empty() {
			return None;
		}

		let index = rng.gen_range(0..self.len());
		self.get_mut(index)
	}
}

impl<'a, TKey: PartialEq + Eq + Hash, TVal> GetRandomMut<'a, (&'a TKey, &'a mut TVal)> for HashMap<TKey, TVal> {
	fn get_random_mut(&'a mut self, rng: &mut impl Rng) -> Option<(&'a TKey, &'a mut TVal)> {
		if self.is_empty() {
			return None;
		}

		let index = rng.gen_range(0..self.len());
		self.iter_mut().nth(index)
	}
}