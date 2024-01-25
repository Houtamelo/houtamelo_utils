pub trait IsNoneOr<T> {
	fn is_none_or(self, predicate: impl FnOnce(T) -> bool) -> bool;
}

impl<T> IsNoneOr<T> for Option<T> {
	fn is_none_or(self, predicate: impl FnOnce(T) -> bool) -> bool {
		return match self {
			Some(value) => predicate(value),
			None => true,
		};
	}
}

impl<'a, T> IsNoneOr<&'a T> for &'a Option<T> {
	fn is_none_or(self, predicate: impl FnOnce(&'a T) -> bool) -> bool {
		return match self {
			Some(value) => predicate(value),
			None => true,
		};
	}
}

impl<'a, T> IsNoneOr<&'a T> for &'a mut Option<T> {
	fn is_none_or(self, predicate: impl FnOnce(&'a T) -> bool) -> bool {
		return match self {
			Some(value) => predicate(value),
			None => true,
		};
	}
}