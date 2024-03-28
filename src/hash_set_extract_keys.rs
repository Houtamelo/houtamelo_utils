use std::collections::HashMap;
use std::hash::Hash;

pub trait ExtractKeys {
	type Key;
	type Value;

	fn extract_keys<'a>(self,
	                    keys: impl Iterator<Item = &'a Self::Key>)
	                    -> impl Iterator<Item = Self::Value>
	                    where <Self as ExtractKeys>::Key: 'a;
}

impl<TVal, TKey: PartialEq + Eq + Hash> ExtractKeys for &mut HashMap<TKey, TVal> {
	type Key = TKey;
	type Value = TVal;

	fn extract_keys<'a>(self,
	                    keys: impl Iterator<Item = &'a Self::Key>)
	                    -> impl Iterator<Item = Self::Value>
	                    where <Self as ExtractKeys>::Key: 'a {
		std::iter::from_coroutine(move || {
			for key in keys {
				if let Some(_value) = self.remove(key) {
					yield _value;
				}
			}
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let mut map = [(1, "one"), (2, "two"), (3, "three"), (4, "four"), (5, "five")]
			.iter().cloned().collect::<HashMap<_, _>>();
		let extracted = map.extract_keys(vec![&1, &3, &5].into_iter()).collect::<Vec<_>>();
		assert_eq!(extracted, vec!["one", "three", "five"]);
	}
}