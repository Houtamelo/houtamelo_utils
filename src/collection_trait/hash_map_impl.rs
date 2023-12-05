use std::collections::hash_map::{Iter, Keys, Values};
use std::collections::HashMap;
use super::{CollectionByRef, CollectionByValue};

impl<'a, TKey, TValue> CollectionByRef<'a, (&'a TKey, &'a TValue)> for HashMap<TKey, TValue> {
	type Return = Iter<'a, TKey, TValue>;

	fn iterate(&'a self) -> Self::Return {
		return self.iter();
	}
}

impl<'a, TKey, TValue> CollectionByRef<'a, (&'a TKey, &'a TValue)> for &'a HashMap<TKey, TValue> {
	type Return = Iter<'a, TKey, TValue>;

	fn iterate(&'a self) -> Self::Return {
		return self.iter();
	}
}

impl<'a, TKey, TValue> CollectionByRef<'a, (&'a TKey, &'a TValue)> for &'a mut HashMap<TKey, TValue> {
	type Return = Iter<'a, TKey, TValue>;
	
	fn iterate(&'a self) -> Self::Return {
		return self.iter();
	}
}

impl<'a, TKey, TValue> CollectionByValue<&'a TKey> for Keys<'a, TKey, TValue> {
	type Return = Keys<'a, TKey, TValue>;

	fn iterate_by_value(self) -> Self::Return {
		return self;
	}
}

impl<'a, TKey, TValue> CollectionByValue<&'a TValue> for Values<'a, TKey, TValue> {
	type Return = Values<'a, TKey, TValue>;

	fn iterate_by_value(self) -> Self::Return {
		return self;
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test() {
		let hash_map: HashMap<usize, usize> = HashMap::from([(1, 2), (3, 4)]);
		input_value_test(hash_map.keys());
		input_value_test(hash_map.values());
		input_ref_test(&hash_map);
	}

	fn input_ref_test<'a>(input: &'a impl CollectionByRef<'a, (&'a usize, &'a usize)>) {
		for value in input.iterate() {
			println!("{value:?}");
		}
	}

	#[cfg(test)]
	fn input_value_test<'a>(input: impl CollectionByValue<&'a usize>) {
		for value in input.iterate_by_value() {
			println!("{value}");
		}
	}
}