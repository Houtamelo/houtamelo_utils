use super::IterGenerator;

impl<'a, TValue, const TCOUNT: usize> IterGenerator<'a, &'a TValue> for [TValue; TCOUNT] {
	fn iterate(&'a self) -> impl Iterator<Item = &'a TValue> {
		return self.iter();
	}
}

impl<'a, TValue, const TCOUNT: usize> IterGenerator<'a, &'a TValue> for &'a [TValue; TCOUNT] {
	fn iterate(&'a self) -> impl Iterator<Item = &'a TValue> {
		return self.iter();
	}
}

impl<'a, TValue, const COUNT: usize> IterGenerator<'a, &'a TValue> for &'a mut [TValue; COUNT] {
	fn iterate(&'a self) -> impl Iterator<Item = &'a TValue> {
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
	
	fn input_ref_test<'a>(input: &'a impl IterGenerator<'a, &'a usize>) {
		for value in input.iterate() {
			println!("{value}");
		}
	}
}