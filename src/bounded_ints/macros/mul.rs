macro_rules! bound_mul {
    ($type_name:ty, $inner:ty) => {
        impl<const MIN: $inner, const MAX: $inner> Mul for $type_name {
            type Output = Self;
            fn mul(self, other: Self) -> Self { return Self::new(self.inner_value * other.inner_value); }
        }

        impl<const MIN: $inner, const MAX: $inner> Mul<usize> for $type_name {
            type Output = Self;
            fn mul(self, other: usize) -> Self { return Self::new(self.inner_value * other as $inner); }
        }

        impl<const MIN: $inner, const MAX: $inner> Mul<u32> for $type_name {
            type Output = Self;
            fn mul(self, other: u32) -> Self { return Self::new(self.inner_value * other as $inner); }
        }

        impl<const MIN: $inner, const MAX: $inner> Mul<u64> for $type_name {
            type Output = Self;
            fn mul(self, other: u64) -> Self { return Self::new(self.inner_value * other as $inner); }
        }

        impl<const MIN: $inner, const MAX: $inner> Mul<isize> for $type_name {
            type Output = Self;
            fn mul(self, other: isize) -> Self { return Self::new((self.inner_value as isize * other).saturate_into()); }
        }

        impl<const MIN: $inner, const MAX: $inner> Mul<i32> for $type_name {
            type Output = Self;
            fn mul(self, other: i32) -> Self { return Self::new((self.inner_value as i32 * other).saturate_into()); }
        }

        impl<const MIN: $inner, const MAX: $inner> Mul<i64> for $type_name {
            type Output = Self;
            fn mul(self, other: i64) -> Self { return Self::new((self.inner_value as i64 * other).saturate_into()); }
        }

        impl<const MIN: $inner, const MAX: $inner> MulAssign for $type_name {
            fn mul_assign(&mut self, other: Self) { self.inner_value = (self.inner_value * other.inner_value).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> MulAssign<usize> for $type_name {
            fn mul_assign(&mut self, other: usize) { self.inner_value = (self.inner_value * other as $inner).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> MulAssign<u32> for $type_name {
            fn mul_assign(&mut self, other: u32) { self.inner_value = (self.inner_value * other as $inner).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> MulAssign<u64> for $type_name {
            fn mul_assign(&mut self, other: u64) { self.inner_value = (self.inner_value * other as $inner).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> MulAssign<isize> for $type_name {
            fn mul_assign(&mut self, other: isize) { self.inner_value = SaturateInto::<$inner>::saturate_into(self.inner_value as isize * other).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> MulAssign<i32> for $type_name {
            fn mul_assign(&mut self, other: i32) { self.inner_value = SaturateInto::<$inner>::saturate_into(self.inner_value as i32 * other).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> MulAssign<i64> for $type_name {
            fn mul_assign(&mut self, other: i64) { self.inner_value = SaturateInto::<$inner>::saturate_into(self.inner_value as i64 * other).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> Mul<$type_name> for usize {
            type Output = Self;
            fn mul(self, other: $type_name) -> usize { return (self as $inner * other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> Mul<$type_name> for u32 {
            type Output = Self;
            fn mul(self, other: $type_name) -> u32 { return (self as $inner * other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> Mul<$type_name> for u64 {
            type Output = Self;
            fn mul(self, other: $type_name) -> u64 { return (self as $inner * other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> Mul<$type_name> for isize {
            type Output = Self;
            fn mul(self, other: $type_name) -> isize { return self * other.inner_value as isize; }
        }

        impl<const MIN: $inner, const MAX: $inner> Mul<$type_name> for i32 {
            type Output = Self;
            fn mul(self, other: $type_name) -> i32 { return self * other.inner_value as i32; }
        }

        impl<const MIN: $inner, const MAX: $inner> Mul<$type_name> for i64 {
            type Output = Self;
            fn mul(self, other: $type_name) -> i64 { return self * other.inner_value as i64; }
        }

        impl<const MIN: $inner, const MAX: $inner> MulAssign<$type_name> for usize {
            fn mul_assign(&mut self, other: $type_name) { *self = (*self as $inner * other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> MulAssign<$type_name> for u32 {
            fn mul_assign(&mut self, other: $type_name) { *self = (*self as $inner * other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> MulAssign<$type_name> for u64 {
            fn mul_assign(&mut self, other: $type_name) { *self = (*self as $inner * other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> MulAssign<$type_name> for isize {
            fn mul_assign(&mut self, other: $type_name) { *self *= other.inner_value as isize; }
        }

        impl<const MIN: $inner, const MAX: $inner> MulAssign<$type_name> for i32 {
            fn mul_assign(&mut self, other: $type_name) { *self *= other.inner_value as i32; }
        }

        impl<const MIN: $inner, const MAX: $inner> MulAssign<$type_name> for i64 {
            fn mul_assign(&mut self, other: $type_name) { *self *= other.inner_value as i64; }
        }
    }
}

pub(crate) use bound_mul;