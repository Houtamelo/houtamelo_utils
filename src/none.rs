pub trait None<T> {
	fn none(&mut self, condition: impl FnMut(T) -> bool) -> bool;
}

impl<T, TIter: Iterator<Item = T>> None<T> for TIter {
	fn none(&mut self, mut condition: impl FnMut(T) -> bool) -> bool {
		self.all(|item| !condition(item))
	}
}