use super::IterGenerator;

impl<'a, TValue> IterGenerator<'a, &'a TValue> for [TValue] {
	fn iterate(&'a self) -> impl ExactSizeIterator<Item = &'a TValue> {
		return self.iter();
	}
}

impl<'a, TValue> IterGenerator<'a, &'a TValue> for &'a [TValue] {
	fn iterate(&'a self) -> impl ExactSizeIterator<Item = &'a TValue> {
		return self.iter();
	}
}

impl<'a, TValue> IterGenerator<'a, &'a TValue> for &'a mut [TValue] {
	fn iterate(&'a self) -> impl ExactSizeIterator<Item = &'a TValue> {
		return self.iter();
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let slice = [1, 2, 3].as_slice();
		input_ref_test(&slice);

		let mut s = [1, 2, 3];
		let slice = s.as_mut_slice();
		input_ref_test(&slice);
	}

	fn input_ref_test<'a>(input: &'a impl IterGenerator<'a, &'a usize>) {
		for value in input.iterate() {
			println!("{value}");
		}
	}
}