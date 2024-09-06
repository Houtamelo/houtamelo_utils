use rand::Rng;

pub trait GetRandom<T> {
	fn get_random(self, rng: &mut impl Rng) -> Option<T>;
}

impl<T, Iter: IntoIterator<Item = T>> GetRandom<T> for Iter {
	fn get_random(self, rng: &mut impl Rng) -> Option<T> {
		let mut elements = self.into_iter().collect::<Vec<_>>();

		if !elements.is_empty() {
			let index = rng.gen_range(0..elements.len());
			Some(elements.remove(index))
		} else {
			None
		}
	}
}

pub trait GetRandomRef<'a, T: 'a> {
	fn get_random_ref(self, rng: &mut impl Rng) -> Option<&'a T>;
}

impl<'a, T: 'a, Iter: IntoIterator<Item = &'a T>> GetRandomRef<'a, T> for Iter {
	fn get_random_ref(self, rng: &mut impl Rng) -> Option<&'a T> {
		let mut elements = self.into_iter().collect::<Vec<_>>();

		if !elements.is_empty() {
			let index = rng.gen_range(0..elements.len());
			Some(elements.remove(index))
		} else {
			None
		}
	}
}

pub trait GetRandomMut<'a, T: 'a> {
	fn get_random_mut(self, rng: &mut impl Rng) -> Option<&'a mut T>;
}

impl<'a, T: 'a, Iter: IntoIterator<Item = &'a mut T>> GetRandomMut<'a, T> for Iter {
	fn get_random_mut(self, rng: &mut impl Rng) -> Option<&'a mut T> {
		let mut elements = self.into_iter().collect::<Vec<_>>();
		
		if !elements.is_empty() {
			let index = rng.gen_range(0..elements.len());
			Some(elements.remove(index))
		} else {
			None
		}
	}
}


#[allow(unused)]
#[test]
fn must_compile() {
	use std::collections::{HashMap, HashSet};
	let mut rng = rand::thread_rng();
	let x = vec![1, 2, 3].get_random(&mut rng);
	let x= vec![1, 2, 3].get_random_ref(&mut rng);
	
	vec![1, 2, 3].get_random_mut(&mut rng);
	[1, 2, 3].get_random(&mut rng);
	[1, 2, 3].get_random_mut(&mut rng);
	let str = HashSet::from(["1", "2", "3"]).get_random(&mut rng);
	let t = HashMap::from([(1, 1), (2, 2), (3, 3)]).get_random(&mut rng).unwrap();
}