/// # Example
///
/// ---
/// use houtamelo_utils::variant_table;
///
/// variant_table! {
///     { Variants: [derive(Debug)] } // [Optional] Attributes applied on all variants
///     #[derive(Debug, Clone, Copy)] // [Optional] Attributes applied only on the main enum
///     pub enum Setting {
///         #[derive(Default)] // [Optional] Attributes applied on individual variant 
///         WindowMaximized(bool),
///         WindowSize(f32),
///         SkillOverlayMode(i32),
///         Language(i64),
///         MaxFps(i32),
///         DialogueTextSpeed(i32),
///         Vsync(bool),
///         MainVolume(i32),
///         MusicVolume(i32),
///         SfxVolume(i32),
///         VoiceVolume(i32),
///     }
/// }
/// ---
/// 
/// # Restrictions
/// - Only tuple variants with a single field are supported
/// - Main enum must have uppercase letters
#[macro_export]
macro_rules! variant_table {
	(
		$( 
	        { Variants: $( [$all_vars_meta: meta] ),* }
	    )?
		$( #[$meta: meta] )*
	    $enum_vis: vis enum $enum_ident: ident {
		    $(
		        $( #[$var_meta: meta] )*
		        $var_ident: ident ($var_ty: ty) 
		    ),* 
		    $(,)?
	    }
    ) => {
		$crate::variant_table! {
			@ENTRY
	    
		    { $( #[$meta] )* }
			{ $($( #[$all_vars_meta] ),*)? }
		    $enum_vis enum $enum_ident {
			    $(
			        $( #[$var_meta] )*
			        $var_ident ($var_ty) 
			    ),*
		    }
		}
    };
	(@ENTRY
	    $enum_metas: tt
	    $all_vars_metas: tt
	    $enum_vis:vis enum $enum_ident: ident {
		    $(
		        $( #[$var_meta: meta] )*
		        $var_ident: ident ($var_ty: ty) 
		    ),* 
		    $(,)?
	    }
    ) => {
		
		paste::paste! {
			
			#[allow(non_snake_case)]
			#[allow(non_camel_case_types)]
			#[allow(unused)]
			#[allow(clippy::needless_lifetimes)]
			#[allow(private_interfaces)]
			#[allow(private_bounds)]
			$enum_vis mod [<$enum_ident:snake>] {
			    use super::*;
			    
			    $crate::variant_table! {
				    @ENUM_TABLE
				    
				    $enum_metas
				    
				    enum $enum_ident {
					    $(
					        $( #[$var_meta] )*
					        $var_ident ($var_ty) 
					    ),*
				    }
			    }
			    
			    $($crate::variant_table! { 
				    @VARIANTS
				    $all_vars_metas
				    { $( #[$var_meta] )* }
				    $enum_ident
				    $var_ident: $var_ty
			    })*
		    }
		}
	};
	(@ENUM_TABLE
		{ $( #[$meta: meta] )* }
	    enum $enum_ident: ident {
		    $(
		        $( #[$var_meta: meta] )*
		        $var_ident: ident ($var_ty: ty) 
		    ),* 
		    $(,)?
	    }
	) => {
		$( #[$meta] )*
	    pub enum $enum_ident {
		    $( $var_ident ($var_ty) ),*
	    }
		
		paste::paste! {
			pub enum [<$enum_ident Ref>]<'a> {
			    $( $var_ident (&'a $var_ty) ),*
		    }
			
			pub enum [<$enum_ident RefMut>]<'a> {
			    $( $var_ident (&'a mut $var_ty) ),*
		    }
			
		    pub struct [<$enum_ident Table>] {
			    $( pub [<$var_ident:snake>]: $var_ident ),*
		    }
		    
		    pub trait [<GetIn $enum_ident Table>] {
				type Ret;
				
			    fn get_in_table(table: &[<$enum_ident Table>]) -> &Self::Ret;
			    fn get_in_table_mut(table: &mut [<$enum_ident Table>]) -> &mut Self::Ret;
		    }
		    
		    impl [<$enum_ident Table>] {
				pub fn get<T: [<GetIn $enum_ident Table>]>(&self) -> &T::Ret {
				    T::get_in_table(self)
			    }
			    
			    pub fn get_mut<T: [<GetIn $enum_ident Table>]>(&mut self) -> &mut T::Ret {
				    T::get_in_table_mut(self)
			    }
	
				pub fn iter<'a>(&'a self) -> impl Iterator<Item = [<$enum_ident Ref>]<'a> > + 'a {
					std::iter::from_coroutine(
						#[coroutine] || {
							$(
								yield [<$enum_ident Ref>]::$var_ident(&self.[<$var_ident:snake>].0);
								//
							)*
						}
					)
				}
				
				pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = [<$enum_ident RefMut>]<'a> > + 'a {
					std::iter::from_coroutine(
						#[coroutine] || {
							$(
								yield [<$enum_ident RefMut>]::$var_ident(&mut self.[<$var_ident:snake>].0);
							)*
						}
					)
				}
			}
		}
	};
	(@VARIANTS
		{ $( #[$all_vars_meta: meta] )* }
		{ $( #[$var_meta: meta] )* }
		$enum_ident: ident
		$var_ident: ident: $var_ty: ty
	) => {
		$( #[$all_vars_meta] )*
		$( #[$var_meta] )*
        pub struct $var_ident( pub $var_ty );
    
		paste::paste!{
			impl [<GetIn $enum_ident Table>] for $var_ident {
				type Ret = $var_ty ;
				
	            fn get_in_table(table: & [<$enum_ident Table>]) -> &Self::Ret {
	                &table.[<$var_ident:snake>].0
	            }
		        
	            fn get_in_table_mut(table: &mut [<$enum_ident Table>]) -> &mut Self::Ret {
	                &mut table.[<$var_ident:snake>].0
	            }
	        }
		}
		
		impl From<$var_ident> for $enum_ident {
			fn from(value: $var_ident) -> Self {
				Self::$var_ident(value.0)
			}
		}
		
		impl TryFrom<$enum_ident> for $var_ident {
			type Error = $enum_ident;
		
			fn try_from(value: $enum_ident) -> Result<Self, Self::Error> {
				if let $enum_ident::$var_ident(v) = value {
					Ok($var_ident(v))
				} else {
					Err(value)
				}
			}
		}
	};
}


#[cfg(test)]
mod tests {
	variant_table! {
	    { Variants: [derive(Debug)] }
	    #[derive(Debug, Clone, Copy)]
	    pub enum Setting {
	        #[derive(Default)]
	        WindowMaximized(bool),
	        WindowSize(f32),
	        SkillOverlayMode(i32),
	        Language(i64),
	        MaxFps(i32),
	        DialogueTextSpeed(i32),
	        Vsync(bool),
	        MainVolume(i32),
	        MusicVolume(i32),
	        SfxVolume(i32),
	        VoiceVolume(i32),
	    }
	}
}