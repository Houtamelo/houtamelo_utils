pub trait Clamp01<T> {
	fn clamp01(self) -> Self;
}

impl Clamp01<f32> for f32 {
	fn clamp01(self) -> f32 { f32::clamp(self, 0.0, 1.0) }
}

impl Clamp01<f64> for f64 {
	fn clamp01(self) -> f64 { f64::clamp(self, 0.0, 1.0) }
}

#[cfg(test)]
mod tests {
	use float_cmp::approx_eq;

	use super::*;

	#[test]
	fn should_return_input_value_if_between_0_and_1() {
		let input = 0.5;
		let result = input.clamp01();
		assert!(approx_eq!(
			f32, result, input
		));
	}

	#[test]
	fn should_return_0_if_input_value_is_less_than_0() {
		let input = -1.0;
		let result = input.clamp01();
		assert!(approx_eq!(
			f32, result, 0.0
		));
	}

	#[test]
	fn should_return_1_if_input_value_is_greater_than_1() {
		let input = 2.0;
		let result = input.clamp01();
		assert!(approx_eq!(
			f32, result, 1.0
		));
	}

	#[test]
	fn should_return_nan_if_input_value_is_nan() {
		let input = f32::NAN;
		let result = input.clamp01();
		assert!(result.is_nan());
	}

	#[test]
	fn should_work_with_negative_input_values() {
		let input = -0.5;
		let result = input.clamp01();
		assert!(approx_eq!(
			f32, result, 0.0
		));
	}

	#[test]
	fn should_work_with_very_large_input_values() {
		let input = 1000000.0;
		let result = input.clamp01();
		assert!(approx_eq!(
			f32, result, 1.0
		));
	}

	#[test]
	fn should_work_with_very_small_input_values() {
		let input = 0.000001;
		let result = input.clamp01();
		assert!(approx_eq!(
			f32, result, 0.000001
		));
	}
}
