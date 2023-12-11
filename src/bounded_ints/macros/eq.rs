macro_rules! bound_eq {
    ($type_name:ty, $inner:ty) => {
        impl<const MIN: $inner, const MAX: $inner> Eq for $type_name {}

        impl<const MIN: $inner, const MAX: $inner> PartialEq for $type_name {
            fn eq(&self, other: &Self) -> bool { return self.inner_value == other.inner_value; }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialEq<usize> for $type_name {
            fn eq(&self, other: &usize) -> bool { return self.inner_value == *other as $inner; }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialEq<u32> for $type_name {
            fn eq(&self, other: &u32) -> bool { return self.inner_value == *other as $inner; }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialEq<u64> for $type_name {
            fn eq(&self, other: &u64) -> bool { return self.inner_value == *other as $inner; }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialEq<isize> for $type_name {
            fn eq(&self, other: &isize) -> bool { return self.inner_value as isize == *other; }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialEq<i32> for $type_name {
            fn eq(&self, other: &i32) -> bool { return self.inner_value as i32 == *other; }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialEq<i64> for $type_name {
            fn eq(&self, other: &i64) -> bool { return self.inner_value as i64 == *other; }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialEq<$type_name> for usize {
            fn eq(&self, other: &$type_name) -> bool { return *self as $inner == other.inner_value; }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialEq<$type_name> for u32 {
            fn eq(&self, other: &$type_name) -> bool { return *self as $inner == other.inner_value; }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialEq<$type_name> for u64 {
            fn eq(&self, other: &$type_name) -> bool { return *self as $inner == other.inner_value; }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialEq<$type_name> for isize {
            fn eq(&self, other: &$type_name) -> bool { return *self == other.inner_value as isize; }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialEq<$type_name> for i32 {
            fn eq(&self, other: &$type_name) -> bool { return *self == other.inner_value as i32; }
        }

        impl<const MIN: $inner, const MAX: $inner> PartialEq<$type_name> for i64 {
            fn eq(&self, other: &$type_name) -> bool { return *self == other.inner_value as i64; }
        }
    };
}

pub(crate) use bound_eq;