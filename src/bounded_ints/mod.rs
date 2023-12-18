#[macro_use] pub mod macros;
pub mod bound_u8;
pub mod bound_u16;
pub mod bound_i8;
pub mod bound_i16;

use macros::saturate_into::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use core::cmp::{Ord, Ordering, PartialOrd};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct BoundUSize<const MIN: usize, const MAX: usize> {
	inner_value: usize
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct BoundU32<const MIN: u32, const MAX: u32> {
	inner_value: u32
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct BoundU64<const MIN: u64, const MAX: u64> {
	inner_value: u64
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct BoundISize<const MIN: isize, const MAX: isize> {
	inner_value: isize
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct BoundI32<const MIN: i32, const MAX: i32> {
	inner_value: i32
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct BoundI64<const MIN: i64, const MAX: i64> {
	inner_value: i64
}

macros::bound_main!(BoundISize<MIN, MAX>, isize);
macros::bound_from!(BoundISize<MIN, MAX>, isize);
macros::bound_add!(BoundISize<MIN, MAX>, isize);
macros::bound_sub!(BoundISize<MIN, MAX>, isize);
macros::bound_mul!(BoundISize<MIN, MAX>, isize);
macros::bound_div!(BoundISize<MIN, MAX>, isize);
macros::bound_rem!(BoundISize<MIN, MAX>, isize);
macros::bound_eq!(BoundISize<MIN, MAX>, isize);
macros::bound_ord!(BoundISize<MIN, MAX>, isize);

macros::bound_main!(BoundI32<MIN, MAX>, i32);
macros::bound_from!(BoundI32<MIN, MAX>, i32);
macros::bound_add!(BoundI32<MIN, MAX>, i32);
macros::bound_sub!(BoundI32<MIN, MAX>, i32);
macros::bound_mul!(BoundI32<MIN, MAX>, i32);
macros::bound_div!(BoundI32<MIN, MAX>, i32);
macros::bound_rem!(BoundI32<MIN, MAX>, i32);
macros::bound_eq!(BoundI32<MIN, MAX>, i32);
macros::bound_ord!(BoundI32<MIN, MAX>, i32);

macros::bound_main!(BoundI64<MIN, MAX>, i64);
macros::bound_from!(BoundI64<MIN, MAX>, i64);
macros::bound_add!(BoundI64<MIN, MAX>, i64);
macros::bound_sub!(BoundI64<MIN, MAX>, i64);
macros::bound_mul!(BoundI64<MIN, MAX>, i64);
macros::bound_div!(BoundI64<MIN, MAX>, i64);
macros::bound_rem!(BoundI64<MIN, MAX>, i64);
macros::bound_eq!(BoundI64<MIN, MAX>, i64);
macros::bound_ord!(BoundI64<MIN, MAX>, i64);

macros::bound_main!(BoundUSize<MIN, MAX>, usize);
macros::bound_from!(BoundUSize<MIN, MAX>, usize);
macros::bound_add!(BoundUSize<MIN, MAX>, usize);
macros::bound_sub!(BoundUSize<MIN, MAX>, usize);
macros::bound_mul!(BoundUSize<MIN, MAX>, usize);
macros::bound_div!(BoundUSize<MIN, MAX>, usize);
macros::bound_rem!(BoundUSize<MIN, MAX>, usize);
macros::bound_eq!(BoundUSize<MIN, MAX>, usize);

macros::bound_ord!(BoundUSize<MIN, MAX>, usize);
macros::bound_main!(BoundU32<MIN, MAX>, u32);
macros::bound_from!(BoundU32<MIN, MAX>, u32);
macros::bound_add!(BoundU32<MIN, MAX>, u32);
macros::bound_sub!(BoundU32<MIN, MAX>, u32);
macros::bound_mul!(BoundU32<MIN, MAX>, u32);
macros::bound_div!(BoundU32<MIN, MAX>, u32);
macros::bound_rem!(BoundU32<MIN, MAX>, u32);
macros::bound_eq!(BoundU32<MIN, MAX>, u32);

macros::bound_ord!(BoundU32<MIN, MAX>, u32);
macros::bound_main!(BoundU64<MIN, MAX>, u64);
macros::bound_from!(BoundU64<MIN, MAX>, u64);
macros::bound_add!(BoundU64<MIN, MAX>, u64);
macros::bound_sub!(BoundU64<MIN, MAX>, u64);
macros::bound_mul!(BoundU64<MIN, MAX>, u64);
macros::bound_div!(BoundU64<MIN, MAX>, u64);
macros::bound_rem!(BoundU64<MIN, MAX>, u64);
macros::bound_eq!(BoundU64<MIN, MAX>, u64);

macros::bound_ord!(BoundU64<MIN, MAX>, u64);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_create_boundusize_within_bounds() {
		let boundusize = BoundUSize::<0, 10>::new(5);
		assert_eq!(boundusize.get(), 5);
	}

	#[test]
	fn test_create_boundusize_greater_than_maximum() {
		let boundusize = BoundUSize::<0, 10>::new(15);
		assert_eq!(boundusize.get(), 10);
	}

	#[test]
	fn test_create_boundusize_less_than_minimum() {
		let boundusize = BoundUSize::<2, 10>::new(1);
		assert_eq!(boundusize.get(), 2);
	}

	#[test]
	fn test_set_boundusize_within_bounds() {
		let mut boundusize = BoundUSize::<0, 10>::new(5);
		boundusize.set(7);
		assert_eq!(boundusize.get(), 7);
	}

	#[test]
	fn test_convert_boundusize_maximum_value_to_usize() {
		let boundusize = BoundUSize::<0, 10>::new(15);
		let value: usize = boundusize.into();
		assert_eq!(value, 10);
	}

	#[test]
	fn convert_boundusize_to_usize_returns_inner_value() {
		let bound_usize: BoundUSize<0, 10> = BoundUSize::new(5);
		let inner_value: usize = bound_usize.into();
		assert_eq!(inner_value, 5);
	}


	// Converting a usize to a BoundUSize with a value within the bounds returns a BoundUSize with the same value.
	#[test]
	fn convert_usize_within_bounds_returns_same_value() {
		let value: usize = 10;
		let bound: BoundUSize<0, 100> = BoundUSize::from(value);
		assert_eq!(bound.get(), value);
	}

	// Converting a usize to a BoundUSize with a value greater than the maximum returns a BoundUSize with the maximum value.
	#[test]
	fn convert_usize_to_boundusize_with_greater_value() {
		let value: usize = 100;
		let bound_usize: BoundUSize<0, 50> = BoundUSize::from(value);
		assert_eq!(bound_usize.get(), 50);
	}

	// Converting a usize to a BoundUSize with a value less than the minimum returns a BoundUSize with the minimum value.
	#[test]
	fn convert_usize_to_boundusize_with_value_less_than_minimum_returns_minimum() {
		let value: usize = 5;
		let bound: BoundUSize<10, 20> = BoundUSize::from(value);
		assert_eq!(bound.get(), 10);
	}


	// Creating a BoundI32 instance with a value within the bounds results in a valid instance
	#[test]
	fn test_bound_i32_within_bounds() {
		let bound_i32 = BoundI32::<-10, 10>::new(5);
		assert_eq!(bound_i32.get(), 5);
	}

	// Creating a BoundI32 instance with a value equal to the minimum bound results in a valid instance with the minimum bound value
	#[test]
	fn test_bound_i32_equal_to_minimum_bound() {
		let bound_i32 = BoundI32::<-10, 10>::new(-10);
		assert_eq!(bound_i32.get(), -10);
	}

	// Creating a BoundI32 instance with a value equal to the maximum bound results in a valid instance with the maximum bound value
	#[test]
	fn test_bound_i32_equal_to_maximum_bound() {
		let bound_i32 = BoundI32::<-10, 10>::new(10);
		assert_eq!(bound_i32.get(), 10);
	}

	// Getting the value of a BoundI32 instance returns the correct value
	#[test]
	fn test_bound_i32_get_value() {
		let bound_i32 = BoundI32::<-10, 10>::new(5);
		assert_eq!(bound_i32.get(), 5);
	}

	// Setting the value of a BoundI32 instance with a value within the bounds results in a valid instance with the new value
	#[test]
	fn test_bound_i32_set_value_within_bounds() {
		let mut bound_i32 = BoundI32::<-10, 10>::new(5);
		bound_i32.set(7);
		assert_eq!(bound_i32.get(), 7);
	}

	// Setting the value of a BoundI32 instance with a value less than the minimum bound results in a valid instance with the minimum bound value
	#[test]
	fn test_bound_i32_set_value_less_than_minimum_bound() {
		let mut bound_i32 = BoundI32::<-10, 10>::new(5);
		bound_i32.set(-15);
		assert_eq!(bound_i32.get(), -10);
	}

	// Creating a BoundI32 instance with a value less than the minimum bound results in a valid instance with the minimum bound value
	#[test]
	fn test_bound_i32_value_less_than_minimum_bound() {
		let bound_i32 = BoundI32::<-10, 10>::new(-15);
		assert_eq!(bound_i32.get(), -10);
	}

	// Creating a BoundI32 instance with a value greater than the maximum bound results in a valid instance with the maximum bound value
	#[test]
	fn test_bound_i32_value_greater_than_maximum_bound() {
		let bound_i32 = BoundI32::<-10, 10>::new(15);
		assert_eq!(bound_i32.get(), 10);
	}

	// Setting the value of a BoundI32 instance with a value less than the minimum bound and greater than the maximum bound results in a valid instance with the minimum bound value
	#[test]
	fn test_bound_i32_value_less_than_minimum_and_greater_than_maximum() {
		let mut bound_i32 = BoundI32::<-10, 10>::new(5);
		bound_i32.set(-15);
		assert_eq!(bound_i32.get(), -10);
	}

	// Setting the value of a BoundI32 instance with a value greater than the maximum bound and less than the minimum bound results in a valid instance with the maximum bound value
	#[test]
	fn test_bound_i32_value_greater_than_maximum_and_less_than_minimum() {
		let mut bound_i32 = BoundI32::<-10, 10>::new(5);
		bound_i32.set(15);
		assert_eq!(bound_i32.get(), 10);
	}

	// Creating a BoundI32 instance with MIN and MAX set to the same value results in a valid instance with that value
	#[test]
	fn test_bound_i32_same_min_max() {
		let bound_i32 = BoundI32::<5, 5>::new(7);
		assert_eq!(bound_i32.get(), 5);
	}

	// Setting the value of a BoundI32 instance with a value greater than the maximum bound results in a valid instance with the maximum bound value
	#[test]
	fn test_bound_i32_set_value_greater_than_maximum_bound() {
		let mut bound_i32 = BoundI32::<-10, 10>::new(5);
		bound_i32.set(15);
		assert_eq!(bound_i32.get(), 10);
	}
}
