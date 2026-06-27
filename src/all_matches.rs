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
#[macro_export]
macro_rules! all_matches {
    ($collection:expr, $pattern:pat) => {{ $collection.into_iter().all(|e| matches!(e, $pattern)) }};
    ($collection:ident, $pattern:pat) => {{ $collection.into_iter().all(|e| matches!(e, $pattern)) }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_all_matches_vec() {
        let vec = vec![1, 2, 3, 4, 5];
        assert!(all_matches!(vec, 1..=5));
    }

    #[test]
    fn test_all_matches_slice() {
        let slice = &[1, 2, 3, 4, 5];
        assert!(all_matches!(slice, 1..=5));
    }
}
