#![feature(structural_match)]
#![feature(hash_extract_if)]
#![feature(const_type_name)]
#![feature(iter_from_coroutine)]
#![feature(coroutines)]
#![feature(vec_push_within_capacity)]
#![allow(non_camel_case_types)]

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
mod none_or;
mod none;
mod closure_converters;
mod weighted_rand;
mod str_macro;
mod count_or_more;
mod fn_name;
mod all_matches;
mod hash_set_extract_keys;
mod clone_random;
mod take_random;
mod get_random;

pub mod prelude {
	pub use std::any::{type_name, type_name_of_val};
	pub use std::collections::{HashMap, HashSet, VecDeque};
	pub use std::hash::Hash;
	pub use std::marker::PhantomData;
	pub use std::mem;
	pub use std::ops::{Deref, DerefMut};
	pub use std::rc::{Rc, Weak};

	pub use anyhow::{anyhow, bail, Result};
	pub use inline_format::format as iformat;
	pub use inline_format::format_args as iformat_args;
	pub use inline_format::write as iwrite;
	pub use pluck::pluck;

	pub use super::all_matches;
	pub use super::any_matches;
	pub use super::bound_f32_impl;
	pub use super::bound_f64_impl;
	pub use super::clamp01::*;
	pub use super::clone_random::*;
	pub use super::closure_converters::*;
	pub use super::count_or_more::*;
	pub use super::dynamic_array::*;
	pub use super::fn_name::*;
	pub use super::get_random::*;
	pub use super::hash_set_extract_keys::*;
	pub use super::indexed_hash_map::*;
	pub use super::indexed_set::*;
	pub use super::iter_generator::*;
	pub use super::no_matches;
	pub use super::none::None;
	#[allow(unused)]
	pub use super::none_or::*;
	pub use super::own;
	pub use super::own_vec;
	pub use super::remove_many::*;
	pub use super::take_first::*;
	pub use super::take_random::*;
	pub use super::touch::*;
	pub use super::weighted_rand::*;
}

#[cfg(test)]
mod tests {
	use crate::prelude::Clamp01;

	#[test]
	fn test() {
		let mut chances : Vec<f64> = Vec::new();
		
		let offset = 0.0;
		let mut turn: isize = 0;
		loop {
			let chance = calc_chance(turn) - offset;
			turn += 1;
			
			if chance < 0.0 {
				break;
			} else {
				chances.push(chance.clamp01());
			}
		}
		
		let mut current_chance = 1.0;
		let mut medium_chances = Vec::new();
		
		for chance in chances.iter() {
			current_chance *= chance;
			medium_chances.push(current_chance);
		}

		let mut to_sub = 0.0;
		let mut final_chances = medium_chances.clone();
		let mut index = final_chances.len() - 1;
		while index > 0 {
			to_sub += final_chances[index];
			final_chances[index - 1] -= to_sub;
			index -= 1;
		}
		
		let average_turns = final_chances.iter().enumerate()
										 .fold(0.0, |sum, (index, chance)| {
					sum + chance * (index as f64 + 1.0)
				});
		
		let chance_sum = final_chances.iter().fold(0.0, |sum, chance| sum + chance);
		
		println!("Average turns: {}", average_turns);
		println!("Chance sum: {chance_sum:.4}");
		println!("Base Chances: {chances:.4?}");
		println!("Medium chances: {medium_chances:.4?}");
		println!("Final chances: {final_chances:.4?}");
	}
	
	fn calc_chance(input: isize) -> f64 {
		return 1.0 - match input {
			0 => 0.0,
			1 => 0.005,
			2 => 0.02,
			3 => 0.04,
			4 => 0.08,
			5 => 0.12,
			6 => 0.16,
			7 => 0.20,
			8 => 0.24,
			9 => 0.28,
			10 => 0.32,
			11 => 0.40,
			12 => 0.48,
			13 => 0.56,
			_ => input as f64,
		};
	}
}