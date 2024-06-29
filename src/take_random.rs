use rand::Rng;

use crate::prelude::*;

pub trait TakeRandom<T> {
	fn take_random(&mut self, rng:&mut impl Rng) -> Option<T>;
}

impl<T> TakeRandom<T> for Vec<T> {
	fn take_random(&mut self, rng:&mut impl Rng) -> Option<T> {
		if self.is_empty() {
			return None;
		}

		let index = rng.gen_range(0..self.len());
		Some(self.remove(index))
	}
}

impl<T> TakeRandom<T> for VecDeque<T> {
	fn take_random(&mut self, rng:&mut impl Rng) -> Option<T> {
		if self.is_empty() {
			return None;
		}

		let index = rng.gen_range(0..self.len());
		self.remove(index)
	}
}

impl<T:PartialEq + Eq + Hash + Clone> TakeRandom<T> for HashSet<T> {
	fn take_random(&mut self, rng:&mut impl Rng) -> Option<T> {
		if self.is_empty() {
			return None;
		}

		let index = rng.gen_range(0..self.len());
		let opti = self
			.iter()
			.nth(index)
			.cloned();

		opti.inspect(|item| {
			self.remove(item);
		})
	}
}

impl<TKey:PartialEq + Eq + Hash + Clone, TVal> TakeRandom<(TKey, TVal)>
	for HashMap<TKey, TVal>
{
	fn take_random(&mut self, rng:&mut impl Rng) -> Option<(TKey, TVal)> {
		if self.is_empty() {
			return None;
		}

		let index = rng.gen_range(0..self.len());
		let key = self
			.keys()
			.nth(index)
			.cloned();

		key.and_then(|key| {
			self.remove(&key)
				.map(|val| (key, val))
		})
	}
}
