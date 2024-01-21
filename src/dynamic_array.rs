use std::fmt::Debug;
use std::ops::Deref;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug)]
pub enum DynamicArray<T> where T: Sized + 'static {
	Static(&'static [T]),
	Owned(Vec<T>),
}

impl<T> Deref for DynamicArray<T> where T: Sized + 'static {
	type Target = [T];

	fn deref(&self) -> &Self::Target {
		return match self {
			DynamicArray::Static(array) => array,
			DynamicArray::Owned(vec) => vec.as_slice(),
		}
	}
}

impl<T: Clone> DynamicArray<T> {
	pub fn convert_to_owned(&mut self) -> &mut Vec<T> {
		match self {
			DynamicArray::Owned(vec) => return vec,
			DynamicArray::Static(static_array) => {
				let vec = static_array.iter()
					.map(|item| item.clone())
					.collect();
				*self = DynamicArray::Owned(vec);
				let DynamicArray::Owned(vec) = self 
					else { unreachable!(); };
				return vec;
			}
		}
	}
}

impl<T> AsRef<[T]> for DynamicArray<T> where T: Sized + 'static {
	fn as_ref(&self) -> &[T] {
		return self.deref();
	}
}

impl<T> Serialize for DynamicArray<T> where T: Sized + 'static + Serialize {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
		return self.deref().serialize(serializer);
	}
}

impl<'de, T> Deserialize<'de> for DynamicArray<T> where T: Sized + 'static + Deserialize<'de> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
		return Ok(Self::Owned(<Vec<T> as Deserialize>::deserialize(deserializer)?));
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	// Can create a DynamicArray with a static array
	#[test]
	fn test_create_dynamic_array_with_static_array() {
		let array: &[i32] = &[1, 2, 3, 4, 5];
		let dynamic_array = DynamicArray::Static(array);
		assert_eq!(dynamic_array.deref(), array);
	}

	// Can create a DynamicArray with a vector
	#[test]
	fn test_create_dynamic_array_with_vector() {
		let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
		let dynamic_array = DynamicArray::Owned(vec.clone());
		assert_eq!(dynamic_array.deref(), vec.as_slice());
	}

	// Can create a DynamicArray with an empty static array
	#[test]
	fn test_create_dynamic_array_with_empty_static_array() {
		let array: &[i32] = &[];
		let dynamic_array = DynamicArray::Static(array);
		assert_eq!(dynamic_array.deref(), array);
	}

	// Can create a DynamicArray with an empty vector
	#[test]
	fn test_create_dynamic_array_with_empty_vector() {
		let vec: Vec<i32> = vec![];
		let dynamic_array = DynamicArray::Owned(vec.clone());
		assert_eq!(dynamic_array.deref(), vec.as_slice());
	}


	// DynamicArray can be cloned
	#[test]
	fn test_dynamic_array_clone() {
		let array: &[i32] = &[1, 2, 3, 4, 5];
		let dynamic_array = DynamicArray::Static(array);
		let cloned_array = dynamic_array.clone();
		assert_eq!(dynamic_array.deref(), cloned_array.deref());
	}

	// DynamicArray can be used as a reference to an array
	#[test]
	fn test_dynamic_array_as_array_reference() {
		let array: &[i32] = &[1, 2, 3, 4, 5];
		let dynamic_array = DynamicArray::Static(array);
		assert_eq!(dynamic_array.deref(), array);
	}

	// DynamicArray can be serialized
	#[test]
	fn test_dynamic_array_serialization() {
		let array: &[i32] = &[1, 2, 3, 4, 5];
		let dynamic_array = DynamicArray::Static(array);

		let serialized = ron::to_string(&dynamic_array).unwrap();
		let deserialized: DynamicArray<i32> = ron::from_str(&serialized).unwrap();

		assert_eq!(dynamic_array.deref(), deserialized.deref());
	}
	
	// Can create a DynamicArray with an empty vector and dereference it
	#[test]
	fn test_create_dynamic_array_with_empty_vector_and_dereference() {
		let vec: Vec<i32> = vec![];
		let dynamic_array = DynamicArray::Owned(vec.clone());
		assert_eq!(dynamic_array.deref(), vec.as_slice());
	}

	// Can create a DynamicArray with a vector and dereference it
	#[test]
	fn test_create_dynamic_array_with_vector_and_dereference() {
		let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
		let dynamic_array = DynamicArray::Owned(vec.clone());
		assert_eq!(dynamic_array.deref(), vec.as_slice());
	}

	// Can create a DynamicArray with an empty static array and dereference it
	#[test]
	fn test_create_dynamic_array_with_empty_static_array_and_dereference() {
		let array: &[i32] = &[];
		let dynamic_array = DynamicArray::Static(array);
		assert_eq!(dynamic_array.deref(), array);
	}

	// Can create a DynamicArray with a static array and dereference it
	#[test]
	fn test_create_dynamic_array_with_static_array_and_dereference() {
		let array: &[i32] = &[1, 2, 3, 4, 5];
		let dynamic_array = DynamicArray::Static(array);
		assert_eq!(dynamic_array.deref(), array);
	}

	// DynamicArray can be created with a static array and vector of different types
	#[test]
	fn test_create_dynamic_array_with_different_types() {
		let array: &[i32] = &[1, 2, 3, 4, 5];
		let dynamic_array = DynamicArray::Static(array);
		assert_eq!(dynamic_array.deref(), array);

		let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
		let dynamic_array = DynamicArray::Owned(vec.clone());
		assert_eq!(dynamic_array.deref(), vec.as_slice());

		let array: &[u8] = &[1, 2, 3, 4, 5];
		let dynamic_array = DynamicArray::Static(array);
		assert_eq!(dynamic_array.deref(), array);

		let vec: Vec<u8> = vec![1, 2, 3, 4, 5];
		let dynamic_array = DynamicArray::Owned(vec.clone());
		assert_eq!(dynamic_array.deref(), vec.as_slice());

		let array: &[char] = &['a', 'b', 'c'];
		let dynamic_array = DynamicArray::Static(array);
		assert_eq!(dynamic_array.deref(), array);

		let vec: Vec<char> = vec!['a', 'b', 'c'];
		let dynamic_array = DynamicArray::Owned(vec.clone());
		assert_eq!(dynamic_array.deref(), vec.as_slice());
	}

	// DynamicArray can be created with a static array and vector of different sizes
	#[test]
	fn test_dynamic_array_creation_with_different_sizes() {
		let static_array: &[i32] = &[1, 2, 3];
		let dynamic_array_static = DynamicArray::Static(static_array);
		assert_eq!(dynamic_array_static.deref(), static_array);

		let vector: Vec<i32> = vec![4, 5, 6, 7];
		let dynamic_array_vector = DynamicArray::Owned(vector.clone());
		assert_eq!(dynamic_array_vector.deref(), vector.as_slice());
	}

	// DynamicArray can be created with a static array and vector of different lengths
	#[test]
	fn test_create_dynamic_array_with_different_lengths() {
		let array: &[i32] = &[1, 2, 3, 4, 5];
		let dynamic_array = DynamicArray::Static(array);
		assert_eq!(dynamic_array.deref(), array);

		let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
		let dynamic_array = DynamicArray::Owned(vec.clone());
		assert_eq!(dynamic_array.deref(), vec.as_slice());

		let empty_array: &[i32] = &[];
		let dynamic_array = DynamicArray::Static(empty_array);
		assert_eq!(dynamic_array.deref(), empty_array);

		let empty_vec: Vec<i32> = vec![];
		let dynamic_array = DynamicArray::Owned(empty_vec.clone());
		assert_eq!(dynamic_array.deref(), empty_vec.as_slice());
	}
}