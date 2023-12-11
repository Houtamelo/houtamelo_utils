macro_rules! bound_ord {
    ($type_name:ty, $inner:ty) => {
        impl<const MIN: $inner, const MAX: $inner> Ord for $type_name {
            fn cmp(&self, other: &Self) -> Ordering { return self.inner_value.cmp(&other.inner_value); }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialOrd for $type_name {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> { return self.inner_value.partial_cmp(&other.inner_value); }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialOrd<usize> for $type_name {
            fn partial_cmp(&self, other: &usize) -> Option<Ordering> { return self.inner_value.partial_cmp(&(*other as $inner)); }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialOrd<u32> for $type_name {
            fn partial_cmp(&self, other: &u32) -> Option<Ordering> { return self.inner_value.partial_cmp(&(*other as $inner)); }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialOrd<u64> for $type_name {
            fn partial_cmp(&self, other: &u64) -> Option<Ordering> { return self.inner_value.partial_cmp(&(*other as $inner)); }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialOrd<isize> for $type_name {
            fn partial_cmp(&self, other: &isize) -> Option<Ordering> { return (self.inner_value as isize).partial_cmp(other); }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialOrd<i32> for $type_name {
            fn partial_cmp(&self, other: &i32) -> Option<Ordering> { return (self.inner_value as i32).partial_cmp(other); }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialOrd<i64> for $type_name {
            fn partial_cmp(&self, other: &i64) -> Option<Ordering> { return (self.inner_value as i64).partial_cmp(other); }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialOrd<$type_name> for usize {
            fn partial_cmp(&self, other: &$type_name) -> Option<Ordering> { return (*self as $inner).partial_cmp(&other.inner_value); }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialOrd<$type_name> for u32 {
            fn partial_cmp(&self, other: &$type_name) -> Option<Ordering> { return (*self as $inner).partial_cmp(&other.inner_value); }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialOrd<$type_name> for u64 {
            fn partial_cmp(&self, other: &$type_name) -> Option<Ordering> { return (*self as $inner).partial_cmp(&other.inner_value); }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialOrd<$type_name> for isize {
            fn partial_cmp(&self, other: &$type_name) -> Option<Ordering> { return self.partial_cmp(&(other.inner_value as isize)); }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialOrd<$type_name> for i32 {
            fn partial_cmp(&self, other: &$type_name) -> Option<Ordering> { return self.partial_cmp(&(other.inner_value as i32)); }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialOrd<$type_name> for i64 {
            fn partial_cmp(&self, other: &$type_name) -> Option<Ordering> { return self.partial_cmp(&(other.inner_value as i64)); }
        }
    }
}

pub(crate) use bound_ord;