#[macro_export]
/// Checks if all elements in the collection do not match the given pattern.
///
/// # Arguments
///
/// * `$collection` - The collection to iterate over.
/// * `$pattern` - The pattern to match against each element.
///
/// # Returns
///
/// Returns `true` if all elements do not match the pattern, otherwise `false`.
macro_rules! no_matches {
    ($collection: expr, $pattern: pat) => {
		{ 
			#[allow(unused_imports)]
			use $crate::prelude::IterGenerator;
			$collection.iterate().all(|e| false == matches!(e, $pattern))
		}
	};
	($collection: ident, $pattern: pat) => {
		{ 
			#[allow(unused_imports)]
			use $crate::prelude::IterGenerator;
			$collection.iterate().all(|e| false == matches!(e, $pattern))
		}
	}
}