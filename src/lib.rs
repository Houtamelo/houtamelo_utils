#![feature(structural_match)]

mod iter_generator;
mod any_matches_macro;
mod bounded_ints;
mod clamp01;
mod bounded_floats;
mod indexed_set;
mod indexed_hash_map;
mod dynamic_array;

pub mod prelude {
	pub use super::iter_generator::*;
	pub use super::any_matches;
	pub use super::clamp01::*;
	pub use super::bounded_ints::*;
	pub use super::bounded_ints::bound_i8::*;
	pub use super::bounded_ints::bound_i16::*;
	pub use super::bounded_ints::bound_u8::*;
	pub use super::bounded_ints::bound_u16::*;
	pub use super::bound_f32_impl;
	pub use super::bound_f64_impl;
	pub use super::indexed_set::*;
	pub use super::indexed_hash_map::*;
	pub use super::dynamic_array::*;
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