#![feature(structural_match)]
#![feature(hash_extract_if)]
#![feature(const_type_name)]
#![feature(vec_push_within_capacity)]
#![feature(impl_trait_in_assoc_type)]
#![feature(coroutines)]
#![feature(iter_from_coroutine)]
#![feature(macro_metavar_expr)]
#![allow(non_camel_case_types)]
#![allow(clippy::doc_lazy_continuation)]

mod iter_generator;
mod remove_many;
mod take_first;
mod any_matches_macro;
mod clamp01;
mod bounded_floats;
mod indexed_set;
mod indexed_hash_map;
mod dynamic_array;
mod no_matches_macro;
mod touch;
mod none;
mod closure_converters;
mod str_macro;
mod count_or_more;
mod fn_name;
mod all_matches;
mod hash_set_extract_keys;
mod delegate_impls_macro;

#[cfg(feature = "rand")]
mod weighted_rand;

#[cfg(feature = "rand")]
mod clone_random;

#[cfg(feature = "rand")]
mod take_random;

#[cfg(feature = "rand")]
mod get_random;

pub mod prelude {
	pub use std::{
		any::{type_name, type_name_of_val},
		collections::{HashMap, HashSet, VecDeque},
		hash::Hash,
		marker::PhantomData,
		mem,
		ops::{Deref, DerefMut},
		rc::{Rc, Weak},
	};

	pub use anyhow::{anyhow, bail, Result};
	pub use pluck::pluck;

	pub use super::{
		all_matches,
		any_matches,
		bound_f32_impl,
		bound_f64_impl,
		clamp01::*,
		closure_converters::*,
		count_or_more::*,
		dynamic_array::*,
		fn_name::*,
		hash_set_extract_keys::*,
		indexed_hash_map::*,
		indexed_set::*,
		iter_generator::*,
		no_matches,
		none::None,
		own,
		own_vec,
		remove_many::*,
		take_first::*,
		touch::*,
	};
	
	pub use crate::delegate_impls;
	
	#[cfg(feature = "rand")]
	pub use super::clone_random::*;
	#[cfg(feature = "rand")]
	pub use super::get_random::*;
	#[cfg(feature = "rand")]
	pub use super::take_random::*;
	#[cfg(feature = "rand")]
	pub use super::weighted_rand::*;
}