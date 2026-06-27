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
#[macro_export]
macro_rules! no_matches {
    ($collection:expr, $pattern:pat) => {{ $collection.into_iter().all(|e| false == matches!(e, $pattern)) }};
    ($collection:ident, $pattern:pat) => {{ $collection.into_iter().all(|e| false == matches!(e, $pattern)) }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_no_matches_vec() {
        let vec = vec![1, 2, 3, 4, 5];
        assert!(no_matches!(vec, 6..=10));
    }

    #[test]
    fn test_no_matches_slice() {
        let slice = &[1, 2, 3, 4, 5];
        assert!(no_matches!(slice, 6..=10));
    }
}
