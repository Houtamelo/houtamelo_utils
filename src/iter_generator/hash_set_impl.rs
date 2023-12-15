use super::IterGenerator;

impl<'a, TValue> IterGenerator<'a, &'a TValue> for std::collections::HashSet<TValue> {
	fn iterate(&'a self) -> impl ExactSizeIterator<Item = &'a TValue> {
		return self.iter();
	}
}

impl<'a, TValue> IterGenerator<'a, &'a TValue> for &'a std::collections::HashSet<TValue> {
	fn iterate(&'a self) -> impl ExactSizeIterator<Item = &'a TValue> {
		return self.iter();
	}
}

impl<'a, TValue> IterGenerator<'a, &'a TValue> for &'a mut std::collections::HashSet<TValue> {
	fn iterate(&'a self) -> impl ExactSizeIterator<Item = &'a TValue> {
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

	fn input_ref_test<'a>(input: &'a impl IterGenerator<'a, &'a usize>) {
		for value in input.iterate() {
			println!("{value}");
		}
	}
}