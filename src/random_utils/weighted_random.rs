use rand::Rng;
use std::borrow::Borrow;

pub trait WeightedRandom<'a, T: ?Sized> {
	fn get_weighted_random(&'a self, rng: &mut impl Rng) -> Option<&'a T>;
}

impl<'a, T: ?Sized, Iter: 'a + ?Sized, Item: 'a + ValueWeight<T>> WeightedRandom<'a, T> for Iter
	where
		&'a Iter: IntoIterator<Item = &'a Item>,
{
	fn get_weighted_random(&'a self, rng: &mut impl Rng) -> Option<&'a T> {
		let weight_sum = self
			.into_iter()
			.fold(0., |acc, val_weight| acc + val_weight.weight());

		let random = rng.gen_range(0.0..=weight_sum);

		let mut current_sum = weight_sum;
		for val_weight in self.into_iter() {
			current_sum -= val_weight.weight();
			if current_sum <= random {
				return Some(val_weight.value());
			}
		}

		None
	}
}

trait ValueWeight<T: ?Sized> {
	fn weight(&self) -> f32;
	fn value(&self) -> &T;
}

impl<T, float: ?Sized + Borrow<f32>> ValueWeight<T> for (T, float) {
	fn weight(&self) -> f32 { *self.1.borrow() }
	fn value(&self) -> &T { &self.0 }
}

impl<'a, T, float: ?Sized + Borrow<f32>> ValueWeight<T> for (&'a T, &'a float) {
	fn weight(&self) -> f32 { *self.1.borrow() }
	fn value(&self) -> &T { self.0 }
}

impl<'a, T: Clone, float: Copy + Into<f32>> ValueWeight<T> for &'a (T, float) {
	fn weight(&self) -> f32 { self.1.into() }
	fn value(&self) -> &T { &self.0 }
}

#[allow(unused)]
#[test]
fn must_compile() {
	let mut rng = rand::thread_rng();
	let weights = [(1, 1.0), (2, 2.0), (3, 3.0)];
	let random = weights.get_weighted_random(&mut rng);
	
	let weights = vec![(1, 1.0), (2, 2.0), (3, 3.0)];
	let random = weights.get_weighted_random(&mut rng);
	
	let weights = [(1, &1.0), (2, &2.0), (3, &3.0)];
	let random = weights.get_weighted_random(&mut rng);
	
	let weights = [(1, &mut 1.0), (2, &mut 2.0), (3, &mut 3.0)];
	let random = weights.get_weighted_random(&mut rng);
}
