use super::CollectionByRef;

impl<'a, TValue> CollectionByRef<'a, &'a TValue> for std::collections::HashSet<TValue> {
	type Return = std::collections::hash_set::Iter<'a, TValue>;

	fn iterate(&'a self) -> Self::Return {
		return self.iter();
	}
}

impl<'a, TValue> CollectionByRef<'a, &'a TValue> for &'a std::collections::HashSet<TValue> {
	type Return = std::collections::hash_set::Iter<'a, TValue>;

	fn iterate(&'a self) -> Self::Return {
		return self.iter();
	}
}

impl<'a, TValue> CollectionByRef<'a, &'a TValue> for &'a mut std::collections::HashSet<TValue> {
	type Return = std::collections::hash_set::Iter<'a, TValue>;

	fn iterate(&'a self) -> Self::Return {
		return self.iter();
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let hash_set = std::collections::HashSet::new();
		input_ref_test(&hash_set);

		let hash_set = &std::collections::HashSet::new();
		input_ref_test(&hash_set);

		let hash_set = &mut std::collections::HashSet::new();
		input_ref_test(&hash_set);
	}

	fn input_ref_test<'a>(input: &'a impl CollectionByRef<'a, &'a usize>) {
		for value in input.iterate() {
			println!("{value}");
		}
	}
}