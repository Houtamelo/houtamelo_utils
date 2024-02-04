mod fn_once;
mod fn_mut;
mod fn_;

pub use fn_once::{FnOnceArgConverter, FnOnceArgRefConverter};
pub use fn_mut::{FnMutArgConverter, FnMutArgRefConverter};
pub use fn_::{FnArgConverter, FnArgRefConverter};

mod phantom_constraints {
	use super::*;
	
	fn _constrain() {
		_once(_closure.tupled_once());
		_once(_closure.tupled_mut());
		_once(_closure.tupled());
		
		_once_ref(_closure_ref.tupled_once_ref());
		_once_ref(_closure_ref.tupled_mut_ref());
		_once_ref(_closure_ref.tupled_ref());
		
		_mut(_closure.tupled_mut());
		_mut(_closure.tupled());

		_mut_ref(_closure_ref.tupled_mut_ref());
		_mut_ref(_closure_ref.tupled_ref());
		
		_fn(_closure.tupled());
		_fn_ref(_closure_ref.tupled_ref());
	}

	fn _closure(_: Vec<i32>, _: Vec<i64>) {}
	fn _closure_ref(_: &Vec<i32>, _: &Vec<i64>) {}

	fn _once(_: impl FnOnce((Vec<i32>, Vec<i64>))) {}
	fn _once_ref(_: impl FnOnce(&(Vec<i32>, Vec<i64>))) {}

	fn _mut(_: impl FnMut((Vec<i32>, Vec<i64>))) {}
	fn _mut_ref(_: impl FnMut(&(Vec<i32>, Vec<i64>))) {}

	fn _fn(_: impl Fn((Vec<i32>, Vec<i64>))) {}
	fn _fn_ref(_: impl Fn(&(Vec<i32>, Vec<i64>))) {}
}