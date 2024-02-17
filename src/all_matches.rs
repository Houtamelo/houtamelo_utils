#[macro_export]
/// Checks if all elements in the collection matches the given pattern.
///
/// # Arguments
///
/// * `$collection` - The collection to iterate over.
/// * `$pattern` - The pattern to match against each element.
///
/// # Returns
///
/// Returns `true` if all elements matches the pattern, otherwise `false`.
macro_rules! all_matches {
    ($collection: expr, $pattern: pat) => {
		{ 
			#[allow(unused_imports)]
			use $crate::prelude::IterGenerator;
			$collection.iterate().all(|e| matches!(e, $pattern))
		}
	};
	($collection: ident, $pattern: pat) => {
		{ 
			#[allow(unused_imports)]
			use $crate::prelude::IterGenerator;
			$collection.iterate().all(|e| matches!(e, $pattern))
		}
	}
}