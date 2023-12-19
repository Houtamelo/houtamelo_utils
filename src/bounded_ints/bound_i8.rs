#![allow(non_camel_case_types)]

use num_traits::FromPrimitive;
use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct B_i8<const MIN: i8, const MAX: i8> {
	inner_value: i8
}

impl <const MIN: i8, const MAX: i8> B_i8<MIN, MAX> {
	pub fn new(value: i8) -> Self {
		if value < MIN {
			return Self { inner_value: MIN };
		} else if value > MAX {
			return Self { inner_value: MAX };
		} else {
			return Self { inner_value: value };
		}
	}

	pub const fn get(&self) -> i8 { return self.inner_value; }

	pub fn set(&mut self, value: i8) { self.inner_value = value.clamp(MIN, MAX); }
}

impl<const MIN: i8, const MAX: i8> PartialEq for B_i8<MIN, MAX> {
	fn eq(&self, other: &Self) -> bool {
		return self.inner_value == other.inner_value;
	}
}

impl<const MIN: i8, const MAX: i8> Eq for B_i8<MIN, MAX> { }

impl<const MIN: i8, const MAX: i8> PartialEq<i8> for B_i8<MIN, MAX> {
	fn eq(&self, other: &i8) -> bool {
		return self.inner_value == *other;
	}
}

impl<const MIN: i8, const MAX: i8> PartialEq<B_i8<MIN, MAX>> for i8 {
	fn eq(&self, other: &B_i8<MIN, MAX>) -> bool {
		return *self == other.inner_value;
	}
}

impl<const MIN: i8, const MAX: i8> PartialOrd for B_i8<MIN, MAX> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		return self.inner_value.partial_cmp(&other.inner_value);
	}
}

impl<const MIN: i8, const MAX: i8> Ord for B_i8<MIN, MAX> {
	fn cmp(&self, other: &Self) -> Ordering {
		return self.inner_value.cmp(&other.inner_value);
	}
}

impl<const MIN: i8, const MAX: i8> PartialOrd<i8> for B_i8<MIN, MAX> {
	fn partial_cmp(&self, other: &i8) -> Option<Ordering> {
		return self.inner_value.partial_cmp(other);
	}
}

impl<const MIN: i8, const MAX: i8> PartialOrd<B_i8<MIN, MAX>> for i8 {
	fn partial_cmp(&self, other: &B_i8<MIN, MAX>) -> Option<Ordering> {
		return self.partial_cmp(&other.inner_value);
	}
}

impl<const MIN: i8, const MAX: i8> Default for B_i8<MIN, MAX> {
	fn default() -> Self { return Self::new(MIN); }
}

impl<const MIN: i8, const MAX: i8> std::hash::Hash for B_i8<MIN, MAX> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.inner_value.hash(state); }
}

impl<const MIN: i8, const MAX: i8> core::ops::Deref for B_i8<MIN, MAX> {
	type Target = i8;

	fn deref(&self) -> &Self::Target {
		return &self.inner_value;
	}
}

impl<const MIN: i8, const MAX: i8> Add for B_i8<MIN, MAX> {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		return Self::new(i8::saturating_add(self.inner_value, other.inner_value));
	}
}

impl<const MIN: i8, const MAX: i8> Add<i8> for B_i8<MIN, MAX> {
	type Output = i8;

	fn add(self, other: i8) -> Self::Output {
		return i8::saturating_add(self.inner_value, other);
	}
}

impl<const MIN: i8, const MAX: i8> Add<B_i8<MIN, MAX>> for i8 {
	type Output = i8;

	fn add(self, other: B_i8<MIN, MAX>) -> Self::Output {
		return i8::saturating_add(self, other.inner_value);
	}
}

impl<const MIN: i8, const MAX: i8> AddAssign for B_i8<MIN, MAX> {
	fn add_assign(&mut self, other: Self) {
		self.set(i8::saturating_add(self.inner_value, other.inner_value));
	}
}

impl<const MIN: i8, const MAX: i8> AddAssign<i8> for B_i8<MIN, MAX> {
	fn add_assign(&mut self, other: i8) {
		self.set(i8::saturating_add(self.inner_value, other));
	}
}

impl<const MIN: i8, const MAX: i8> AddAssign<B_i8<MIN, MAX>> for i8 {
	fn add_assign(&mut self, other: B_i8<MIN, MAX>) {
		*self = i8::saturating_add(*self, other.inner_value);
	}
}

impl<const MIN: i8, const MAX: i8> Sub for B_i8<MIN, MAX> {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		return Self::new(i8::saturating_sub(self.inner_value, other.inner_value));
	}
}

impl<const MIN: i8, const MAX: i8> Sub<i8> for B_i8<MIN, MAX> {
	type Output = i8;

	fn sub(self, other: i8) -> Self::Output {
		return i8::saturating_sub(self.inner_value, other);
	}
}

impl<const MIN: i8, const MAX: i8> Sub<B_i8<MIN, MAX>> for i8 {
	type Output = i8;

	fn sub(self, other: B_i8<MIN, MAX>) -> Self::Output {
		return i8::saturating_sub(self, other.inner_value);
	}
}

impl<const MIN: i8, const MAX: i8> SubAssign for B_i8<MIN, MAX> {
	fn sub_assign(&mut self, other: Self) {
		self.set(i8::saturating_sub(self.inner_value, other.inner_value));
	}
}

impl<const MIN: i8, const MAX: i8> SubAssign<i8> for B_i8<MIN, MAX> {
	fn sub_assign(&mut self, other: i8) {
		self.set(i8::saturating_sub(self.inner_value, other));
	}
}

impl<const MIN: i8, const MAX: i8> SubAssign<B_i8<MIN, MAX>> for i8 {
	fn sub_assign(&mut self, other: B_i8<MIN, MAX>) {
		*self = i8::saturating_sub(*self, other.inner_value);
	}
}

impl<const MIN: i8, const MAX: i8> Mul for B_i8<MIN, MAX> {
	type Output = Self;

	fn mul(self, other: Self) -> Self::Output {
		return Self::new(i8::saturating_mul(self.inner_value, other.inner_value));
	}
}

impl<const MIN: i8, const MAX: i8> Mul<i8> for B_i8<MIN, MAX> {
	type Output = i8;

	fn mul(self, other: i8) -> Self::Output {
		return i8::saturating_mul(self.inner_value, other);
	}
}

impl<const MIN: i8, const MAX: i8> Mul<B_i8<MIN, MAX>> for i8 {
	type Output = i8;

	fn mul(self, other: B_i8<MIN, MAX>) -> Self::Output {
		return i8::saturating_mul(self, other.inner_value);
	}
}

impl<const MIN: i8, const MAX: i8> MulAssign for B_i8<MIN, MAX> {
	fn mul_assign(&mut self, other: Self) {
		self.set(i8::saturating_mul(self.inner_value, other.inner_value));
	}
}

impl<const MIN: i8, const MAX: i8> MulAssign<i8> for B_i8<MIN, MAX> {
	fn mul_assign(&mut self, other: i8) {
		self.set(i8::saturating_mul(self.inner_value, other));
	}
}

impl<const MIN: i8, const MAX: i8> MulAssign<B_i8<MIN, MAX>> for i8 {
	fn mul_assign(&mut self, other: B_i8<MIN, MAX>) {
		*self = i8::saturating_mul(*self, other.inner_value);
	}
}

impl<const MIN: i8, const MAX: i8> Div for B_i8<MIN, MAX> {
	type Output = Self;

	fn div(self, other: Self) -> Self::Output {
		return Self::new(i8::saturating_div(self.inner_value, other.inner_value));
	}
}

impl<const MIN: i8, const MAX: i8> Div<i8> for B_i8<MIN, MAX> {
	type Output = i8;

	fn div(self, other: i8) -> Self::Output {
		return i8::saturating_div(self.inner_value, other);
	}
}

impl<const MIN: i8, const MAX: i8> Div<B_i8<MIN, MAX>> for i8 {
	type Output = i8;

	fn div(self, other: B_i8<MIN, MAX>) -> Self::Output {
		return i8::saturating_div(self, other.inner_value);
	}
}

impl<const MIN: i8, const MAX: i8> DivAssign for B_i8<MIN, MAX> {
	fn div_assign(&mut self, other: Self) {
		self.set(i8::saturating_div(self.inner_value, other.inner_value));
	}
}

impl<const MIN: i8, const MAX: i8> DivAssign<i8> for B_i8<MIN, MAX> {
	fn div_assign(&mut self, other: i8) {
		self.set(i8::saturating_div(self.inner_value, other));
	}
}

impl<const MIN: i8, const MAX: i8> DivAssign<B_i8<MIN, MAX>> for i8 {
	fn div_assign(&mut self, other: B_i8<MIN, MAX>) {
		*self = i8::saturating_div(*self, other.inner_value);
	}
}

impl<const MIN: i8, const MAX: i8> Rem for B_i8<MIN, MAX> {
	type Output = Self;

	fn rem(self, other: Self) -> Self::Output {
		return Self::new(i8::checked_rem(self.inner_value, other.inner_value).unwrap_or(0));
	}
}

impl<const MIN: i8, const MAX: i8> Rem<i8> for B_i8<MIN, MAX> {
	type Output = i8;

	fn rem(self, other: i8) -> Self::Output {
		return i8::checked_rem(self.inner_value, other).unwrap_or(0);
	}
}

impl<const MIN: i8, const MAX: i8> Rem<B_i8<MIN, MAX>> for i8 {
	type Output = i8;

	fn rem(self, other: B_i8<MIN, MAX>) -> Self::Output {
		return i8::checked_rem(self, other.inner_value).unwrap_or(0);
	}
}

impl<const MIN: i8, const MAX: i8> RemAssign for B_i8<MIN, MAX> {
	fn rem_assign(&mut self, other: Self) {
		self.set(i8::checked_rem(self.inner_value, other.inner_value).unwrap_or(MIN));
	}
}

impl<const MIN: i8, const MAX: i8> RemAssign<i8> for B_i8<MIN, MAX> {
	fn rem_assign(&mut self, other: i8) {
		self.set(i8::checked_rem(self.inner_value, other).unwrap_or(MIN));
	}
}

impl<const MIN: i8, const MAX: i8> RemAssign<B_i8<MIN, MAX>> for i8 {
	fn rem_assign(&mut self, other: B_i8<MIN, MAX>) {
		*self = i8::checked_rem(*self, other.inner_value).unwrap_or(0);
	}
}

impl<const MIN: i8, const MAX: i8> From<u8> for B_i8<MIN, MAX> {
	fn from(value: u8) -> Self {
		if let Some(value) = i8::from_u8(value) {
			return Self::new(value);
		} else {
			return Self::new(MAX);
		}
	}
}

impl<const MIN: i8, const MAX: i8> From<u16> for B_i8<MIN, MAX> {
	fn from(value: u16) -> Self {
		if let Some(value) = i8::from_u16(value) {
			return Self::new(value);
		} else {
			return Self::new(MAX);
		}
	}
}

impl<const MIN: i8, const MAX: i8> From<u32> for B_i8<MIN, MAX> {
	fn from(value: u32) -> Self {
		if let Some(value) = i8::from_u32(value) {
			return Self::new(value);
		} else {
			return Self::new(MAX);
		}
	}
}

impl<const MIN: i8, const MAX: i8> From<u64> for B_i8<MIN, MAX> {
	fn from(value: u64) -> Self {
		if let Some(value) = i8::from_u64(value) {
			return Self::new(value);
		} else {
			return Self::new(MAX);
		}
	}
}

impl<const MIN: i8, const MAX: i8> From<u128> for B_i8<MIN, MAX> {
	fn from(value: u128) -> Self {
		if let Some(value) = i8::from_u128(value) {
			return Self::new(value);
		} else {
			return Self::new(MAX);
		}
	}
}

impl<const MIN: i8, const MAX: i8> From<i8> for B_i8<MIN, MAX> {
	fn from(value: i8) -> Self {
		return Self::new(value);
	}
}

impl<const MIN: i8, const MAX: i8> From<i16> for B_i8<MIN, MAX> {
	fn from(value: i16) -> Self {
		if value < MIN as i16 {
			return B_i8::<MIN, MAX>::new(MIN);
		} else if value > MAX as i16 {
			return B_i8::<MIN, MAX>::new(MAX);
		} else {
			return B_i8::<MIN, MAX>::new(value as i8);
		}
	}
}

impl<const MIN: i8, const MAX: i8> From<i32> for B_i8<MIN, MAX> {
	fn from(value: i32) -> Self {
		if value < MIN as i32 {
			return B_i8::<MIN, MAX>::new(MIN);
		} else if value > MAX as i32 {
			return B_i8::<MIN, MAX>::new(MAX);
		} else {
			return B_i8::<MIN, MAX>::new(value as i8);
		}
	}
}

impl<const MIN: i8, const MAX: i8> From<i64> for B_i8<MIN, MAX> {
	fn from(value: i64) -> Self {
		if value < MIN as i64 {
			return B_i8::<MIN, MAX>::new(MIN);
		} else if value > MAX as i64 {
			return B_i8::<MIN, MAX>::new(MAX);
		} else {
			return B_i8::<MIN, MAX>::new(value as i8);
		}
	}
}

impl<const MIN: i8, const MAX: i8> From<i128> for B_i8<MIN, MAX> {
	fn from(value: i128) -> Self {
		if value < MIN as i128 {
			return B_i8::<MIN, MAX>::new(MIN);
		} else if value > MAX as i128 {
			return B_i8::<MIN, MAX>::new(MAX);
		} else {
			return B_i8::<MIN, MAX>::new(value as i8);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// The 'new' function creates a B_i8 instance with a value within the specified range.
	#[test]
	fn new() {
		let min = -10;
		let max = 10;
		let value = 5;
		let b_i8 = B_i8::<-10, 10>::new(value);
		assert_eq!(b_i8, value);

		let value = -15;
		let b_i8 = B_i8::<-10, 10>::new(value);
		assert_eq!(b_i8, min);

		let value = 15;
		let b_i8 = B_i8::<-10, 10>::new(value);
		assert_eq!(b_i8, max);
	}

	// The 'get' function returns the inner value of the B_i8 instance.
	#[test]
	fn function_returns_inner_value() {
		let b_i8 = B_i8::<-10, 10>::new(5);
		assert_eq!(b_i8, 5);
	}

	// The 'set' function updates the inner value of the B_i8 instance to a new value within the specified range.
	#[test]
	fn set_function_updates_inner_value() {
		let mut b_i8: B_i8<0, 10> = B_i8::new(5);
		assert_eq!(b_i8, 5);

		b_i8.set(7);
		assert_eq!(b_i8, 7);

		b_i8.set(12);
		assert_eq!(b_i8, 10);

		b_i8.set(-3);
		assert_eq!(b_i8, 0);
	}

	// Two B_i8 instances with the same inner value are considered equal.
	#[test]
	fn eq_ne() {
		let b1: B_i8<0, 10> = B_i8::new(5);
		let b2: B_i8<0, 10> = B_i8::new(5);
		let b3: B_i8<0, 10> = B_i8::new(10);

		assert_eq!(b1, b2);
		assert_ne!(b1, b3);
	}
	

	// B_i8 instances can be compared with i8 values for equality.
	#[test]
	fn b_i8_eq_i8() {
		let b_i8: B_i8<0, 10> = B_i8::new(5);
		let i8_value: i8 = 5;

		assert_eq!(b_i8, i8_value);
	}

	// i8 values can be compared with B_i8 instances for equality.
	#[test]
	fn i8_eq_b_i8() {
		let b_i8: B_i8<0, 10> = B_i8::new(5);
		let i8_value: i8 = 5;

		assert_eq!(b_i8, i8_value);
		assert_eq!(i8_value, b_i8);
	}

	// B_i8 instances can be compared with each other for ordering.
	#[test]
	fn ord() {
		let b1: B_i8<0, 10> = B_i8::new(5);
		let b2: B_i8<0, 10> = B_i8::new(7);
		let b3: B_i8<0, 10> = B_i8::new(3);

		assert!(b1 < b2);
		assert!(b2 > b1);
		assert!(b1 <= b2);
		assert!(b2 >= b1);
		assert_ne!(b1, b2);
		assert_ne!(b1, b3);
	}

	// B_i8 instances can be compared with i8 values for ordering.
	#[test]
	fn b_i8_ord_i8() {
		let b1: B_i8<0, 10> = B_i8::new(5);
		let b2: B_i8<0, 10> = B_i8::new(7);
		let b3: B_i8<0, 10> = B_i8::new(3);
		let i1: i8 = 6;
		let i2: i8 = 2;

		assert!(b1 < i1);
		assert!(b2 > i1);
		assert!(b3 > i2);
		assert!(i1 > b3);
	}

	// B_i8 instances can be added to each other, resulting in a new B_i8 instance with the sum of their inner values.
	#[test]
	fn add() {
		let b1: B_i8<0, 10> = B_i8::new(5);
		let b2: B_i8<0, 10> = B_i8::new(3);
		let result = b1 + b2;
		assert_eq!(result, 8);
	}

	// B_i8 instances can be added to i8 values, resulting in a new i8 value with the sum of their inner values.
	#[test]
	fn b_i8_add_i8() {
		let b_i8: B_i8<0, 10> = B_i8::new(5);
		let i8_value: i8 = 3;
		let result = b_i8 + i8_value;
		assert_eq!(result, 8);
	}

	// i8 values can be added to B_i8 instances, resulting in a new i8 value with the sum of their inner values.
	#[test]
	fn i8_add_b_i8() {
		let b_i8: B_i8<0, 10> = B_i8::new(5);
		let i8_value: i8 = 3;
		let result: i8 = b_i8 + i8_value;
		assert_eq!(result, 8);
	}

	// B_i8 instances can be subtracted from each other, resulting in a new B_i8 instance with the difference of their inner values.
	#[test]
	fn sub() {
		let b1: B_i8<0, 10> = B_i8::new(5);
		let b2: B_i8<0, 10> = B_i8::new(3);
		let result = b1 - b2;
		assert_eq!(result, 2);
	}

	// B_i8 instances can be subtracted from i8 values, resulting in a new i8 value with the difference of their inner values.
	#[test]
	fn i8_sub_b_i8() {
		// Create a B_i8 instance with inner value 5
		let b_i8: B_i8<0, 10> = B_i8::new(5);

		// Subtract the B_i8 instance from an i8 value
		let result = 10 - b_i8;

		// Check if the result is the difference of the inner values
		assert_eq!(result, 5);
	}

	// B_i8 instances can be multiplied with each other, resulting in a new B_i8 instance with the product of their inner values.
	#[test]
	fn mult() {
		let b1: B_i8<0, 10> = B_i8::new(2);
		let b2: B_i8<0, 10> = B_i8::new(4);
		let result: B_i8<0, 10> = b1 * b2;
		assert_eq!(result, 8);
	}

	// Creating a B_i8 instance with a value greater than the specified maximum should set the inner value to the maximum.
	#[test]
	fn max() {
		let b_i8: B_i8<0, 5> = B_i8::new(10);
		assert_eq!(b_i8, 5);
	}

	// Setting the inner value of a B_i8 instance to a value greater than the specified maximum should set the inner value to the maximum.
	#[test]
	fn set_max() {
		let mut b_i8: B_i8<0, 10> = B_i8::new(5);
		b_i8.set(15);
		assert_eq!(b_i8.get(), 10);
	}

	// Creating a B_i8 instance with a value less than the specified minimum should set the inner value to the minimum.
	#[test]
	fn new_min() {
		let b_i8 = B_i8::<-10, 10>::new(-20);
		assert_eq!(b_i8, -10);
	}

	// Setting the inner value of a B_i8 instance to a value less than the specified minimum should set the inner value to the minimum.
	#[test]
	fn set_min() {
		let mut b_i8: B_i8<-10, 10> = B_i8::new(5);
		b_i8.set(-15);
		assert_eq!(b_i8, -10);
	}
}