macro_rules! bound_sub {
    ($type_name:ty, $inner:ty) => {
        impl<const MIN: $inner, const MAX: $inner> Sub for $type_name {
            type Output = Self;
            fn sub(self, other: Self) -> Self { return Self::new(self.inner_value - other.inner_value); }
        }

        impl<const MIN: $inner, const MAX: $inner> Sub<usize> for $type_name {
            type Output = Self;
            fn sub(self, other: usize) -> Self { return Self::new(self.inner_value - other as $inner); }
        }

        impl<const MIN: $inner, const MAX: $inner> Sub<u32> for $type_name {
            type Output = Self;
            fn sub(self, other: u32) -> Self { return Self::new(self.inner_value - other as $inner); }
        }

        impl<const MIN: $inner, const MAX: $inner> Sub<u64> for $type_name {
            type Output = Self;
            fn sub(self, other: u64) -> Self { return Self::new(self.inner_value - other as $inner); }
        }

        impl<const MIN: $inner, const MAX: $inner> Sub<isize> for $type_name {
            type Output = Self;
            fn sub(self, other: isize) -> Self { return Self::new((self.inner_value as isize - other).saturate_into()); }
        }

        impl<const MIN: $inner, const MAX: $inner> Sub<i32> for $type_name {
            type Output = Self;
            fn sub(self, other: i32) -> Self { return Self::new((self.inner_value as i32 - other).saturate_into()); }
        }

        impl<const MIN: $inner, const MAX: $inner> Sub<i64> for $type_name {
            type Output = Self;
            fn sub(self, other: i64) -> Self { return Self::new((self.inner_value as i64 - other).saturate_into()); }
        }

        impl<const MIN: $inner, const MAX: $inner> SubAssign for $type_name {
            fn sub_assign(&mut self, other: Self) { self.inner_value = (self.inner_value - other.inner_value).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> SubAssign<usize> for $type_name {
            fn sub_assign(&mut self, other: usize) { self.inner_value = (self.inner_value - other as $inner).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> SubAssign<u32> for $type_name {
            fn sub_assign(&mut self, other: u32) { self.inner_value = (self.inner_value - other as $inner).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> SubAssign<u64> for $type_name {
            fn sub_assign(&mut self, other: u64) { self.inner_value = (self.inner_value - other as $inner).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> SubAssign<isize> for $type_name {
            fn sub_assign(&mut self, other: isize) { self.inner_value = SaturateInto::<$inner>::saturate_into(self.inner_value as isize - other).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> SubAssign<i32> for $type_name {
            fn sub_assign(&mut self, other: i32) { self.inner_value = SaturateInto::<$inner>::saturate_into(self.inner_value as i32 - other).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> SubAssign<i64> for $type_name {
            fn sub_assign(&mut self, other: i64) { self.inner_value = SaturateInto::<$inner>::saturate_into(self.inner_value as i64 - other).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> Sub<$type_name> for usize {
            type Output = Self;
            fn sub(self, other: $type_name) -> usize { return (self as $inner - other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> Sub<$type_name> for u32 {
            type Output = Self;
            fn sub(self, other: $type_name) -> u32 { return (self as $inner - other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> Sub<$type_name> for u64 {
            type Output = Self;
            fn sub(self, other: $type_name) -> u64 { return (self as $inner - other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> Sub<$type_name> for isize {
            type Output = Self;
            fn sub(self, other: $type_name) -> isize { return self - other.inner_value as isize; }
        }

        impl<const MIN: $inner, const MAX: $inner> Sub<$type_name> for i32 {
            type Output = Self;
            fn sub(self, other: $type_name) -> i32 { return self - other.inner_value as i32; }
        }

        impl<const MIN: $inner, const MAX: $inner> Sub<$type_name> for i64 {
            type Output = Self;
            fn sub(self, other: $type_name) -> i64 { return self - other.inner_value as i64; }
        }

        impl<const MIN: $inner, const MAX: $inner> SubAssign<$type_name> for usize {
            fn sub_assign(&mut self, other: $type_name) { *self = (*self as $inner - other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> SubAssign<$type_name> for u32 {
            fn sub_assign(&mut self, other: $type_name) { *self = (*self as $inner - other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> SubAssign<$type_name> for u64 {
            fn sub_assign(&mut self, other: $type_name) { *self = (*self as $inner - other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> SubAssign<$type_name> for isize {
            fn sub_assign(&mut self, other: $type_name) { *self -= other.inner_value as isize; }
        }

        impl<const MIN: $inner, const MAX: $inner> SubAssign<$type_name> for i32 {
            fn sub_assign(&mut self, other: $type_name) { *self -= other.inner_value as i32; }
        }

        impl<const MIN: $inner, const MAX: $inner> SubAssign<$type_name> for i64 {
            fn sub_assign(&mut self, other: $type_name) { *self -= other.inner_value as i64; }
        }
    }
}

pub(crate) use bound_sub;