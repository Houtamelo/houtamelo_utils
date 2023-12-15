use super::IterGenerator;

impl<'a, TValue> IterGenerator<'a, &'a TValue> for Vec<TValue> {
	fn iterate(&'a self) -> impl ExactSizeIterator<Item = &'a TValue> {
		return self.iter();
	}
}

impl<'a, TValue> IterGenerator<'a, &'a TValue> for &'a Vec<TValue> {
	fn iterate(&'a self) -> impl ExactSizeIterator<Item = &'a TValue> {
		return self.iter();
	}
}

impl<'a, TValue> IterGenerator<'a, &'a TValue> for &'a mut Vec<TValue> {
	fn iterate(&'a self) -> impl ExactSizeIterator<Item = &'a TValue> {
		return self.iter();
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let vec = vec![1, 2, 3];
		input_ref_test(&vec);

		let vec = &vec![1, 2, 3];
		input_ref_test(&vec);

		let vec = &mut vec![1, 2, 3];
		input_ref_test(&vec);
	}

	fn input_ref_test<'a>(input: &'a impl IterGenerator<'a, &'a usize>) {
		for value in input.iterate() {
			println!("{value}");
		}
	}
}