#![allow(non_camel_case_types)]

use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct B_i16<const MIN: i16, const MAX: i16> {
	inner_value: i16
}

impl <const MIN: i16, const MAX: i16> B_i16<MIN, MAX> {
	pub fn new(value: i16) -> Self {
		if value < MIN {
			return Self { inner_value: MIN };
		} else if value > MAX {
			return Self { inner_value: MAX };
		} else {
			return Self { inner_value: value };
		}
	}

	pub const fn get(&self) -> i16 { return self.inner_value; }

	pub fn set(&mut self, value: i16) { self.inner_value = value.clamp(MIN, MAX); }
}

impl<const MIN: i16, const MAX: i16> PartialEq for B_i16<MIN, MAX> {
	fn eq(&self, other: &Self) -> bool {
		return self.inner_value == other.inner_value;
	}
}

impl<const MIN: i16, const MAX: i16> Eq for B_i16<MIN, MAX> { }

impl<const MIN: i16, const MAX: i16> PartialEq<i16> for B_i16<MIN, MAX> {
	fn eq(&self, other: &i16) -> bool {
		return self.inner_value == *other;
	}
}

impl<const MIN: i16, const MAX: i16> PartialEq<B_i16<MIN, MAX>> for i16 {
	fn eq(&self, other: &B_i16<MIN, MAX>) -> bool {
		return *self == other.inner_value;
	}
}

impl<const MIN: i16, const MAX: i16> PartialOrd for B_i16<MIN, MAX> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		return self.inner_value.partial_cmp(&other.inner_value);
	}
}

impl<const MIN: i16, const MAX: i16> Ord for B_i16<MIN, MAX> {
	fn cmp(&self, other: &Self) -> Ordering {
		return self.inner_value.cmp(&other.inner_value);
	}
}

impl<const MIN: i16, const MAX: i16> PartialOrd<i16> for B_i16<MIN, MAX> {
	fn partial_cmp(&self, other: &i16) -> Option<Ordering> {
		return self.inner_value.partial_cmp(other);
	}
}

impl<const MIN: i16, const MAX: i16> PartialOrd<B_i16<MIN, MAX>> for i16 {
	fn partial_cmp(&self, other: &B_i16<MIN, MAX>) -> Option<Ordering> {
		return self.partial_cmp(&other.inner_value);
	}
}

impl<const MIN: i16, const MAX: i16> Default for B_i16<MIN, MAX> {
	fn default() -> Self { return Self::new(MIN); }
}

impl<const MIN: i16, const MAX: i16> From<i16> for B_i16<MIN, MAX> {
	fn from(value: i16) -> Self {
		if value < MIN {
			return Self { inner_value: MIN };
		} else if value > MAX {
			return Self { inner_value: MAX };
		} else {
			return Self { inner_value: value };
		}
	}
}

impl<const MIN: i16, const MAX: i16> From<B_i16<MIN, MAX>> for i16 {
	fn from(value: B_i16<MIN, MAX>) -> Self { return value.inner_value; }
}

impl<const MIN: i16, const MAX: i16> std::hash::Hash for B_i16<MIN, MAX> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.inner_value.hash(state); }
}

impl<const MIN: i16, const MAX: i16> core::ops::Deref for B_i16<MIN, MAX> {
	type Target = i16;

	fn deref(&self) -> &Self::Target {
		return &self.inner_value;
	}
}

impl<const MIN: i16, const MAX: i16> Add for B_i16<MIN, MAX> {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		return Self::new(i16::saturating_add(self.inner_value, other.inner_value));
	}
}

impl<const MIN: i16, const MAX: i16> Add<i16> for B_i16<MIN, MAX> {
	type Output = i16;

	fn add(self, other: i16) -> Self::Output {
		return i16::saturating_add(self.inner_value, other);
	}
}

impl<const MIN: i16, const MAX: i16> Add<B_i16<MIN, MAX>> for i16 {
	type Output = i16;

	fn add(self, other: B_i16<MIN, MAX>) -> Self::Output {
		return i16::saturating_add(self, other.inner_value);
	}
}

impl<const MIN: i16, const MAX: i16> AddAssign for B_i16<MIN, MAX> {
	fn add_assign(&mut self, other: Self) {
		self.set(i16::saturating_add(self.inner_value, other.inner_value));
	}
}

impl<const MIN: i16, const MAX: i16> AddAssign<i16> for B_i16<MIN, MAX> {
	fn add_assign(&mut self, other: i16) {
		self.set(i16::saturating_add(self.inner_value, other));
	}
}

impl<const MIN: i16, const MAX: i16> AddAssign<B_i16<MIN, MAX>> for i16 {
	fn add_assign(&mut self, other: B_i16<MIN, MAX>) {
		*self = i16::saturating_add(*self, other.inner_value);
	}
}

impl<const MIN: i16, const MAX: i16> Sub for B_i16<MIN, MAX> {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		return Self::new(i16::saturating_sub(self.inner_value, other.inner_value));
	}
}

impl<const MIN: i16, const MAX: i16> Sub<i16> for B_i16<MIN, MAX> {
	type Output = i16;

	fn sub(self, other: i16) -> Self::Output {
		return i16::saturating_sub(self.inner_value, other);
	}
}

impl<const MIN: i16, const MAX: i16> Sub<B_i16<MIN, MAX>> for i16 {
	type Output = i16;

	fn sub(self, other: B_i16<MIN, MAX>) -> Self::Output {
		return i16::saturating_sub(self, other.inner_value);
	}
}

impl<const MIN: i16, const MAX: i16> SubAssign for B_i16<MIN, MAX> {
	fn sub_assign(&mut self, other: Self) {
		self.set(i16::saturating_sub(self.inner_value, other.inner_value));
	}
}

impl<const MIN: i16, const MAX: i16> SubAssign<i16> for B_i16<MIN, MAX> {
	fn sub_assign(&mut self, other: i16) {
		self.set(i16::saturating_sub(self.inner_value, other));
	}
}

impl<const MIN: i16, const MAX: i16> SubAssign<B_i16<MIN, MAX>> for i16 {
	fn sub_assign(&mut self, other: B_i16<MIN, MAX>) {
		*self = i16::saturating_sub(*self, other.inner_value);
	}
}

impl<const MIN: i16, const MAX: i16> Mul for B_i16<MIN, MAX> {
	type Output = Self;

	fn mul(self, other: Self) -> Self::Output {
		return Self::new(i16::saturating_mul(self.inner_value, other.inner_value));
	}
}

impl<const MIN: i16, const MAX: i16> Mul<i16> for B_i16<MIN, MAX> {
	type Output = i16;

	fn mul(self, other: i16) -> Self::Output {
		return i16::saturating_mul(self.inner_value, other);
	}
}

impl<const MIN: i16, const MAX: i16> Mul<B_i16<MIN, MAX>> for i16 {
	type Output = i16;

	fn mul(self, other: B_i16<MIN, MAX>) -> Self::Output {
		return i16::saturating_mul(self, other.inner_value);
	}
}

impl<const MIN: i16, const MAX: i16> MulAssign for B_i16<MIN, MAX> {
	fn mul_assign(&mut self, other: Self) {
		self.set(i16::saturating_mul(self.inner_value, other.inner_value));
	}
}

impl<const MIN: i16, const MAX: i16> MulAssign<i16> for B_i16<MIN, MAX> {
	fn mul_assign(&mut self, other: i16) {
		self.set(i16::saturating_mul(self.inner_value, other));
	}
}

impl<const MIN: i16, const MAX: i16> MulAssign<B_i16<MIN, MAX>> for i16 {
	fn mul_assign(&mut self, other: B_i16<MIN, MAX>) {
		*self = i16::saturating_mul(*self, other.inner_value);
	}
}

impl<const MIN: i16, const MAX: i16> Div for B_i16<MIN, MAX> {
	type Output = Self;

	fn div(self, other: Self) -> Self::Output {
		return Self::new(i16::saturating_div(self.inner_value, other.inner_value));
	}
}

impl<const MIN: i16, const MAX: i16> Div<i16> for B_i16<MIN, MAX> {
	type Output = i16;

	fn div(self, other: i16) -> Self::Output {
		return i16::saturating_div(self.inner_value, other);
	}
}

impl<const MIN: i16, const MAX: i16> Div<B_i16<MIN, MAX>> for i16 {
	type Output = i16;

	fn div(self, other: B_i16<MIN, MAX>) -> Self::Output {
		return i16::saturating_div(self, other.inner_value);
	}
}

impl<const MIN: i16, const MAX: i16> DivAssign for B_i16<MIN, MAX> {
	fn div_assign(&mut self, other: Self) {
		self.set(i16::saturating_div(self.inner_value, other.inner_value));
	}
}

impl<const MIN: i16, const MAX: i16> DivAssign<i16> for B_i16<MIN, MAX> {
	fn div_assign(&mut self, other: i16) {
		self.set(i16::saturating_div(self.inner_value, other));
	}
}

impl<const MIN: i16, const MAX: i16> DivAssign<B_i16<MIN, MAX>> for i16 {
	fn div_assign(&mut self, other: B_i16<MIN, MAX>) {
		*self = i16::saturating_div(*self, other.inner_value);
	}
}

impl<const MIN: i16, const MAX: i16> Rem for B_i16<MIN, MAX> {
	type Output = Self;

	fn rem(self, other: Self) -> Self::Output {
		return Self::new(i16::checked_rem(self.inner_value, other.inner_value).unwrap_or(0));
	}
}

impl<const MIN: i16, const MAX: i16> Rem<i16> for B_i16<MIN, MAX> {
	type Output = i16;

	fn rem(self, other: i16) -> Self::Output {
		return i16::checked_rem(self.inner_value, other).unwrap_or(0);
	}
}

impl<const MIN: i16, const MAX: i16> Rem<B_i16<MIN, MAX>> for i16 {
	type Output = i16;

	fn rem(self, other: B_i16<MIN, MAX>) -> Self::Output {
		return i16::checked_rem(self, other.inner_value).unwrap_or(0);
	}
}

impl<const MIN: i16, const MAX: i16> RemAssign for B_i16<MIN, MAX> {
	fn rem_assign(&mut self, other: Self) {
		self.set(i16::checked_rem(self.inner_value, other.inner_value).unwrap_or(MIN));
	}
}

impl<const MIN: i16, const MAX: i16> RemAssign<i16> for B_i16<MIN, MAX> {
	fn rem_assign(&mut self, other: i16) {
		self.set(i16::checked_rem(self.inner_value, other).unwrap_or(MIN));
	}
}

impl<const MIN: i16, const MAX: i16> RemAssign<B_i16<MIN, MAX>> for i16 {
	fn rem_assign(&mut self, other: B_i16<MIN, MAX>) {
		*self = i16::checked_rem(*self, other.inner_value).unwrap_or(0);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// The 'new' function creates a B_i16 instance with a value within the specified range.
	#[test]
	fn new() {
		let min = -10;
		let max = 10;
		let value = 5;
		let b_i16 = B_i16::<-10, 10>::new(value);
		assert_eq!(b_i16, value);

		let value = -15;
		let b_i16 = B_i16::<-10, 10>::new(value);
		assert_eq!(b_i16, min);

		let value = 15;
		let b_i16 = B_i16::<-10, 10>::new(value);
		assert_eq!(b_i16, max);
	}

	// The 'get' function returns the inner value of the B_i16 instance.
	#[test]
	fn function_returns_inner_value() {
		let b_i16 = B_i16::<-10, 10>::new(5);
		assert_eq!(b_i16, 5);
	}

	// The 'set' function updates the inner value of the B_i16 instance to a new value within the specified range.
	#[test]
	fn set_function_updates_inner_value() {
		let mut b_i16: B_i16<0, 10> = B_i16::new(5);
		assert_eq!(b_i16, 5);

		b_i16.set(7);
		assert_eq!(b_i16, 7);

		b_i16.set(12);
		assert_eq!(b_i16, 10);

		b_i16.set(-3);
		assert_eq!(b_i16, 0);
	}

	// Two B_i16 instances with the same inner value are considered equal.
	#[test]
	fn eq_ne() {
		let b1: B_i16<0, 10> = B_i16::new(5);
		let b2: B_i16<0, 10> = B_i16::new(5);
		let b3: B_i16<0, 10> = B_i16::new(10);

		assert_eq!(b1, b2);
		assert_ne!(b1, b3);
	}


	// B_i16 instances can be compared with i16 values for equality.
	#[test]
	fn b_i16_eq_i16() {
		let b_i16: B_i16<0, 10> = B_i16::new(5);
		let i16_value: i16 = 5;

		assert_eq!(b_i16, i16_value);
	}

	// i16 values can be compared with B_i16 instances for equality.
	#[test]
	fn i16_eq_b_i16() {
		let b_i16: B_i16<0, 10> = B_i16::new(5);
		let i16_value: i16 = 5;

		assert_eq!(b_i16, i16_value);
		assert_eq!(i16_value, b_i16);
	}

	// B_i16 instances can be compared with each other for ordering.
	#[test]
	fn ord() {
		let b1: B_i16<0, 10> = B_i16::new(5);
		let b2: B_i16<0, 10> = B_i16::new(7);
		let b3: B_i16<0, 10> = B_i16::new(3);

		assert!(b1 < b2);
		assert!(b2 > b1);
		assert!(b1 <= b2);
		assert!(b2 >= b1);
		assert_ne!(b1, b2);
		assert_ne!(b1, b3);
	}

	// B_i16 instances can be compared with i16 values for ordering.
	#[test]
	fn b_i16_ord_i16() {
		let b1: B_i16<0, 10> = B_i16::new(5);
		let b2: B_i16<0, 10> = B_i16::new(7);
		let b3: B_i16<0, 10> = B_i16::new(3);
		let i1: i16 = 6;
		let i2: i16 = 2;

		assert!(b1 < i1);
		assert!(b2 > i1);
		assert!(b3 > i2);
		assert!(i1 > b3);
	}

	// B_i16 instances can be added to each other, resulting in a new B_i16 instance with the sum of their inner values.
	#[test]
	fn add() {
		let b1: B_i16<0, 10> = B_i16::new(5);
		let b2: B_i16<0, 10> = B_i16::new(3);
		let result = b1 + b2;
		assert_eq!(result, 8);
	}

	// B_i16 instances can be added to i16 values, resulting in a new i16 value with the sum of their inner values.
	#[test]
	fn b_i16_add_i16() {
		let b_i16: B_i16<0, 10> = B_i16::new(5);
		let i16_value: i16 = 3;
		let result = b_i16 + i16_value;
		assert_eq!(result, 8);
	}

	// i16 values can be added to B_i16 instances, resulting in a new i16 value with the sum of their inner values.
	#[test]
	fn i16_add_b_i16() {
		let b_i16: B_i16<0, 10> = B_i16::new(5);
		let i16_value: i16 = 3;
		let result: i16 = b_i16 + i16_value;
		assert_eq!(result, 8);
	}

	// B_i16 instances can be subtracted from each other, resulting in a new B_i16 instance with the difference of their inner values.
	#[test]
	fn sub() {
		let b1: B_i16<0, 10> = B_i16::new(5);
		let b2: B_i16<0, 10> = B_i16::new(3);
		let result = b1 - b2;
		assert_eq!(result, 2);
	}

	// B_i16 instances can be subtracted from i16 values, resulting in a new i16 value with the difference of their inner values.
	#[test]
	fn i16_sub_b_i16() {
		// Create a B_i16 instance with inner value 5
		let b_i16: B_i16<0, 10> = B_i16::new(5);

		// Subtract the B_i16 instance from an i16 value
		let result = 10 - b_i16;

		// Check if the result is the difference of the inner values
		assert_eq!(result, 5);
	}

	// B_i16 instances can be multiplied with each other, resulting in a new B_i16 instance with the product of their inner values.
	#[test]
	fn mult() {
		let b1: B_i16<0, 10> = B_i16::new(2);
		let b2: B_i16<0, 10> = B_i16::new(4);
		let result: B_i16<0, 10> = b1 * b2;
		assert_eq!(result, 8);
	}

	// Creating a B_i16 instance with a value greater than the specified maximum should set the inner value to the maximum.
	#[test]
	fn max() {
		let b_i16: B_i16<0, 5> = B_i16::new(10);
		assert_eq!(b_i16, 5);
	}

	// Setting the inner value of a B_i16 instance to a value greater than the specified maximum should set the inner value to the maximum.
	#[test]
	fn set_max() {
		let mut b_i16: B_i16<0, 10> = B_i16::new(5);
		b_i16.set(15);
		assert_eq!(b_i16.get(), 10);
	}

	// Creating a B_i16 instance with a value less than the specified minimum should set the inner value to the minimum.
	#[test]
	fn new_min() {
		let b_i16 = B_i16::<-10, 10>::new(-20);
		assert_eq!(b_i16, -10);
	}

	// Setting the inner value of a B_i16 instance to a value less than the specified minimum should set the inner value to the minimum.
	#[test]
	fn set_min() {
		let mut b_i16: B_i16<-10, 10> = B_i16::new(5);
		b_i16.set(-15);
		assert_eq!(b_i16, -10);
	}
}