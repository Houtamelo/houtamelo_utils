use super::CollectionByRef;

impl<'a, TValue> CollectionByRef<'a, &'a TValue> for Vec<TValue> {
	type Return = core::slice::Iter<'a, TValue>;

	fn iterate(&'a self) -> Self::Return {
		return self.iter();
	}
}

impl<'a, TValue> CollectionByRef<'a, &'a TValue> for &'a Vec<TValue> {
	type Return = core::slice::Iter<'a, TValue>;

	fn iterate(&'a self) -> Self::Return {
		return self.iter();
	}
}

impl<'a, TValue> CollectionByRef<'a, &'a TValue> for &'a mut Vec<TValue> {
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
		let vec = vec![1, 2, 3];
		input_ref_test(&vec);

		let vec = &vec![1, 2, 3];
		input_ref_test(&vec);

		let vec = &mut vec![1, 2, 3];
		input_ref_test(&vec);
	}

	fn input_ref_test<'a>(input: &'a impl CollectionByRef<'a, &'a usize>) {
		for value in input.iterate() {
			println!("{value}");
		}
	}
}