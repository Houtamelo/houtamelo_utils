macro_rules! bound_rem {
    ($type_name:ty, $inner:ty) => {
        impl<const MIN: $inner, const MAX: $inner> Rem for $type_name {
            type Output = Self;
            fn rem(self, other: Self) -> Self { return Self::new(self.inner_value % other.inner_value); }
        }

        impl<const MIN: $inner, const MAX: $inner> Rem<usize> for $type_name {
            type Output = Self;
            fn rem(self, other: usize) -> Self { return Self::new(self.inner_value % other as $inner); }
        }

        impl<const MIN: $inner, const MAX: $inner> Rem<u32> for $type_name {
            type Output = Self;
            fn rem(self, other: u32) -> Self { return Self::new(self.inner_value % other as $inner); }
        }

        impl<const MIN: $inner, const MAX: $inner> Rem<u64> for $type_name {
            type Output = Self;
            fn rem(self, other: u64) -> Self { return Self::new(self.inner_value % other as $inner); }
        }

        impl<const MIN: $inner, const MAX: $inner> Rem<isize> for $type_name {
            type Output = Self;
            fn rem(self, other: isize) -> Self { return Self::new((self.inner_value as isize % other).saturate_into()); }
        }

        impl<const MIN: $inner, const MAX: $inner> Rem<i32> for $type_name {
            type Output = Self;
            fn rem(self, other: i32) -> Self { return Self::new((self.inner_value as i32 % other).saturate_into()); }
        }

        impl<const MIN: $inner, const MAX: $inner> Rem<i64> for $type_name {
            type Output = Self;
            fn rem(self, other: i64) -> Self { return Self::new((self.inner_value as i64 % other).saturate_into()); }
        }

        impl<const MIN: $inner, const MAX: $inner> RemAssign for $type_name {
            fn rem_assign(&mut self, other: Self) { self.inner_value = (self.inner_value % other.inner_value).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> RemAssign<usize> for $type_name {
            fn rem_assign(&mut self, other: usize) { self.inner_value = (self.inner_value % other as $inner).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> RemAssign<u32> for $type_name {
            fn rem_assign(&mut self, other: u32) { self.inner_value = (self.inner_value % other as $inner).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> RemAssign<u64> for $type_name {
            fn rem_assign(&mut self, other: u64) { self.inner_value = (self.inner_value % other as $inner).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> RemAssign<isize> for $type_name {
            fn rem_assign(&mut self, other: isize) { self.inner_value = SaturateInto::<$inner>::saturate_into(self.inner_value as isize % other).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> RemAssign<i32> for $type_name {
            fn rem_assign(&mut self, other: i32) { self.inner_value = SaturateInto::<$inner>::saturate_into(self.inner_value as i32 % other).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> RemAssign<i64> for $type_name {
            fn rem_assign(&mut self, other: i64) { self.inner_value = SaturateInto::<$inner>::saturate_into(self.inner_value as i64 % other).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> Rem<$type_name> for usize {
            type Output = Self;
            fn rem(self, other: $type_name) -> usize { return (self as $inner % other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> Rem<$type_name> for u32 {
            type Output = Self;
            fn rem(self, other: $type_name) -> u32 { return (self as $inner % other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> Rem<$type_name> for u64 {
            type Output = Self;
            fn rem(self, other: $type_name) -> u64 { return (self as $inner % other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> Rem<$type_name> for isize {
            type Output = Self;
            fn rem(self, other: $type_name) -> isize { return self % other.inner_value as isize; }
        }

        impl<const MIN: $inner, const MAX: $inner> Rem<$type_name> for i32 {
            type Output = Self;
            fn rem(self, other: $type_name) -> i32 { return self % other.inner_value as i32; }
        }

        impl<const MIN: $inner, const MAX: $inner> Rem<$type_name> for i64 {
            type Output = Self;
            fn rem(self, other: $type_name) -> i64 { return self % other.inner_value as i64; }
        }

        impl<const MIN: $inner, const MAX: $inner> RemAssign<$type_name> for usize {
            fn rem_assign(&mut self, other: $type_name) { *self = (*self as $inner % other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> RemAssign<$type_name> for u32 {
            fn rem_assign(&mut self, other: $type_name) { *self = (*self as $inner % other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> RemAssign<$type_name> for u64 {
            fn rem_assign(&mut self, other: $type_name) { *self = (*self as $inner % other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> RemAssign<$type_name> for isize {
            fn rem_assign(&mut self, other: $type_name) { *self %= other.inner_value as isize; }
        }

        impl<const MIN: $inner, const MAX: $inner> RemAssign<$type_name> for i32 {
            fn rem_assign(&mut self, other: $type_name) { *self %= other.inner_value as i32; }
        }

        impl<const MIN: $inner, const MAX: $inner> RemAssign<$type_name> for i64 {
            fn rem_assign(&mut self, other: $type_name) { *self %= other.inner_value as i64; }
        }
    }
}

pub(crate) use bound_rem;