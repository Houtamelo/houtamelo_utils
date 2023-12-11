macro_rules! bound_div {
    ($type_name:ty, $inner:ty) => {
        impl<const MIN: $inner, const MAX: $inner> Div for $type_name {
            type Output = Self;
            fn div(self, other: Self) -> Self { return Self::new(self.inner_value / other.inner_value); }
        }

        impl<const MIN: $inner, const MAX: $inner> Div<usize> for $type_name {
            type Output = Self;
            fn div(self, other: usize) -> Self { return Self::new(self.inner_value / other as $inner); }
        }

        impl<const MIN: $inner, const MAX: $inner> Div<u32> for $type_name {
            type Output = Self;
            fn div(self, other: u32) -> Self { return Self::new(self.inner_value / other as $inner); }
        }

        impl<const MIN: $inner, const MAX: $inner> Div<u64> for $type_name {
            type Output = Self;
            fn div(self, other: u64) -> Self { return Self::new(self.inner_value / other as $inner); }
        }

        impl<const MIN: $inner, const MAX: $inner> Div<isize> for $type_name {
            type Output = Self;
            fn div(self, other: isize) -> Self { return Self::new((self.inner_value as isize / other).saturate_into()); }
        }

        impl<const MIN: $inner, const MAX: $inner> Div<i32> for $type_name {
            type Output = Self;
            fn div(self, other: i32) -> Self { return Self::new((self.inner_value as i32 / other).saturate_into()); }
        }

        impl<const MIN: $inner, const MAX: $inner> Div<i64> for $type_name {
            type Output = Self;
            fn div(self, other: i64) -> Self { return Self::new((self.inner_value as i64 / other).saturate_into()); }
        }

        impl<const MIN: $inner, const MAX: $inner> DivAssign for $type_name {
            fn div_assign(&mut self, other: Self) { self.inner_value = (self.inner_value / other.inner_value).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> DivAssign<usize> for $type_name {
            fn div_assign(&mut self, other: usize) { self.inner_value = (self.inner_value / other as $inner).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> DivAssign<u32> for $type_name {
            fn div_assign(&mut self, other: u32) { self.inner_value = (self.inner_value / other as $inner).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> DivAssign<u64> for $type_name {
            fn div_assign(&mut self, other: u64) { self.inner_value = (self.inner_value / other as $inner).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> DivAssign<isize> for $type_name {
            fn div_assign(&mut self, other: isize) { self.inner_value = SaturateInto::<$inner>::saturate_into(self.inner_value as isize / other).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> DivAssign<i32> for $type_name {
            fn div_assign(&mut self, other: i32) { self.inner_value = SaturateInto::<$inner>::saturate_into(self.inner_value as i32 / other).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> DivAssign<i64> for $type_name {
            fn div_assign(&mut self, other: i64) { self.inner_value = SaturateInto::<$inner>::saturate_into(self.inner_value as i64 / other).clamp(MIN, MAX); }
        }

        impl<const MIN: $inner, const MAX: $inner> Div<$type_name> for usize {
            type Output = Self;
            fn div(self, other: $type_name) -> usize { return (self as $inner / other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> Div<$type_name> for u32 {
            type Output = Self;
            fn div(self, other: $type_name) -> u32 { return (self as $inner / other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> Div<$type_name> for u64 {
            type Output = Self;
            fn div(self, other: $type_name) -> u64 { return (self as $inner / other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> Div<$type_name> for isize {
            type Output = Self;
            fn div(self, other: $type_name) -> isize { return self / other.inner_value as isize; }
        }

        impl<const MIN: $inner, const MAX: $inner> Div<$type_name> for i32 {
            type Output = Self;
            fn div(self, other: $type_name) -> i32 { return self / other.inner_value as i32; }
        }

        impl<const MIN: $inner, const MAX: $inner> Div<$type_name> for i64 {
            type Output = Self;
            fn div(self, other: $type_name) -> i64 { return self / other.inner_value as i64; }
        }

        impl<const MIN: $inner, const MAX: $inner> DivAssign<$type_name> for usize {
            fn div_assign(&mut self, other: $type_name) { *self = (*self as $inner / other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> DivAssign<$type_name> for u32 {
            fn div_assign(&mut self, other: $type_name) { *self = (*self as $inner / other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> DivAssign<$type_name> for u64 {
            fn div_assign(&mut self, other: $type_name) { *self = (*self as $inner / other.inner_value).saturate_into(); }
        }

        impl<const MIN: $inner, const MAX: $inner> DivAssign<$type_name> for isize {
            fn div_assign(&mut self, other: $type_name) { *self /= other.inner_value as isize; }
        }

        impl<const MIN: $inner, const MAX: $inner> DivAssign<$type_name> for i32 {
            fn div_assign(&mut self, other: $type_name) { *self /= other.inner_value as i32; }
        }

        impl<const MIN: $inner, const MAX: $inner> DivAssign<$type_name> for i64 {
            fn div_assign(&mut self, other: $type_name) { *self /= other.inner_value as i64; }
        }
    }
}

pub(crate) use bound_div;