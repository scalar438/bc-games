use crate::game_utils::{self, RefIter};

use super::game_utils::{GameParams, Number};
use super::Strategy;

#[derive(Clone)]
pub struct NaiveStrategy {
	cur_number_iter: Box<dyn RefIter<Item = Number>>,
	responses: Vec<(Number, u8, u8)>,
	game: GameParams,
	last_response: Number,
}

impl NaiveStrategy {
	pub fn new(game: GameParams) -> Self {
		NaiveStrategy {
			cur_number_iter: game_utils::get_numbers_iter_ref(&game),
			responses: Vec::new(),
			game,
			last_response: Number::empty(),
		}
	}
}

impl Strategy for NaiveStrategy {
	fn init(&mut self) {
		self.cur_number_iter = game_utils::get_numbers_iter_ref(&self.game);
		self.responses.clear();
	}

	fn make_guess(&mut self) -> Option<&Number> {
		while let Some(new_num) = self.cur_number_iter.next() {
			if self
				.responses
				.iter()
				.all(|(n, b, c)| self.game.calc_bc(n, new_num) == (*b, *c))
			{
				self.last_response = new_num.clone();
				return Some(&self.last_response);
			}
		}
		None
	}

	fn respond_to_guess(&mut self, bulls: u8, cows: u8) {
		self.responses.push((
			std::mem::replace(&mut self.last_response, Number::empty()),
			bulls,
			cows,
		));
	}

	fn clone_strategy(&self) -> Box<dyn Strategy> {
		Box::new(self.clone())
	}
}
