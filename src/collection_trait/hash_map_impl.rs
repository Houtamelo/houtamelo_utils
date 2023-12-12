use std::collections::HashMap;
use super::IterGenerator;

impl<'a, TKey, TValue> IterGenerator<'a, (&'a TKey, &'a TValue)> for HashMap<TKey, TValue> {
	fn iterate(&'a self) -> impl ExactSizeIterator<Item = (&'a TKey, &'a TValue)> {
		return self.iter();
	}
}

impl<'a, TKey, TValue> IterGenerator<'a, (&'a TKey, &'a TValue)> for &'a HashMap<TKey, TValue> {
	fn iterate(&'a self) -> impl ExactSizeIterator<Item = (&'a TKey, &'a TValue)> {
		return self.iter();
	}
}

impl<'a, TKey, TValue> IterGenerator<'a, (&'a TKey, &'a TValue)> for &'a mut HashMap<TKey, TValue> {
	fn iterate(&'a self) -> impl ExactSizeIterator<Item = (&'a TKey, &'a TValue)> {
		return self.iter();
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test() {
		let hash_map: HashMap<usize, usize> = HashMap::from([(1, 2), (3, 4)]);
		input_ref_test(&hash_map);
	}

	fn input_ref_test<'a>(input: &'a impl IterGenerator<'a, (&'a usize, &'a usize)>) {
		for value in input.iterate() {
			println!("{value:?}");
		}
	}
}