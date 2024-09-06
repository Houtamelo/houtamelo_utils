#[macro_export]
/// Checks if any element in the collection matches the given pattern.
///
/// # Arguments
///
/// * `$collection` - The collection to iterate over.
/// * `$pattern` - The pattern to match against each element.
///
/// # Returns
///
/// Returns `true` if any element matches the pattern, otherwise `false`.
macro_rules! any_matches {
    ($collection: expr, $pattern: pat) => {{
			(&$collection).into_iter().any(|e| matches!(e, $pattern))
    }};
	
	($collection: ident, $pattern: pat) => {{
		(&$collection).into_iter().any(|e| matches!(e, $pattern))
	}};
}

#[cfg(test)]
mod tests {
	#[test]
	fn should_return_true_if_pattern_is_found_in_collection() {
		let collection = vec![1, 2, 3, 4, 5];
		let result = any_matches!(collection, 3);
		assert_eq!(result, true);
	}

	#[test]
	fn should_return_false_if_pattern_is_not_found_in_collection() {
		let collection = vec![1, 2, 3, 4, 5];
		let result = any_matches!(collection, 6);
		assert_eq!(result, false);
	}

	#[test]
	fn should_work_with_custom_pattern() {
		let collection = vec![1, 2, 3, 4, 5];
		let result = any_matches!(collection, 1..=3);
		assert_eq!(result, true);
	}

	#[test]
	fn should_return_false_if_collection_is_empty() {
		let collection: Vec<i32> = vec![];
		let result = any_matches!(collection, 1);
		assert_eq!(result, false);
	}

	#[test]
	fn should_return_true_if_collection_has_one_element_and_it_matches_pattern() {
		let collection = vec![1];
		let result = any_matches!(collection, 1);
		assert_eq!(result, true);
	}

	#[test]
	fn should_return_true_if_pattern_matches_all_elements_in_collection() {
		let collection = vec![1, 1, 1, 1, 1];
		let result = any_matches!(collection, 1);
		assert_eq!(result, true);
	}

	#[test]
	fn should_return_true_if_collection_has_large_number_of_elements_and_pattern_is_found() {
		let collection: Vec<i32> = (1..=1000000).collect();
		let result = any_matches!(collection, 999999);
		assert_eq!(result, true);
	}
}