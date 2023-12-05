mod vec_impl;
mod array_impl;
mod slice_impl;
mod hash_set_impl;
mod hash_map_impl;

pub trait CollectionByRef<'a, TValue> {
	type Return: Iterator<Item = TValue>;
	
	fn iterate(&'a self) -> Self::Return;
}

pub trait CollectionByValue<TValue> {
	type Return: Iterator<Item = TValue>;

	fn iterate_by_value(self) -> Self::Return;
}