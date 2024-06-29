#[macro_export]
macro_rules! bound_f32_impl {
	($struct_name: ty, $min: literal, $max: literal) => {
		impl $struct_name {
			pub const MIN: f32 = $min;
			pub const MAX: f32 = $max;
			
			pub fn new(value: f32) -> Self {
				return Self { inner_value: f32::clamp(value, Self::MIN, Self::MAX) };
			}
			
			pub fn get(&self) -> f32 {
				return self.inner_value;
			}
			
			pub fn set(&mut self, value: f32) {
				self.inner_value = f32::clamp(value, Self::MIN, Self::MAX);
			}
		}
		
		impl std::ops::Add for $struct_name {
			type Output = Self;
		
			fn add(self, other: Self) -> Self::Output {
				return Self::new(self.inner_value + other.inner_value);
			}
		}
		
		impl std::ops::Add<f32> for $struct_name {
			type Output = f32;
		
			fn add(self, other: f32) -> Self::Output {
				return self.inner_value + other;
			}
		}
		
		impl std::ops::Add<$struct_name> for f32 {
			type Output = f32;
		
			fn add(self, other: $struct_name) -> Self::Output {
				return self + other.inner_value;
			}
		}
		
		impl std::ops::AddAssign for $struct_name {
			fn add_assign(&mut self, other: Self) {
				self.set(self.inner_value + other.inner_value);
			}
		}
		
		impl std::ops::AddAssign<f32> for $struct_name {
			fn add_assign(&mut self, other: f32) {
				self.set(self.inner_value + other);
			}
		}
		
		impl std::ops::Sub for $struct_name {
			type Output = Self;
		
			fn sub(self, other: Self) -> Self::Output {
				return Self::new(self.inner_value - other.inner_value);
			}
		}
		
		impl std::ops::Sub<f32> for $struct_name {
			type Output = f32;
		
			fn sub(self, other: f32) -> Self::Output {
				return self.inner_value - other;
			}
		}
		
		impl std::ops::Sub<$struct_name> for f32 {
			type Output = f32;
		
			fn sub(self, other: $struct_name) -> Self::Output {
				return self - other.inner_value;
			}
		}
		
		impl std::ops::SubAssign for $struct_name {
			fn sub_assign(&mut self, other: Self) {
				self.set(self.inner_value - other.inner_value);
			}
		}
		
		impl std::ops::SubAssign<f32> for $struct_name {
			fn sub_assign(&mut self, other: f32) {
				self.set(self.inner_value - other);
			}
		}
		
		impl std::ops::Div for $struct_name {
			type Output = Self;
		
			fn div(self, other: Self) -> Self::Output {
				return Self::new(self.inner_value / other.inner_value);
			}
		}
		
		impl std::ops::Div<f32> for $struct_name {
			type Output = f32;
		
			fn div(self, other: f32) -> Self::Output {
				return self.inner_value / other;
			}
		}
		
		impl std::ops::Div<$struct_name> for f32 {
			type Output = f32;
		
			fn div(self, other: $struct_name) -> Self::Output {
				return self / other.inner_value;
			}
		}
		
		impl std::ops::DivAssign for $struct_name {
			fn div_assign(&mut self, other: Self) {
				self.set(self.inner_value / other.inner_value);
			}
		}
		
		impl std::ops::DivAssign<f32> for $struct_name {
			fn div_assign(&mut self, other: f32) {
				self.set(self.inner_value / other);
			}
		}
		
		impl std::ops::Mul for $struct_name {
			type Output = Self;
		
			fn mul(self, other: Self) -> Self::Output {
				return Self::new(self.inner_value * other.inner_value);
			}
		}
		
		impl std::ops::Mul<f32> for $struct_name {
			type Output = f32;
		
			fn mul(self, other: f32) -> Self::Output {
				return self.inner_value * other;
			}
		}
		
		impl std::ops::Mul<$struct_name> for f32 {
			type Output = f32;
		
			fn mul(self, other: $struct_name) -> Self::Output {
				return self * other.inner_value;
			}
		}
		
		impl std::ops::MulAssign for $struct_name {
			fn mul_assign(&mut self, other: Self) {
				self.set(self.inner_value * other.inner_value);
			}
		}
		
		impl std::ops::MulAssign<f32> for $struct_name {
			fn mul_assign(&mut self, other: f32) {
				self.set(self.inner_value * other);
			}
		}
		
		impl core::cmp::PartialEq<Self> for $struct_name {
			fn eq(&self, other: &Self) -> bool {
				return float_cmp::approx_eq!(f32, self.inner_value, other.inner_value);
			}
		}
		
		impl core::cmp::PartialEq<f32> for $struct_name {
			fn eq(&self, other: &f32) -> bool {
				return float_cmp::approx_eq!(f32, self.inner_value, *other);
			}
		}
		
		impl core::cmp::PartialEq<$struct_name> for f32 {
			fn eq(&self, other: &$struct_name) -> bool {
				return float_cmp::approx_eq!(f32, *self, other.inner_value);
			}
		}
		
		impl core::cmp::PartialOrd for $struct_name {
			fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
				return f32::partial_cmp(&self.inner_value, &other.inner_value);
			}
		}
		
		impl core::cmp::PartialOrd<f32> for $struct_name {
			fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
				return f32::partial_cmp(&self.inner_value, &(*other));
			}
		}
		
		impl core::cmp::PartialOrd<$struct_name> for f32 {
			fn partial_cmp(&self, other: &$struct_name) -> Option<std::cmp::Ordering> {
				return f32::partial_cmp(&(*self), &other.inner_value);
			}
		}
		
		impl core::convert::From<f32> for $struct_name {
			fn from(value: f32) -> Self {
				return Self::new(value);
			}
		}
		
		impl core::default::Default for $struct_name {
			fn default() -> Self {
				return Self::new(Self::MIN);
			}
		}
		
		impl std::ops::Deref for $struct_name {
			type Target = f32;
		
			fn deref(&self) -> &Self::Target {
				return &self.inner_value;
			}
		}
	};
}

#[cfg(all(test, feature = "serde"))]
mod tests {
	use serde::{Deserialize, Serialize};

	#[allow(non_camel_case_types)]
	#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
	pub struct B_f32 {
		inner_value: f32
	}

	bound_f32_impl!(B_f32, -5.0, 5.0);

	#[test]
	fn test_new_bound_f32_within_range() {
		let value = 3.0;
		let bound_f32 = B_f32::new(value);
		assert!(float_cmp::approx_eq!(f32, bound_f32.inner_value, value));
	}

	#[test]
	fn test_new_bound_f32_outside_range() {
		let value = 10.0;
		let bound_f32 = B_f32::new(value);
		assert!(float_cmp::approx_eq!(f32, bound_f32.inner_value, B_f32::MAX));
	}

	#[test]
	fn test_new_bound_f32_less_than_min() {
		let value = -10.0;
		let bound_f32 = B_f32::new(value);
		assert!(float_cmp::approx_eq!(f32, bound_f32.inner_value, B_f32::MIN));
	}

	#[test]
	fn test_new_bound_f32_greater_than_max() {
		let value = 10.0;
		let bound_f32 = B_f32::new(value);
		assert!(float_cmp::approx_eq!(f32, bound_f32.inner_value, B_f32::MAX));
	}

	#[test]
	fn test_new_bound_f32_with_min_value() {
		let bound_f32 = B_f32::new(B_f32::MIN);
		assert!(float_cmp::approx_eq!(f32, bound_f32.inner_value, B_f32::MIN));
	}

	#[test]
	fn test_set_bound_f32_within_range() {
		let mut bound_f32 = B_f32::default();
		let new_value = 2.0;
		bound_f32.set(new_value);
		assert!(float_cmp::approx_eq!(f32, bound_f32.inner_value, new_value));
	}

	#[test]
	fn test_set_bound_f32_outside_range() {
		let mut bound_f32 = B_f32::default();
		let new_value = 10.0;
		bound_f32.set(new_value);
		assert!(float_cmp::approx_eq!(f32, bound_f32.inner_value, B_f32::MAX));
	}

	#[test]
	fn test_add_bound_f32_within_range() {
		let value1 = 2.0;
		let value2 = 3.0;
		let bound_f32_1 = B_f32::new(value1);
		let bound_f32_2 = B_f32::new(value2);
		let result = bound_f32_1 + bound_f32_2;
		assert!(float_cmp::approx_eq!(f32, result.inner_value, 5.0));
	}

	#[test]
	fn test_add_bound_f32_greater_than_max() {
		let value1 = 4.0;
		let value2 = 3.0;
		let bound_f32_1 = B_f32::new(value1);
		let bound_f32_2 = B_f32::new(value2);
		let result = bound_f32_1 + bound_f32_2;
		assert!(float_cmp::approx_eq!(f32, result.inner_value, B_f32::MAX));
	}

	#[test]
	fn test_addition_bound_f32_f32() {
		let bound = B_f32::new(2.0);
		let value = 3.0;
		let result = bound + value;
		assert!(float_cmp::approx_eq!(f32, result, 5.0));
	}

	#[test]
	fn test_addition_f32_bound_f32() {
		let value = 2.0;
		let bound = B_f32::new(3.0);
		let result = value + bound;
		assert!(float_cmp::approx_eq!(f32, result, 5.0));
	}

	#[test]
	fn subtract_bound_f32_objects_within_range() {
		let bound1 = B_f32::new(3.0);
		let bound2 = B_f32::new(2.0);
		let result = bound1 - bound2;
		assert!(float_cmp::approx_eq!(f32, result.inner_value, 1.0));
	}

	#[test]
	fn subtract_bound_f32_objects_outside_range() {
		let bound1 = B_f32::new(-6.0);
		let bound2 = B_f32::new(4.0);
		let result = bound1 - bound2;
		assert!(float_cmp::approx_eq!(f32, result.inner_value, B_f32::MIN));
	}

	#[test]
	fn multiply_bound_f32_objects_within_range() {
		let bound1 = B_f32::new(2.0);
		let bound2 = B_f32::new(3.0);
		let result = bound1 * bound2;
		assert!(float_cmp::approx_eq!(f32, result.inner_value, B_f32::MAX));
	}

	#[test]
	fn multiply_with_product_less_than_min_should_return_min() {
		let a = B_f32::new(-4.0);
		let b = B_f32::new(2.0);
		let result = a * b;
		assert!(float_cmp::approx_eq!(f32, result.inner_value, B_f32::MIN));
	}

	#[test]
	fn multiply_with_product_greater_than_min_should_return_correct_value() {
		let a = B_f32::new(-4.0);
		let b = B_f32::new(0.5);
		let result = a * b;
		assert!(float_cmp::approx_eq!(f32, result.inner_value, -2.0));
	}

	#[test]
	fn multiply_with_product_greater_than_max_should_return_max() {
		let a = B_f32::new(4.0);
		let b = B_f32::new(2.0);
		let result = a * b;
		assert!(float_cmp::approx_eq!(f32, result.inner_value, B_f32::MAX));
	}

	#[test]
	fn test_divide_bound_f32_objects_within_range() {
		let bound1 = B_f32::new(2.0);
		let bound2 = B_f32::new(0.5);
		let result = bound1 / bound2;
		assert!(float_cmp::approx_eq!(f32, result.inner_value, 4.0));
	}

	#[test]
	fn test_divide_by_non_zero_returns_correct_result() {
		let bound_f32 = B_f32::new(10.0);
		let result = bound_f32 / 2.0;
		assert!(float_cmp::approx_eq!(f32, result, 2.5));
	}

	#[test]
	fn test_bound_f32_comparison() {
		let value1 = B_f32::new(2.0);
		let value2 = B_f32::new(2.0);
		assert!(float_cmp::approx_eq!(f32, value1.inner_value, value2.inner_value));
	}
}
