use crate::game_utils;

use super::game_utils::{GameParams, Number};
use super::Strategy;

#[derive(Clone)]
pub struct NaiveStrategy {
	all_values: Vec<Number>,
	candidates: Vec<Number>,
	game: GameParams,
}

impl NaiveStrategy {
	pub fn new(game: GameParams) -> Self {
		let mut all_values: Vec<_> = game_utils::get_numbers_iter(&game).collect();
		all_values.sort();
		NaiveStrategy {
			all_values,
			candidates: Vec::new(),
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
		if let Some(last) = self.candidates.first() {
			let last = last.clone();
			self.candidates.retain(|x| {
				let bc = self.game.calc_bc(&last, &x);
				bc.0 == bulls && bc.1 == cows
			});
		}
	}

	fn clone_strategy(&self) -> Box<dyn Strategy> {
		Box::new(self.clone())
	}
}
