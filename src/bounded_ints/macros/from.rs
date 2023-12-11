macro_rules! bound_from {
	($type_name:ty, $inner:ty) => {
		impl<const MIN: $inner, const MAX: $inner> From<usize> for $type_name {
			fn from(value: usize) -> Self { return Self::new(value as $inner); }
		}

		impl<const MIN: $inner, const MAX: $inner> From<u32> for $type_name {
			fn from(value: u32) -> Self { return Self::new(value as $inner); }
		}

		impl<const MIN: $inner, const MAX: $inner> From<u64> for $type_name {
			fn from(value: u64) -> Self { return Self::new(value as $inner); }
		}

		impl<const MIN: $inner, const MAX: $inner> From<isize> for $type_name {
			fn from(value: isize) -> Self { return Self::new(value.saturate_into()); }
		}

		impl<const MIN: $inner, const MAX: $inner> From<i32> for $type_name {
			fn from(value: i32) -> Self { return Self::new(value.saturate_into()); }
		}

		impl<const MIN: $inner, const MAX: $inner> From<i64> for $type_name {
			fn from(value: i64) -> Self { return Self::new(value.saturate_into()); }
		}

		impl<const MIN: $inner, const MAX: $inner> From<$type_name> for usize {
			fn from(value: $type_name) -> Self { return value.inner_value.saturate_into(); }
		}

		impl<const MIN: $inner, const MAX: $inner> From<$type_name> for u32 {
			fn from(value: $type_name) -> Self { return value.inner_value.saturate_into(); }
		}

		impl<const MIN: $inner, const MAX: $inner> From<$type_name> for u64 {
			fn from(value: $type_name) -> Self { return value.inner_value.saturate_into(); }
		}

		impl<const MIN: $inner, const MAX: $inner> From<$type_name> for isize {
			fn from(value: $type_name) -> Self { return value.inner_value as isize; }
		}

		impl<const MIN: $inner, const MAX: $inner> From<$type_name> for i32 {
			fn from(value: $type_name) -> Self { return value.inner_value as i32; }
		}

		impl<const MIN: $inner, const MAX: $inner> From<$type_name> for i64 {
			fn from(value: $type_name) -> Self { return value.inner_value as i64; }
		}
	};
}

pub(crate) use bound_from;
