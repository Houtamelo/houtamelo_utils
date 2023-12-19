#![allow(non_camel_case_types)]

use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign};
use num_traits::FromPrimitive;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct B_u16<const MIN: u16, const MAX: u16> {
	inner_value: u16
}

impl <const MIN: u16, const MAX: u16> B_u16<MIN, MAX> {
	pub fn new(value: u16) -> Self {
		if value < MIN {
			return Self { inner_value: MIN };
		} else if value > MAX {
			return Self { inner_value: MAX };
		} else {
			return Self { inner_value: value };
		}
	}

	pub const fn get(&self) -> u16 { return self.inner_value; }

	pub fn set(&mut self, value: u16) { self.inner_value = value.clamp(MIN, MAX); }
}

impl<const MIN: u16, const MAX: u16> PartialEq for B_u16<MIN, MAX> {
	fn eq(&self, other: &Self) -> bool {
		return self.inner_value == other.inner_value;
	}
}

impl<const MIN: u16, const MAX: u16> Eq for B_u16<MIN, MAX> { }

impl<const MIN: u16, const MAX: u16> PartialEq<u16> for B_u16<MIN, MAX> {
	fn eq(&self, other: &u16) -> bool {
		return self.inner_value == *other;
	}
}

impl<const MIN: u16, const MAX: u16> PartialEq<B_u16<MIN, MAX>> for u16 {
	fn eq(&self, other: &B_u16<MIN, MAX>) -> bool {
		return *self == other.inner_value;
	}
}

impl<const MIN: u16, const MAX: u16> PartialOrd for B_u16<MIN, MAX> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		return self.inner_value.partial_cmp(&other.inner_value);
	}
}

impl<const MIN: u16, const MAX: u16> Ord for B_u16<MIN, MAX> {
	fn cmp(&self, other: &Self) -> Ordering {
		return self.inner_value.cmp(&other.inner_value);
	}
}

impl<const MIN: u16, const MAX: u16> PartialOrd<u16> for B_u16<MIN, MAX> {
	fn partial_cmp(&self, other: &u16) -> Option<Ordering> {
		return self.inner_value.partial_cmp(other);
	}
}

impl<const MIN: u16, const MAX: u16> PartialOrd<B_u16<MIN, MAX>> for u16 {
	fn partial_cmp(&self, other: &B_u16<MIN, MAX>) -> Option<Ordering> {
		return self.partial_cmp(&other.inner_value);
	}
}

impl<const MIN: u16, const MAX: u16> Default for B_u16<MIN, MAX> {
	fn default() -> Self { return Self::new(MIN); }
}

impl<const MIN: u16, const MAX: u16> std::hash::Hash for B_u16<MIN, MAX> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.inner_value.hash(state); }
}

impl<const MIN: u16, const MAX: u16> core::ops::Deref for B_u16<MIN, MAX> {
	type Target = u16;

	fn deref(&self) -> &Self::Target {
		return &self.inner_value;
	}
}

impl<const MIN: u16, const MAX: u16> Add for B_u16<MIN, MAX> {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		return Self::new(u16::saturating_add(self.inner_value, other.inner_value));
	}
}

impl<const MIN: u16, const MAX: u16> Add<u16> for B_u16<MIN, MAX> {
	type Output = u16;

	fn add(self, other: u16) -> Self::Output {
		return u16::saturating_add(self.inner_value, other);
	}
}

impl<const MIN: u16, const MAX: u16> Add<B_u16<MIN, MAX>> for u16 {
	type Output = u16;

	fn add(self, other: B_u16<MIN, MAX>) -> Self::Output {
		return u16::saturating_add(self, other.inner_value);
	}
}

impl<const MIN: u16, const MAX: u16> AddAssign for B_u16<MIN, MAX> {
	fn add_assign(&mut self, other: Self) {
		self.set(u16::saturating_add(self.inner_value, other.inner_value));
	}
}

impl<const MIN: u16, const MAX: u16> AddAssign<u16> for B_u16<MIN, MAX> {
	fn add_assign(&mut self, other: u16) {
		self.set(u16::saturating_add(self.inner_value, other));
	}
}

impl<const MIN: u16, const MAX: u16> AddAssign<B_u16<MIN, MAX>> for u16 {
	fn add_assign(&mut self, other: B_u16<MIN, MAX>) {
		*self = u16::saturating_add(*self, other.inner_value);
	}
}

impl<const MIN: u16, const MAX: u16> Sub for B_u16<MIN, MAX> {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		return Self::new(u16::saturating_sub(self.inner_value, other.inner_value));
	}
}

impl<const MIN: u16, const MAX: u16> Sub<u16> for B_u16<MIN, MAX> {
	type Output = u16;

	fn sub(self, other: u16) -> Self::Output {
		return u16::saturating_sub(self.inner_value, other);
	}
}

impl<const MIN: u16, const MAX: u16> Sub<B_u16<MIN, MAX>> for u16 {
	type Output = u16;

	fn sub(self, other: B_u16<MIN, MAX>) -> Self::Output {
		return u16::saturating_sub(self, other.inner_value);
	}
}

impl<const MIN: u16, const MAX: u16> SubAssign for B_u16<MIN, MAX> {
	fn sub_assign(&mut self, other: Self) {
		self.set(u16::saturating_sub(self.inner_value, other.inner_value));
	}
}

impl<const MIN: u16, const MAX: u16> SubAssign<u16> for B_u16<MIN, MAX> {
	fn sub_assign(&mut self, other: u16) {
		self.set(u16::saturating_sub(self.inner_value, other));
	}
}

impl<const MIN: u16, const MAX: u16> SubAssign<B_u16<MIN, MAX>> for u16 {
	fn sub_assign(&mut self, other: B_u16<MIN, MAX>) {
		*self = u16::saturating_sub(*self, other.inner_value);
	}
}

impl<const MIN: u16, const MAX: u16> Mul for B_u16<MIN, MAX> {
	type Output = Self;

	fn mul(self, other: Self) -> Self::Output {
		return Self::new(u16::saturating_mul(self.inner_value, other.inner_value));
	}
}

impl<const MIN: u16, const MAX: u16> Mul<u16> for B_u16<MIN, MAX> {
	type Output = u16;

	fn mul(self, other: u16) -> Self::Output {
		return u16::saturating_mul(self.inner_value, other);
	}
}

impl<const MIN: u16, const MAX: u16> Mul<B_u16<MIN, MAX>> for u16 {
	type Output = u16;

	fn mul(self, other: B_u16<MIN, MAX>) -> Self::Output {
		return u16::saturating_mul(self, other.inner_value);
	}
}

impl<const MIN: u16, const MAX: u16> MulAssign for B_u16<MIN, MAX> {
	fn mul_assign(&mut self, other: Self) {
		self.set(u16::saturating_mul(self.inner_value, other.inner_value));
	}
}

impl<const MIN: u16, const MAX: u16> MulAssign<u16> for B_u16<MIN, MAX> {
	fn mul_assign(&mut self, other: u16) {
		self.set(u16::saturating_mul(self.inner_value, other));
	}
}

impl<const MIN: u16, const MAX: u16> MulAssign<B_u16<MIN, MAX>> for u16 {
	fn mul_assign(&mut self, other: B_u16<MIN, MAX>) {
		*self = u16::saturating_mul(*self, other.inner_value);
	}
}

impl<const MIN: u16, const MAX: u16> Div for B_u16<MIN, MAX> {
	type Output = Self;

	fn div(self, other: Self) -> Self::Output {
		return Self::new(u16::saturating_div(self.inner_value, other.inner_value));
	}
}

impl<const MIN: u16, const MAX: u16> Div<u16> for B_u16<MIN, MAX> {
	type Output = u16;

	fn div(self, other: u16) -> Self::Output {
		return u16::saturating_div(self.inner_value, other);
	}
}

impl<const MIN: u16, const MAX: u16> Div<B_u16<MIN, MAX>> for u16 {
	type Output = u16;

	fn div(self, other: B_u16<MIN, MAX>) -> Self::Output {
		return u16::saturating_div(self, other.inner_value);
	}
}

impl<const MIN: u16, const MAX: u16> DivAssign for B_u16<MIN, MAX> {
	fn div_assign(&mut self, other: Self) {
		self.set(u16::saturating_div(self.inner_value, other.inner_value));
	}
}

impl<const MIN: u16, const MAX: u16> DivAssign<u16> for B_u16<MIN, MAX> {
	fn div_assign(&mut self, other: u16) {
		self.set(u16::saturating_div(self.inner_value, other));
	}
}

impl<const MIN: u16, const MAX: u16> DivAssign<B_u16<MIN, MAX>> for u16 {
	fn div_assign(&mut self, other: B_u16<MIN, MAX>) {
		*self = u16::saturating_div(*self, other.inner_value);
	}
}

impl<const MIN: u16, const MAX: u16> Rem for B_u16<MIN, MAX> {
	type Output = Self;

	fn rem(self, other: Self) -> Self::Output {
		return Self::new(u16::checked_rem(self.inner_value, other.inner_value).unwrap_or(0));
	}
}

impl<const MIN: u16, const MAX: u16> Rem<u16> for B_u16<MIN, MAX> {
	type Output = u16;

	fn rem(self, other: u16) -> Self::Output {
		return u16::checked_rem(self.inner_value, other).unwrap_or(0);
	}
}

impl<const MIN: u16, const MAX: u16> Rem<B_u16<MIN, MAX>> for u16 {
	type Output = u16;

	fn rem(self, other: B_u16<MIN, MAX>) -> Self::Output {
		return u16::checked_rem(self, other.inner_value).unwrap_or(0);
	}
}

impl<const MIN: u16, const MAX: u16> RemAssign for B_u16<MIN, MAX> {
	fn rem_assign(&mut self, other: Self) {
		self.set(u16::checked_rem(self.inner_value, other.inner_value).unwrap_or(MIN));
	}
}

impl<const MIN: u16, const MAX: u16> RemAssign<u16> for B_u16<MIN, MAX> {
	fn rem_assign(&mut self, other: u16) {
		self.set(u16::checked_rem(self.inner_value, other).unwrap_or(MIN));
	}
}

impl<const MIN: u16, const MAX: u16> RemAssign<B_u16<MIN, MAX>> for u16 {
	fn rem_assign(&mut self, other: B_u16<MIN, MAX>) {
		*self = u16::checked_rem(*self, other.inner_value).unwrap_or(0);
	}
}

impl<const MIN: u16, const MAX: u16> From<u8> for B_u16<MIN, MAX> {
	fn from(value: u8) -> Self {
		return Self::new(value as u16);
	}
}

impl<const MIN: u16, const MAX: u16> From<u16> for B_u16<MIN, MAX> {
	fn from(value: u16) -> Self {
		return Self::new(value);
	}
}

impl<const MIN: u16, const MAX: u16> From<u32> for B_u16<MIN, MAX> {
	fn from(value: u32) -> Self {
		if let Some(value) = u16::from_u32(value) {
			return Self::new(value);
		} else {
			return Self::new(MAX);
		}
	}
}

impl<const MIN: u16, const MAX: u16> From<u64> for B_u16<MIN, MAX> {
	fn from(value: u64) -> Self {
		if let Some(value) = u16::from_u64(value) {
			return Self::new(value);
		} else {
			return Self::new(MAX);
		}
	}
}

impl<const MIN: u16, const MAX: u16> From<u128> for B_u16<MIN, MAX> {
	fn from(value: u128) -> Self {
		if let Some(value) = u16::from_u128(value) {
			return Self::new(value);
		} else {
			return Self::new(MAX);
		}
	}
}

impl<const MIN: u16, const MAX: u16> From<i8> for B_u16<MIN, MAX> {
	fn from(value: i8) -> Self {
		if value < 0 {
			return Self::new(MIN);
		} else {
			return Self::new(value as u16);
		}
	}
}

impl<const MIN: u16, const MAX: u16> From<i16> for B_u16<MIN, MAX> {
	fn from(value: i16) -> Self {
		if value < 0 {
			return Self::new(MIN);
		} else {
			return Self::new(value as u16);
		}
	}
}

impl<const MIN: u16, const MAX: u16> From<i32> for B_u16<MIN, MAX> {
	fn from(value: i32) -> Self {
		if value < 0 {
			return Self::new(MIN);
		} else if let Some(value) = u16::from_i32(value) {
			return Self::new(value);
		} else {
			return Self::new(MAX);
		}
	}
}

impl<const MIN: u16, const MAX: u16> From<i64> for B_u16<MIN, MAX> {
	fn from(value: i64) -> Self {
		if value < 0 {
			return Self::new(MIN);
		} else if let Some(value) = u16::from_i64(value) {
			return Self::new(value);
		} else {
			return Self::new(MAX);
		}
	}
}

impl<const MIN: u16, const MAX: u16> From<i128> for B_u16<MIN, MAX> {
	fn from(value: i128) -> Self {
		if value < 0 {
			return Self::new(MIN);
		} else if let Some(value) = u16::from_i128(value) {
			return Self::new(value);
		} else {
			return Self::new(MAX);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// The 'new' function creates a B_u16 instance with a value within the specified range.
	#[test]
	fn new() {
		let b_u16 = B_u16::<2, 10>::new(5);
		assert_eq!(b_u16, 5);

		let b_u16 = B_u16::<2, 10>::new(0);
		assert_eq!(b_u16, 2);

		let b_u16 = B_u16::<0, 10>::new(15);
		assert_eq!(b_u16, 10);
	}

	// The 'get' function returns the inner value of the B_u16 instance.
	#[test]
	fn function_returns_inner_value() {
		let b_u16 = B_u16::<0, 10>::new(5);
		assert_eq!(b_u16, 5);
	}

	// The 'set' function updates the inner value of the B_u16 instance to a new value within the specified range.
	#[test]
	fn set_function_updates_inner_value() {
		let mut b_u16: B_u16<2, 10> = B_u16::new(5);
		assert_eq!(b_u16, 5);

		b_u16.set(7);
		assert_eq!(b_u16, 7);

		b_u16.set(12);
		assert_eq!(b_u16, 10);

		b_u16.set(1);
		assert_eq!(b_u16, 2);
	}

	// Two B_u16 instances with the same inner value are considered equal.
	#[test]
	fn eq_ne() {
		let b1: B_u16<0, 10> = B_u16::new(5);
		let b2: B_u16<0, 10> = B_u16::new(5);
		let b3: B_u16<0, 10> = B_u16::new(10);

		assert_eq!(b1, b2);
		assert_ne!(b1, b3);
	}


	// B_u16 instances can be compared with u16 values for equality.
	#[test]
	fn b_u16_eq_u16() {
		let b_u16: B_u16<0, 10> = B_u16::new(5);
		let u16_value: u16 = 5;

		assert_eq!(b_u16, u16_value);
	}

	// u16 values can be compared with B_u16 instances for equality.
	#[test]
	fn u16_eq_b_u16() {
		let b_u16: B_u16<0, 10> = B_u16::new(5);
		let u16_value: u16 = 5;

		assert_eq!(b_u16, u16_value);
		assert_eq!(u16_value, b_u16);
	}

	// B_u16 instances can be compared with each other for ordering.
	#[test]
	fn ord() {
		let b1: B_u16<0, 10> = B_u16::new(5);
		let b2: B_u16<0, 10> = B_u16::new(7);
		let b3: B_u16<0, 10> = B_u16::new(3);

		assert!(b1 < b2);
		assert!(b2 > b1);
		assert!(b1 <= b2);
		assert!(b2 >= b1);
		assert_ne!(b1, b2);
		assert_ne!(b1, b3);
	}

	// B_u16 instances can be compared with u16 values for ordering.
	#[test]
	fn b_u16_ord_u16() {
		let b1: B_u16<0, 10> = B_u16::new(5);
		let b2: B_u16<0, 10> = B_u16::new(7);
		let b3: B_u16<0, 10> = B_u16::new(3);
		let i1: u16 = 6;
		let i2: u16 = 2;

		assert!(b1 < i1);
		assert!(b2 > i1);
		assert!(b3 > i2);
		assert!(i1 > b3);
	}

	// B_u16 instances can be added to each other, resulting in a new B_u16 instance with the sum of their inner values.
	#[test]
	fn add() {
		let b1: B_u16<0, 10> = B_u16::new(5);
		let b2: B_u16<0, 10> = B_u16::new(3);
		let result = b1 + b2;
		assert_eq!(result, 8);
	}

	// B_u16 instances can be added to u16 values, resulting in a new u16 value with the sum of their inner values.
	#[test]
	fn b_u16_add_u16() {
		let b_u16: B_u16<0, 10> = B_u16::new(5);
		let u16_value: u16 = 3;
		let result = b_u16 + u16_value;
		assert_eq!(result, 8);
	}

	// u16 values can be added to B_u16 instances, resulting in a new u16 value with the sum of their inner values.
	#[test]
	fn u16_add_b_u16() {
		let b_u16: B_u16<0, 10> = B_u16::new(5);
		let u16_value: u16 = 3;
		let result: u16 = b_u16 + u16_value;
		assert_eq!(result, 8);
	}

	// B_u16 instances can be subtracted from each other, resulting in a new B_u16 instance with the difference of their inner values.
	#[test]
	fn sub() {
		let b1: B_u16<0, 10> = B_u16::new(5);
		let b2: B_u16<0, 10> = B_u16::new(3);
		let result = b1 - b2;
		assert_eq!(result, 2);
	}

	// B_u16 instances can be subtracted from u16 values, resulting in a new u16 value with the difference of their inner values.
	#[test]
	fn u16_sub_b_u16() {
		// Create a B_u16 instance with inner value 5
		let b_u16: B_u16<0, 10> = B_u16::new(5);

		// Subtract the B_u16 instance from an u16 value
		let result = 10 - b_u16;

		// Check if the result is the difference of the inner values
		assert_eq!(result, 5);
	}

	// B_u16 instances can be multiplied with each other, resulting in a new B_u16 instance with the product of their inner values.
	#[test]
	fn mult() {
		let b1: B_u16<0, 10> = B_u16::new(2);
		let b2: B_u16<0, 10> = B_u16::new(4);
		let result: B_u16<0, 10> = b1 * b2;
		assert_eq!(result, 8);
	}

	// Creating a B_u16 instance with a value greater than the specified maximum should set the inner value to the maximum.
	#[test]
	fn max() {
		let b_u16: B_u16<0, 5> = B_u16::new(10);
		assert_eq!(b_u16, 5);
	}

	// Setting the inner value of a B_u16 instance to a value greater than the specified maximum should set the inner value to the maximum.
	#[test]
	fn set_max() {
		let mut b_u16: B_u16<0, 10> = B_u16::new(5);
		b_u16.set(15);
		assert_eq!(b_u16.get(), 10);
	}

	// Creating a B_u16 instance with a value less than the specified minimum should set the inner value to the minimum.
	#[test]
	fn new_min() {
		let b_u16 = B_u16::<2, 10>::new(0);
		assert_eq!(b_u16, 2);
	}

	// Setting the inner value of a B_u16 instance to a value less than the specified minimum should set the inner value to the minimum.
	#[test]
	fn set_min() {
		let mut b_u16: B_u16<5, 10> = B_u16::new(6);
		b_u16.set(3);
		assert_eq!(b_u16, 5);
	}
}