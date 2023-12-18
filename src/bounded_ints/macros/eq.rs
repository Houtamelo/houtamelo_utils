macro_rules! bound_eq {
    ($type_name:ty, $inner:ty) => {
        impl<const MIN: $inner, const MAX: $inner> Eq for $type_name {}

        impl<const MIN: $inner, const MAX: $inner> PartialEq for $type_name {
            fn eq(&self, other: &Self) -> bool { return self.inner_value == other.inner_value; }
        }

        impl<const MIN: $inner, const MAX: $inner, T> PartialEq<T> for $type_name where T: SaturateInto<$inner> + Copy {
        	fn eq(&self, other: &T) -> bool {
		        return self.inner_value == <T as SaturateInto<$inner>>::saturate_into(*other);
	        }
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