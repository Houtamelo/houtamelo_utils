#[macro_export]
macro_rules! own {
    ($lit: literal) => {
	    $lit.to_string()
    };
}

#[macro_export]
macro_rules! own_vec {
    [$($lit: literal),+] => {
	    vec![$($lit.to_string()),+]
    };
}

#[test]
fn test() {
	let str = own!("hello");
	assert_eq!(str, "hello".to_string());

	let str = own!("hello world");
	assert_eq!(str, "hello world".to_string());
	
	let vec = own_vec!["hello", "world"];
	assert_eq!(vec, vec!["hello".to_string(), "world".to_string()]);
	
	let vec = own_vec!["hello", "world", "how", "are", "you"];
	assert_eq!(vec, vec!["hello".to_string(), "world".to_string(), "how".to_string(), "are".to_string(), "you".to_string()]);
}


