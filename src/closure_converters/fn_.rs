#![allow(non_snake_case)]
#![allow(refining_impl_trait)]

pub trait FnArgConverter<TConvertTo, TOutput> {
	fn tupled(self) -> impl Fn(TConvertTo) -> TOutput;
}

pub trait FnArgRefConverter<TConvertTo, TOutput> {
	fn tupled_ref(self) -> impl Fn(TConvertTo) -> TOutput;
}

macro_rules! impl_fn_converter {
	($($args:ident),*) => {
		impl<$($args),*, TOutput, GenFn> FnArgConverter<($($args),*), TOutput> for GenFn 
			where GenFn: Fn($($args),*) -> TOutput {
			fn tupled(self) -> impl Fn(($($args),*)) -> TOutput {
				move |($($args),*)| { self($($args),*) }
			}
		}
		
		impl<$($args),*, TOutput, GenFn> FnArgRefConverter<&($($args),*), TOutput> for GenFn 
			where GenFn: Fn($(&$args),*) -> TOutput {
			fn tupled_ref(self) -> impl Fn(&($($args),*)) -> TOutput {
				move |($($args),*)| { self($($args),*) }
			}
		}
	};
}

impl_fn_converter!(A, B);
impl_fn_converter!(A, B, C);
impl_fn_converter!(A, B, C, D);
impl_fn_converter!(A, B, C, D, E);
impl_fn_converter!(A, B, C, D, E, F);
impl_fn_converter!(A, B, C, D, E, F, G);
impl_fn_converter!(A, B, C, D, E, F, G, H);
impl_fn_converter!(A, B, C, D, E, F, G, H, I);
impl_fn_converter!(A, B, C, D, E, F, G, H, I, J);
impl_fn_converter!(A, B, C, D, E, F, G, H, I, J, K);
