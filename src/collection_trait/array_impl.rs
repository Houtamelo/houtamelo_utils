use super::CollectionByRef;

impl<'a, TValue, const TCOUNT: usize> CollectionByRef<'a, &'a TValue> for [TValue; TCOUNT] {
	type Return = core::slice::Iter<'a, TValue>;

	fn iterate(&'a self) -> Self::Return {
		return self.iter();
	}
}

impl<'a, TValue, const TCOUNT: usize> CollectionByRef<'a, &'a TValue> for &'a [TValue; TCOUNT] {
	type Return = core::slice::Iter<'a, TValue>;

	fn iterate(&'a self) -> Self::Return {
		return self.iter();
	}
}

impl<'a, TValue, const COUNT: usize> CollectionByRef<'a, &'a TValue> for &'a mut [TValue; COUNT] {
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
		let array = [1, 2, 3];
		input_ref_test(&array);

		let array = &[1, 2, 3];
		input_ref_test(&array);

		let array = &mut [1, 2, 3];
		input_ref_test(&array);
	}
	
	fn input_ref_test<'a>(input: &'a impl CollectionByRef<'a, &'a usize>) {
		for value in input.iterate() {
			println!("{value}");
		}
	}
}