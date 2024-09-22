use crate::game_utils;

use super::game_utils::{GameParams, Number};
use super::Strategy;

#[derive(Clone)]
pub struct NaiveStrategy {
	all_values: Vec<String>,
	candidates: Vec<String>,
	last_guess: String,
	game: GameParams,
}

impl NaiveStrategy {
	pub fn new(game: GameParams) -> Self {
		let mut all_values: Vec<_> = game_utils::get_numbers_iter(&game)
			.map(|x| x.to_string())
			.collect();
		all_values.sort();
		NaiveStrategy {
			all_values,
			candidates: Vec::new(),
			last_guess: String::new(),
			game,
		}
	}
}

impl Strategy for NaiveStrategy {
	fn init(&mut self) {
		self.candidates = self.all_values.clone();
	}

	fn make_guess(&mut self) -> Option<&str> {
		if let Some(guess) = self.candidates.first() {
			self.last_guess = guess.clone();
			Some(self.last_guess.as_str())
		} else {
			None
		}
	}

	fn respond_to_guess(&mut self, bulls: u8, cows: u8) {
		let last_guess = Number::from(&self.last_guess);
		self.candidates.retain(|x| {
			let bc = self.game.calc_bc(&last_guess, &Number::from(x));
			bc.0 == bulls && bc.1 == cows
		});
	}

	fn clone_strategy(&self) -> Box<dyn Strategy> {
		Box::new(self.clone())
	}
}
