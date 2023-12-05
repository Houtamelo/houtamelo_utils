#[macro_export]
macro_rules! any_matches {
    ($collection: ident, $pattern: pat) => {
		{ 
			use $crate::collection_trait::*;
			$collection.iterate().any(|d| matches!(d, $pattern))
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_any() {
		let v = vec![1, 2, 3];
		assert!(any_matches!(v, 2));
	}
}