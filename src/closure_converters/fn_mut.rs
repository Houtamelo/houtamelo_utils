#![allow(non_snake_case)]
#![allow(refining_impl_trait)]

pub trait FnMutArgConverter<TConvertTo, TOutput> {
	fn tupled_mut(self) -> impl FnMut(TConvertTo) -> TOutput;
}

pub trait FnMutArgRefConverter<TConvertTo, TOutput> {
	fn tupled_mut_ref(self) -> impl FnMut(TConvertTo) -> TOutput;
}

macro_rules! impl_fn_mut_converter {
	($($args:ident),*) => {
		impl<$($args),*, TOutput, Fn> FnMutArgConverter<($($args),*), TOutput> for Fn 
			where Fn: FnMut($($args),*) -> TOutput {
			fn tupled_mut(mut self) -> impl FnMut(($($args),*)) -> TOutput {
				move |($($args),*)| { self($($args),*) }
			}
		}
		
		impl<$($args),*, TOutput, Fn> FnMutArgRefConverter<&($($args),*), TOutput> for Fn 
			where Fn: FnMut($(&$args),*) -> TOutput {
			fn tupled_mut_ref(mut self) -> impl FnMut(&($($args),*)) -> TOutput {
				move |($($args),*)| { self($($args),*) }
			}
		}
	};
}

impl_fn_mut_converter!(A, B);
impl_fn_mut_converter!(A, B, C);
impl_fn_mut_converter!(A, B, C, D);
impl_fn_mut_converter!(A, B, C, D, E);
impl_fn_mut_converter!(A, B, C, D, E, F);
impl_fn_mut_converter!(A, B, C, D, E, F, G);
impl_fn_mut_converter!(A, B, C, D, E, F, G, H);
impl_fn_mut_converter!(A, B, C, D, E, F, G, H, I);
impl_fn_mut_converter!(A, B, C, D, E, F, G, H, I, J);
impl_fn_mut_converter!(A, B, C, D, E, F, G, H, I, J, K);
