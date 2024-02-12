#[macro_export]
macro_rules! own_str {
    ($lit: literal) => {
	    $lit.to_string()
    };
}

#[test]
fn test() {
	let str = own_str!("hello");
	assert_eq!(str, "hello".to_string());
	
	let str = own_str!("hello world");
	assert_eq!(str, "hello world".to_string());
}