use super::Strategy;
use crate::common;

#[derive(Clone)]
pub struct MinAvg {
	all_values: Vec<String>,
	candidates: Vec<String>,
	is_first: bool,
	last_guess: String,
	n: i32,
}

impl MinAvg {
	pub fn new(n: i32) -> Self {
		let all_values = common::gen_values(n);
		Self {
			all_values,
			candidates: Vec::new(),
			is_first: false,
			last_guess: String::new(),
			n: n + 1,
		}
	}

	fn evaluate_attempt(&self, attempt: &str) -> u64 {
		let mut v = [0; 25];
		for ans in self.candidates.iter() {
			let bc = common::calc_bc(attempt, ans);
			v[(bc.0 * self.n + bc.1) as usize] += 1;
		}
		v.iter()
			.filter_map(|x| if *x != 0 { Some(x * x) } else { None })
			.sum()
	}
}

impl Strategy for MinAvg {
	fn init(&mut self) {
		self.candidates = self.all_values.clone();
		self.is_first = true;
	}

	fn make_guess(&mut self) -> Option<&str> {
		if self.is_first {
			self.is_first = false;
			self.last_guess = self.candidates[0].clone();
		} else {
			if self.candidates.len() == 0 {
				return None;
			} else {
				let mut min_value = u64::max_value();
				let mut hs = std::collections::HashSet::new();
				for attempt in self.candidates.iter() {
					let value = self.evaluate_attempt(attempt);
					if min_value > value {
						min_value = value;
						self.last_guess = attempt.clone();
					}
					hs.insert(attempt);
				}
				for attempt in self.all_values.iter() {
					if hs.contains(attempt) {
						continue;
					}
					let value = self.evaluate_attempt(attempt);
					if min_value > value {
						min_value = value;
						self.last_guess = attempt.clone();
					}
				}
			}
		}
		Some(&self.last_guess)
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
