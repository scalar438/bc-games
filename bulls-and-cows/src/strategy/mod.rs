use std::mem;

use super::common;

pub trait Strategy {
	// Init the strategy. After this call the object is ready to start a game
	fn init(&mut self);

	fn make_guess(&mut self) -> &str;

	fn answer_to_guess(&mut self, bulls: i32, cows: i32);

	fn _clone_dyn(&self) -> Box<dyn Strategy>;
}

#[derive(Clone)]
struct NaiveStrategy {
	all_values: Vec<String>,
	candidates: Vec<String>,
	last_guess: String,
}

impl NaiveStrategy {
	fn new(n: i32) -> Self {
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

#[derive(Clone)]
struct EnthropyStrategy {
	all_values: Vec<String>,
	candidates: Vec<String>,
	is_first: bool,
	last_guess: String,
	n: i32,
}

impl EnthropyStrategy {
	fn new(n: i32) -> Self {
		let all_values = common::gen_values(n);
		Self {
			all_values,
			candidates: Vec::new(),
			is_first: false,
			last_guess: String::new(),
			n: n + 1,
		}
	}

	fn calc_info(&self, attempt: &str) -> f64 {
		let mut v = [0; 25];
		for ans in self.candidates.iter() {
			let bc = common::calc_bc(attempt, ans);
			v[(bc.0 * self.n + bc.1) as usize] += 1;
		}
		let l = self.candidates.len() as f64;
		v.iter()
			.filter_map(|x| {
				if *x != 0 {
					let p = (*x as f64) / l;
					Some(p * -f64::ln(p))
				} else {
					None
				}
			})
			.sum()
	}
}

impl Strategy for EnthropyStrategy {
	fn init(&mut self) {
		self.candidates = self.all_values.clone();
		self.is_first = true;
	}

	fn make_guess(&mut self) -> &str {
		if self.is_first {
			self.is_first = false;
			self.last_guess = self.candidates[0].clone();
		} else {
			if self.candidates.len() == 1 {
				self.last_guess = self.candidates[0].clone();
				return &self.last_guess;
			}
			let mut avg_info = 0.0;
			let mut hs = std::collections::HashSet::new();
			for attempt in self.candidates.iter() {
				let ent = self.calc_info(attempt);
				if ent > avg_info {
					avg_info = ent;
					self.last_guess = attempt.clone();
				}
				hs.insert(attempt);
			}
			for attempt in self.all_values.iter() {
				if hs.contains(attempt) {
					continue;
				}
				let ent = self.calc_info(attempt);
				if ent > avg_info {
					avg_info = ent;
					self.last_guess = attempt.clone();
				}
			}
		}
		&self.last_guess
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
		todo!()
	}
}

struct MinMaxStrategy {
	all_values: Vec<String>,
	candidates: Vec<String>,
	is_first: bool,
	last_guess: String,
	n: i32,
}

impl MinMaxStrategy {
	fn new(n: i32) -> Self {
		let all_values = common::gen_values(n);
		Self {
			all_values,
			candidates: Vec::new(),
			is_first: false,
			last_guess: String::new(),
			n: n + 1,
		}
	}

	fn evaluate_attempt(&self, attempt: &str) -> i32 {
		let mut v = [0; 25];
		for ans in self.candidates.iter() {
			let bc = common::calc_bc(attempt, ans);
			v[(bc.0 * self.n + bc.1) as usize] += 1;
		}
		*v.iter().max().unwrap()
	}
}

impl Strategy for MinMaxStrategy {
	fn init(&mut self) {
		self.candidates = self.all_values.clone();
		self.is_first = true;
	}

	fn make_guess(&mut self) -> &str {
		if self.is_first {
			self.is_first = false;
			self.last_guess = self.candidates[0].clone();
		} else {
			if self.candidates.len() == 1 {
				self.last_guess = self.candidates[0].clone();
				return &self.last_guess;
			}
			let mut min_value = self.candidates.len() as i32;
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
		&self.last_guess
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
		todo!()
	}
}

#[derive(Debug, Clone, Copy)]
pub enum StrategyType {
	Naive,
	AmountInformation,
	MinMax,
}

pub fn create_strategy(t: StrategyType, n: i32) -> Box<dyn Strategy> {
	match t {
		StrategyType::Naive => Box::new(NaiveStrategy::new(n)),
		StrategyType::AmountInformation => Box::new(EnthropyStrategy::new(n)),
		StrategyType::MinMax => Box::new(MinMaxStrategy::new(n)),
	}
}
