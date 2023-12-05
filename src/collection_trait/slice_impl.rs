use super::CollectionByRef;

impl<'a, TValue> CollectionByRef<'a, &'a TValue> for [TValue] {
	type Return = core::slice::Iter<'a, TValue>;

	fn iterate(&'a self) -> Self::Return {
		return self.iter();
	}
}

impl<'a, TValue> CollectionByRef<'a, &'a TValue> for &'a [TValue] {
	type Return = core::slice::Iter<'a, TValue>;

	fn iterate(&'a self) -> Self::Return {
		return self.iter();
	}
}

impl<'a, TValue> CollectionByRef<'a, &'a TValue> for &'a mut [TValue] {
	type Return = core::slice::Iter<'a, TValue>;

	fn iterate(&'a self) -> Self::Return {
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

	fn input_ref_test<'a>(input: &'a impl CollectionByRef<'a, &'a usize>) {
		for value in input.iterate() {
			println!("{value}");
		}
	}
}