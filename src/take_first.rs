use std::collections::HashSet;

pub trait TakeFirst<T> {
	fn take_any(&mut self) -> Option<T>;
}

impl<T> TakeFirst<T> for Vec<T> {
	fn take_any(&mut self) -> Option<T> {
		return self.pop();
	}
}

impl<T> TakeFirst<T> for HashSet<T>
	where T: Eq + std::hash::Hash {
	fn take_any(&mut self) -> Option<T> {
		return self.extract_if(|_| true).take(1).next();
	}
}

impl<TKey, TValue> TakeFirst<(TKey, TValue)> for std::collections::HashMap<TKey, TValue>
	where TKey: Eq + std::hash::Hash {
	fn take_any(&mut self) -> Option<(TKey, TValue)> {
		return self.extract_if(|_, _| true).take(1).next();
	}
}