#[macro_export]
macro_rules! own {
    ($lit: literal) => {
	    $lit.to_string()
    };
}

#[test]
fn test() {
	let str = own!("hello");
	assert_eq!(str, "hello".to_string());
	
	let str = own!("hello world");
	assert_eq!(str, "hello world".to_string());
}