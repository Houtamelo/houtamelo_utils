use crate::prelude::*;

pub trait TakeFirst<T> {
	fn take_any(&mut self) -> Option<T>;
}

impl<T> TakeFirst<T> for Vec<T> {
	fn take_any(&mut self) -> Option<T> { self.pop() }
}

impl<T> TakeFirst<T> for HashSet<T>
where T: Eq + Hash
{
	fn take_any(&mut self) -> Option<T> {
		self.extract_if(|_| true)
			.take(1)
			.next()
	}
}

impl<TKey, TValue> TakeFirst<(TKey, TValue)> for HashMap<TKey, TValue>
where TKey: Eq + Hash
{
	fn take_any(&mut self) -> Option<(TKey, TValue)> {
		self.extract_if(|_, _| true)
			.take(1)
			.next()
	}
}
