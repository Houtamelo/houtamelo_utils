mod vec_impl;
mod array_impl;
mod slice_impl;
mod hash_set_impl;
mod hash_map_impl;

pub trait IterGenerator<'a, TValue> {
	fn iterate(&'a self) -> impl ExactSizeIterator<Item = TValue>;
}