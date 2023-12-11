#![feature(structural_match)]

mod collection_trait;
mod any_matches_macro;
mod bounded_ints;
mod clamp01;
mod bounded_floats;

pub mod prelude {
	pub use super::collection_trait::*;
	pub use super::any_matches;
	pub use super::clamp01::*;
	pub use super::bounded_ints::*;
	pub use super::bound_f32_impl;
	pub use super::bound_f64_impl;
}