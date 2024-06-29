pub trait Touch<T> {
	fn touch(self, touch:impl FnOnce(&mut T));
}

impl<T> Touch<T> for Option<&mut T> {
	fn touch(self, touch:impl FnOnce(&mut T)) {
		if let Some(value) = self {
			touch(value);
		}
	}
}

impl<T> Touch<T> for &mut Option<&mut T> {
	fn touch(self, touch:impl FnOnce(&mut T)) {
		if let Some(value) = self {
			touch(value);
		}
	}
}

impl<T> Touch<T> for &mut Option<T> {
	fn touch(self, touch:impl FnOnce(&mut T)) {
		if let Some(value) = self {
			touch(value);
		}
	}
}

impl<T, TErr> Touch<T> for Result<&mut T, TErr> {
	fn touch(self, touch:impl FnOnce(&mut T)) {
		if let Ok(value) = self {
			touch(value);
		}
	}
}

impl<T, TErr> Touch<T> for &mut Result<&mut T, TErr> {
	fn touch(self, touch:impl FnOnce(&mut T)) {
		if let Ok(value) = self {
			touch(value);
		}
	}
}

impl<T, TErr> Touch<T> for &mut Result<T, TErr> {
	fn touch(self, touch:impl FnOnce(&mut T)) {
		if let Ok(value) = self {
			touch(value);
		}
	}
}
