use crate::game_utils;

use super::game_utils::{GameParams, Number};
use super::Strategy;

#[derive(Clone)]
pub struct NaiveStrategy {
	all_values: Vec<Number>,
	candidates: Vec<Number>,
	last_guess: Number,
	last_guess_str: String,
	game: GameParams,
}

impl NaiveStrategy {
	pub fn new(game: GameParams) -> Self {
		let mut all_values: Vec<_> = game_utils::get_numbers_iter(&game).collect();
		all_values.sort();
		NaiveStrategy {
			all_values,
			candidates: Vec::new(),
			last_guess: Number::default(),
			last_guess_str: String::new(),
			game,
		}
	}
}

impl Strategy for NaiveStrategy {
	fn init(&mut self) {
		self.candidates = self.all_values.clone();
	}

	fn make_guess(&mut self) -> Option<&Number> {
		self.candidates.first()
	}

	fn respond_to_guess(&mut self, bulls: u8, cows: u8) {
		self.candidates.retain(|x| {
			let bc = self.game.calc_bc(&self.last_guess, &x);
			bc.0 == bulls && bc.1 == cows
		});
	}

	fn clone_strategy(&self) -> Box<dyn Strategy> {
		Box::new(self.clone())
	}
}
