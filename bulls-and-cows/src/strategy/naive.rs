use super::common;
use super::Strategy;

#[derive(Clone)]
pub struct NaiveStrategy {
	all_values: Vec<String>,
	candidates: Vec<String>,
	last_guess: String,
}

impl NaiveStrategy {
	pub fn new(n: i32) -> Self {
		let mut all_values = common::gen_values(n);
		all_values.sort();
		NaiveStrategy {
			all_values,
			candidates: Vec::new(),
			last_guess: String::new(),
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

	fn respond_to_guess(&mut self, bulls: i32, cows: i32) {
		self.candidates.retain(|x| {
			let bc = common::calc_bc(&self.last_guess, x);
			bc.0 == bulls && bc.1 == cows
		});
	}

	fn clone_strategy(&self) -> Box<dyn Strategy> {
		Box::new(self.clone())
	}
}
