#![allow(non_snake_case)]
#![allow(refining_impl_trait)]

pub trait FnOnceArgConverter<TConvertTo, TOutput> {
	fn tupled_once(self) -> impl FnOnce(TConvertTo) -> TOutput;
}

pub trait FnOnceArgRefConverter<TConvertTo, TOutput> {
	fn tupled_once_ref(self) -> impl FnOnce(TConvertTo) -> TOutput;
}

macro_rules! impl_fn_once_converter {
	($($args:ident),*) => {
		impl<$($args),*, TOutput, Fn> FnOnceArgConverter<($($args),*), TOutput> for Fn 
			where Fn: FnOnce($($args),*) -> TOutput {
			fn tupled_once(self) -> impl FnOnce(($($args),*)) -> TOutput {
				move |($($args),*)| { self($($args),*) }
			}
		}
		
		impl<$($args),*, TOutput, Fn> FnOnceArgRefConverter<&($($args),*), TOutput> for Fn 
			where Fn: FnOnce($(&$args),*) -> TOutput {
			fn tupled_once_ref(self) -> impl FnOnce(&($($args),*)) -> TOutput {
				move |($($args),*)| { self($($args),*) }
			}
		}
	};
}

impl_fn_once_converter!(A, B);
impl_fn_once_converter!(A, B, C);
impl_fn_once_converter!(A, B, C, D);
impl_fn_once_converter!(A, B, C, D, E);
impl_fn_once_converter!(A, B, C, D, E, F);
impl_fn_once_converter!(A, B, C, D, E, F, G);
impl_fn_once_converter!(A, B, C, D, E, F, G, H);
impl_fn_once_converter!(A, B, C, D, E, F, G, H, I);
impl_fn_once_converter!(A, B, C, D, E, F, G, H, I, J);
impl_fn_once_converter!(A, B, C, D, E, F, G, H, I, J, K);


