#![allow(non_camel_case_types)]

use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign};
use num_traits::FromPrimitive;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct B_u8<const MIN: u8, const MAX: u8> {
	inner_value: u8
}

impl <const MIN: u8, const MAX: u8> B_u8<MIN, MAX> {
	pub fn new(value: u8) -> Self {
		if value < MIN {
			return Self { inner_value: MIN };
		} else if value > MAX {
			return Self { inner_value: MAX };
		} else {
			return Self { inner_value: value };
		}
	}

	pub const fn get(&self) -> u8 { return self.inner_value; }

	pub fn set(&mut self, value: u8) { self.inner_value = value.clamp(MIN, MAX); }
}

impl<const MIN: u8, const MAX: u8> PartialEq for B_u8<MIN, MAX> {
	fn eq(&self, other: &Self) -> bool {
		return self.inner_value == other.inner_value;
	}
}

impl<const MIN: u8, const MAX: u8> Eq for B_u8<MIN, MAX> { }

impl<const MIN: u8, const MAX: u8> PartialEq<u8> for B_u8<MIN, MAX> {
	fn eq(&self, other: &u8) -> bool {
		return self.inner_value == *other;
	}
}

impl<const MIN: u8, const MAX: u8> PartialEq<B_u8<MIN, MAX>> for u8 {
	fn eq(&self, other: &B_u8<MIN, MAX>) -> bool {
		return *self == other.inner_value;
	}
}

impl<const MIN: u8, const MAX: u8> PartialOrd for B_u8<MIN, MAX> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		return self.inner_value.partial_cmp(&other.inner_value);
	}
}

impl<const MIN: u8, const MAX: u8> Ord for B_u8<MIN, MAX> {
	fn cmp(&self, other: &Self) -> Ordering {
		return self.inner_value.cmp(&other.inner_value);
	}
}

impl<const MIN: u8, const MAX: u8> PartialOrd<u8> for B_u8<MIN, MAX> {
	fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
		return self.inner_value.partial_cmp(other);
	}
}

impl<const MIN: u8, const MAX: u8> PartialOrd<B_u8<MIN, MAX>> for u8 {
	fn partial_cmp(&self, other: &B_u8<MIN, MAX>) -> Option<Ordering> {
		return self.partial_cmp(&other.inner_value);
	}
}

impl<const MIN: u8, const MAX: u8> Default for B_u8<MIN, MAX> {
	fn default() -> Self { return Self::new(MIN); }
}

impl<const MIN: u8, const MAX: u8> std::hash::Hash for B_u8<MIN, MAX> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.inner_value.hash(state); }
}

impl<const MIN: u8, const MAX: u8> core::ops::Deref for B_u8<MIN, MAX> {
	type Target = u8;

	fn deref(&self) -> &Self::Target {
		return &self.inner_value;
	}
}

impl<const MIN: u8, const MAX: u8> Add for B_u8<MIN, MAX> {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		return Self::new(u8::saturating_add(self.inner_value, other.inner_value));
	}
}

impl<const MIN: u8, const MAX: u8> Add<u8> for B_u8<MIN, MAX> {
	type Output = u8;

	fn add(self, other: u8) -> Self::Output {
		return u8::saturating_add(self.inner_value, other);
	}
}

impl<const MIN: u8, const MAX: u8> Add<B_u8<MIN, MAX>> for u8 {
	type Output = u8;

	fn add(self, other: B_u8<MIN, MAX>) -> Self::Output {
		return u8::saturating_add(self, other.inner_value);
	}
}

impl<const MIN: u8, const MAX: u8> AddAssign for B_u8<MIN, MAX> {
	fn add_assign(&mut self, other: Self) {
		self.set(u8::saturating_add(self.inner_value, other.inner_value));
	}
}

impl<const MIN: u8, const MAX: u8> AddAssign<u8> for B_u8<MIN, MAX> {
	fn add_assign(&mut self, other: u8) {
		self.set(u8::saturating_add(self.inner_value, other));
	}
}

impl<const MIN: u8, const MAX: u8> AddAssign<B_u8<MIN, MAX>> for u8 {
	fn add_assign(&mut self, other: B_u8<MIN, MAX>) {
		*self = u8::saturating_add(*self, other.inner_value);
	}
}

impl<const MIN: u8, const MAX: u8> Sub for B_u8<MIN, MAX> {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		return Self::new(u8::saturating_sub(self.inner_value, other.inner_value));
	}
}

impl<const MIN: u8, const MAX: u8> Sub<u8> for B_u8<MIN, MAX> {
	type Output = u8;

	fn sub(self, other: u8) -> Self::Output {
		return u8::saturating_sub(self.inner_value, other);
	}
}

impl<const MIN: u8, const MAX: u8> Sub<B_u8<MIN, MAX>> for u8 {
	type Output = u8;

	fn sub(self, other: B_u8<MIN, MAX>) -> Self::Output {
		return u8::saturating_sub(self, other.inner_value);
	}
}

impl<const MIN: u8, const MAX: u8> SubAssign for B_u8<MIN, MAX> {
	fn sub_assign(&mut self, other: Self) {
		self.set(u8::saturating_sub(self.inner_value, other.inner_value));
	}
}

impl<const MIN: u8, const MAX: u8> SubAssign<u8> for B_u8<MIN, MAX> {
	fn sub_assign(&mut self, other: u8) {
		self.set(u8::saturating_sub(self.inner_value, other));
	}
}

impl<const MIN: u8, const MAX: u8> SubAssign<B_u8<MIN, MAX>> for u8 {
	fn sub_assign(&mut self, other: B_u8<MIN, MAX>) {
		*self = u8::saturating_sub(*self, other.inner_value);
	}
}

impl<const MIN: u8, const MAX: u8> Mul for B_u8<MIN, MAX> {
	type Output = Self;

	fn mul(self, other: Self) -> Self::Output {
		return Self::new(u8::saturating_mul(self.inner_value, other.inner_value));
	}
}

impl<const MIN: u8, const MAX: u8> Mul<u8> for B_u8<MIN, MAX> {
	type Output = u8;

	fn mul(self, other: u8) -> Self::Output {
		return u8::saturating_mul(self.inner_value, other);
	}
}

impl<const MIN: u8, const MAX: u8> Mul<B_u8<MIN, MAX>> for u8 {
	type Output = u8;

	fn mul(self, other: B_u8<MIN, MAX>) -> Self::Output {
		return u8::saturating_mul(self, other.inner_value);
	}
}

impl<const MIN: u8, const MAX: u8> MulAssign for B_u8<MIN, MAX> {
	fn mul_assign(&mut self, other: Self) {
		self.set(u8::saturating_mul(self.inner_value, other.inner_value));
	}
}

impl<const MIN: u8, const MAX: u8> MulAssign<u8> for B_u8<MIN, MAX> {
	fn mul_assign(&mut self, other: u8) {
		self.set(u8::saturating_mul(self.inner_value, other));
	}
}

impl<const MIN: u8, const MAX: u8> MulAssign<B_u8<MIN, MAX>> for u8 {
	fn mul_assign(&mut self, other: B_u8<MIN, MAX>) {
		*self = u8::saturating_mul(*self, other.inner_value);
	}
}

impl<const MIN: u8, const MAX: u8> Div for B_u8<MIN, MAX> {
	type Output = Self;

	fn div(self, other: Self) -> Self::Output {
		return Self::new(u8::saturating_div(self.inner_value, other.inner_value));
	}
}

impl<const MIN: u8, const MAX: u8> Div<u8> for B_u8<MIN, MAX> {
	type Output = u8;

	fn div(self, other: u8) -> Self::Output {
		return u8::saturating_div(self.inner_value, other);
	}
}

impl<const MIN: u8, const MAX: u8> Div<B_u8<MIN, MAX>> for u8 {
	type Output = u8;

	fn div(self, other: B_u8<MIN, MAX>) -> Self::Output {
		return u8::saturating_div(self, other.inner_value);
	}
}

impl<const MIN: u8, const MAX: u8> DivAssign for B_u8<MIN, MAX> {
	fn div_assign(&mut self, other: Self) {
		self.set(u8::saturating_div(self.inner_value, other.inner_value));
	}
}

impl<const MIN: u8, const MAX: u8> DivAssign<u8> for B_u8<MIN, MAX> {
	fn div_assign(&mut self, other: u8) {
		self.set(u8::saturating_div(self.inner_value, other));
	}
}

impl<const MIN: u8, const MAX: u8> DivAssign<B_u8<MIN, MAX>> for u8 {
	fn div_assign(&mut self, other: B_u8<MIN, MAX>) {
		*self = u8::saturating_div(*self, other.inner_value);
	}
}

impl<const MIN: u8, const MAX: u8> Rem for B_u8<MIN, MAX> {
	type Output = Self;

	fn rem(self, other: Self) -> Self::Output {
		return Self::new(u8::checked_rem(self.inner_value, other.inner_value).unwrap_or(0));
	}
}

impl<const MIN: u8, const MAX: u8> Rem<u8> for B_u8<MIN, MAX> {
	type Output = u8;

	fn rem(self, other: u8) -> Self::Output {
		return u8::checked_rem(self.inner_value, other).unwrap_or(0);
	}
}

impl<const MIN: u8, const MAX: u8> Rem<B_u8<MIN, MAX>> for u8 {
	type Output = u8;

	fn rem(self, other: B_u8<MIN, MAX>) -> Self::Output {
		return u8::checked_rem(self, other.inner_value).unwrap_or(0);
	}
}

impl<const MIN: u8, const MAX: u8> RemAssign for B_u8<MIN, MAX> {
	fn rem_assign(&mut self, other: Self) {
		self.set(u8::checked_rem(self.inner_value, other.inner_value).unwrap_or(MIN));
	}
}

impl<const MIN: u8, const MAX: u8> RemAssign<u8> for B_u8<MIN, MAX> {
	fn rem_assign(&mut self, other: u8) {
		self.set(u8::checked_rem(self.inner_value, other).unwrap_or(MIN));
	}
}

impl<const MIN: u8, const MAX: u8> RemAssign<B_u8<MIN, MAX>> for u8 {
	fn rem_assign(&mut self, other: B_u8<MIN, MAX>) {
		*self = u8::checked_rem(*self, other.inner_value).unwrap_or(0);
	}
}

impl<const MIN: u8, const MAX: u8> From<u8> for B_u8<MIN, MAX> {
	fn from(value: u8) -> Self {
		return Self::new(value);
	}
}

impl<const MIN: u8, const MAX: u8> From<u16> for B_u8<MIN, MAX> {
	fn from(value: u16) -> Self {
		if let Some(value) = u8::from_u16(value) {
			return Self::new(value);
		} else {
			return Self::new(MAX);
		}
	}
}

impl<const MIN: u8, const MAX: u8> From<u32> for B_u8<MIN, MAX> {
	fn from(value: u32) -> Self {
		if let Some(value) = u8::from_u32(value) {
			return Self::new(value);
		} else {
			return Self::new(MAX);
		}
	}
}

impl<const MIN: u8, const MAX: u8> From<u64> for B_u8<MIN, MAX> {
	fn from(value: u64) -> Self {
		if let Some(value) = u8::from_u64(value) {
			return Self::new(value);
		} else {
			return Self::new(MAX);
		}
	}
}

impl<const MIN: u8, const MAX: u8> From<u128> for B_u8<MIN, MAX> {
	fn from(value: u128) -> Self {
		if let Some(value) = u8::from_u128(value) {
			return Self::new(value);
		} else {
			return Self::new(MAX);
		}
	}
}

impl<const MIN: u8, const MAX: u8> From<i8> for B_u8<MIN, MAX> {
	fn from(value: i8) -> Self {
		if value < 0 {
			return Self::new(MIN);
		} else {
			return Self::new(value as u8);
		}
	}
}

impl<const MIN: u8, const MAX: u8> From<i16> for B_u8<MIN, MAX> {
	fn from(value: i16) -> Self {
		if value < 0 {
			return Self::new(MIN);
		} else if let Some(value) = u8::from_i16(value) {
			return Self::new(value);
		} else {
			return Self::new(MAX);
		}
	}
}

impl<const MIN: u8, const MAX: u8> From<i32> for B_u8<MIN, MAX> {
	fn from(value: i32) -> Self {
		if value < 0 {
			return Self::new(MIN);
		} else if let Some(value) = u8::from_i32(value) {
			return Self::new(value);
		} else {
			return Self::new(MAX);
		}
	}
}

impl<const MIN: u8, const MAX: u8> From<i64> for B_u8<MIN, MAX> {
	fn from(value: i64) -> Self {
		if value < 0 {
			return Self::new(MIN);
		} else if let Some(value) = u8::from_i64(value) {
			return Self::new(value);
		} else {
			return Self::new(MAX);
		}
	}
}

impl<const MIN: u8, const MAX: u8> From<i128> for B_u8<MIN, MAX> {
	fn from(value: i128) -> Self {
		if value < 0 {
			return Self::new(MIN);
		} else if let Some(value) = u8::from_i128(value) {
			return Self::new(value);
		} else {
			return Self::new(MAX);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// The 'new' function creates a B_u8 instance with a value within the specified range.
	#[test]
	fn new() {
		let b_u8 = B_u8::<2, 10>::new(5);
		assert_eq!(b_u8, 5);
		
		let b_u8 = B_u8::<2, 10>::new(0);
		assert_eq!(b_u8, 2);
		
		let b_u8 = B_u8::<0, 10>::new(15);
		assert_eq!(b_u8, 10);
	}

	// The 'get' function returns the inner value of the B_u8 instance.
	#[test]
	fn function_returns_inner_value() {
		let b_u8 = B_u8::<0, 10>::new(5);
		assert_eq!(b_u8, 5);
	}

	// The 'set' function updates the inner value of the B_u8 instance to a new value within the specified range.
	#[test]
	fn set_function_updates_inner_value() {
		let mut b_u8: B_u8<2, 10> = B_u8::new(5);
		assert_eq!(b_u8, 5);

		b_u8.set(7);
		assert_eq!(b_u8, 7);

		b_u8.set(12);
		assert_eq!(b_u8, 10);

		b_u8.set(1);
		assert_eq!(b_u8, 2);
	}

	// Two B_u8 instances with the same inner value are considered equal.
	#[test]
	fn eq_ne() {
		let b1: B_u8<0, 10> = B_u8::new(5);
		let b2: B_u8<0, 10> = B_u8::new(5);
		let b3: B_u8<0, 10> = B_u8::new(10);

		assert_eq!(b1, b2);
		assert_ne!(b1, b3);
	}


	// B_u8 instances can be compared with u8 values for equality.
	#[test]
	fn b_u8_eq_u8() {
		let b_u8: B_u8<0, 10> = B_u8::new(5);
		let u8_value: u8 = 5;

		assert_eq!(b_u8, u8_value);
	}

	// u8 values can be compared with B_u8 instances for equality.
	#[test]
	fn u8_eq_b_u8() {
		let b_u8: B_u8<0, 10> = B_u8::new(5);
		let u8_value: u8 = 5;

		assert_eq!(b_u8, u8_value);
		assert_eq!(u8_value, b_u8);
	}

	// B_u8 instances can be compared with each other for ordering.
	#[test]
	fn ord() {
		let b1: B_u8<0, 10> = B_u8::new(5);
		let b2: B_u8<0, 10> = B_u8::new(7);
		let b3: B_u8<0, 10> = B_u8::new(3);

		assert!(b1 < b2);
		assert!(b2 > b1);
		assert!(b1 <= b2);
		assert!(b2 >= b1);
		assert_ne!(b1, b2);
		assert_ne!(b1, b3);
	}

	// B_u8 instances can be compared with u8 values for ordering.
	#[test]
	fn b_u8_ord_u8() {
		let b1: B_u8<0, 10> = B_u8::new(5);
		let b2: B_u8<0, 10> = B_u8::new(7);
		let b3: B_u8<0, 10> = B_u8::new(3);
		let i1: u8 = 6;
		let i2: u8 = 2;

		assert!(b1 < i1);
		assert!(b2 > i1);
		assert!(b3 > i2);
		assert!(i1 > b3);
	}

	// B_u8 instances can be added to each other, resulting in a new B_u8 instance with the sum of their inner values.
	#[test]
	fn add() {
		let b1: B_u8<0, 10> = B_u8::new(5);
		let b2: B_u8<0, 10> = B_u8::new(3);
		let result = b1 + b2;
		assert_eq!(result, 8);
	}

	// B_u8 instances can be added to u8 values, resulting in a new u8 value with the sum of their inner values.
	#[test]
	fn b_u8_add_u8() {
		let b_u8: B_u8<0, 10> = B_u8::new(5);
		let u8_value: u8 = 3;
		let result = b_u8 + u8_value;
		assert_eq!(result, 8);
	}

	// u8 values can be added to B_u8 instances, resulting in a new u8 value with the sum of their inner values.
	#[test]
	fn u8_add_b_u8() {
		let b_u8: B_u8<0, 10> = B_u8::new(5);
		let u8_value: u8 = 3;
		let result: u8 = b_u8 + u8_value;
		assert_eq!(result, 8);
	}

	// B_u8 instances can be subtracted from each other, resulting in a new B_u8 instance with the difference of their inner values.
	#[test]
	fn sub() {
		let b1: B_u8<0, 10> = B_u8::new(5);
		let b2: B_u8<0, 10> = B_u8::new(3);
		let result = b1 - b2;
		assert_eq!(result, 2);
	}

	// B_u8 instances can be subtracted from u8 values, resulting in a new u8 value with the difference of their inner values.
	#[test]
	fn u8_sub_b_u8() {
		// Create a B_u8 instance with inner value 5
		let b_u8: B_u8<0, 10> = B_u8::new(5);

		// Subtract the B_u8 instance from an u8 value
		let result = 10 - b_u8;

		// Check if the result is the difference of the inner values
		assert_eq!(result, 5);
	}

	// B_u8 instances can be multiplied with each other, resulting in a new B_u8 instance with the product of their inner values.
	#[test]
	fn mult() {
		let b1: B_u8<0, 10> = B_u8::new(2);
		let b2: B_u8<0, 10> = B_u8::new(4);
		let result: B_u8<0, 10> = b1 * b2;
		assert_eq!(result, 8);
	}

	// Creating a B_u8 instance with a value greater than the specified maximum should set the inner value to the maximum.
	#[test]
	fn max() {
		let b_u8: B_u8<0, 5> = B_u8::new(10);
		assert_eq!(b_u8, 5);
	}

	// Setting the inner value of a B_u8 instance to a value greater than the specified maximum should set the inner value to the maximum.
	#[test]
	fn set_max() {
		let mut b_u8: B_u8<0, 10> = B_u8::new(5);
		b_u8.set(15);
		assert_eq!(b_u8.get(), 10);
	}

	// Creating a B_u8 instance with a value less than the specified minimum should set the inner value to the minimum.
	#[test]
	fn new_min() {
		let b_u8 = B_u8::<2, 10>::new(0);
		assert_eq!(b_u8, 2);
	}

	// Setting the inner value of a B_u8 instance to a value less than the specified minimum should set the inner value to the minimum.
	#[test]
	fn set_min() {
		let mut b_u8: B_u8<5, 10> = B_u8::new(6);
		b_u8.set(3);
		assert_eq!(b_u8, 5);
	}
}







