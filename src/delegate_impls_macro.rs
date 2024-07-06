#[macro_export]
macro_rules! delegate_impls {
    (
	    $enum_vis: vis enum $enum_ident: ident {
			$($var_ident: ident($var_field: ty)),* 
			$(,)?
		}
	    
	    $trait_vis: vis trait $trait_ident: ident {
		    $(
		        fn $fn_ident: ident $(<$($gen: ident),+>)? (
			        self: $self_ty: ty,
			        $($args: ident: $arg_tys: ty),*
		        ) -> $fn_ret_ty: ty;
		    )*
	    }
    ) => {
	    delegate_impls!(1
	        ENUM: $enum_ident
	        TRAIT: $trait_ident
	        VARIANTS: { $($var_ident($var_field)),* }
	        FUNCTIONS: 
			    $(
			        fn $fn_ident $(<$($gen),+>)? (
				        self: $self_ty,
				        $($args: $arg_tys),*
			        )-> $fn_ret_ty;
			    )*
	    );
    };
	(1
		ENUM: $enum_ident: ident
		TRAIT: $trait_ident: ident
		VARIANTS: $var_idents: tt
		FUNCTIONS: 
			$(
				fn $fn_ident: ident $(<$($gen: ident),+>)? (
					self: $self_ty: ty,
					$($args: ident: $arg_tys: ty),*
				) -> $fn_ret_ty: ty;
			)*
	) => {
		delegate_impls!(2
			ENUM: $enum_ident
	        TRAIT: $trait_ident
			$(
				VARIANTS: [ $var_idents | { $($args),* } ]
				fn $fn_ident $(<$($gen),+>)? (
					self: $self_ty,
					$($args: $arg_tys),*
				) -> $fn_ret_ty;
			)*
		);
	};
	(2 
		ENUM: $enum_ident: ident
		TRAIT: $trait_ident: ident
		$(
			VARIANTS: [ { $($var_ident: ident($var_field: ty)),* } | $args_call: tt]
			fn $fn_ident: ident $(<$($gen: ident),+>)? (
				self: $self_ty: ty,
				$($args: ident: $arg_tys: ty),*
			) -> $fn_ret_ty: ty;
		)*
	) => {
		delegate_impls!(3
			ENUM: $enum_ident
	        TRAIT: $trait_ident
			$(
				VARIANTS: [ $($var_ident($var_field) | $args_call),* ]
				fn $fn_ident $(<$($gen),+>)? (
					self: $self_ty,
					$($args: $arg_tys),*
				) -> $fn_ret_ty;
			)*
		);
	};
	(3 
		ENUM: $enum_ident: ident
		TRAIT: $trait_ident: ident
		$(
			VARIANTS: [ $($var_ident: ident($var_field: ty) | { $($args_call: ident),* }),* ]
			fn $fn_ident: ident $(<$($gen: ident),+>)? (
				self: $self_ty: ty,
				$($args: ident: $arg_tys: ty),*
			) -> $fn_ret_ty: ty;
		)*
	) => {
		impl $trait_ident for $enum_ident {
			$(
				fn $fn_ident $(<$($gen),+>)? (
					self: $self_ty,
					$($args: $arg_tys),*
				) -> $fn_ret_ty {
			        match self {
			            $(
			                $enum_ident::$var_ident(variant) => {
			                    variant.$fn_ident($($args_call),*)
			                }
			            ),*
			        } 
				}
			)*
		}
	};
}
