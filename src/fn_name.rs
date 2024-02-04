pub const fn fn_name<T: ?Sized>(_val: &T) -> &'static str {
	let name = std::any::type_name::<T>();

	let bytes = name.as_bytes();
	let mut index = { bytes.len() - 1 };
	while index > 0 {
		if bytes[index] == b':' {
			let(_, result) = bytes.split_at(index + 1);
			return match std::str::from_utf8(result) {
				Ok(str) => str,
				Err(_) => panic!(),
			};
		}

		index -= 1;
	}

	return name;
}

pub const fn full_fn_name<T: ?Sized>(_val: &T) -> &'static str {
	return std::any::type_name::<T>();
}