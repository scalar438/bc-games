use super::common;
use super::Strategy;
use core::mem;

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

	fn make_guess(&mut self) -> &str {
		self.last_guess = self.candidates[0].clone();
		self.last_guess.as_str()
	}

	fn answer_to_guess(&mut self, bulls: i32, cows: i32) {
		let q = mem::replace(&mut self.candidates, Vec::new());
		self.candidates = q
			.into_iter()
			.filter(|x| {
				let bc = common::calc_bc(&self.last_guess, x);
				bc.0 == bulls && bc.1 == cows
			})
			.collect();
	}

	fn _clone_dyn(&self) -> Box<dyn Strategy> {
		Box::new(self.clone())
	}
}
