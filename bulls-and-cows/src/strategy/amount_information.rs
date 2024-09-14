use super::Strategy;
use crate::common;
use core::mem;

#[derive(Clone)]
pub struct AmountInfStrategy {
	all_values: Vec<String>,
	candidates: Vec<String>,
	is_first: bool,
	last_guess: String,
	n: i32,
}

impl AmountInfStrategy {
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

impl Strategy for AmountInfStrategy {
	fn init(&mut self) {
		self.candidates = self.all_values.clone();
		self.is_first = true;
	}

	fn make_guess(&mut self) -> Option<&str> {
		if self.is_first {
			self.is_first = false;
			self.last_guess = self.candidates[0].clone();
		} else {
			match self.candidates.len() {
				0 => return None,
				1 => self.last_guess = self.candidates[0].clone(),
				_ => {
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
			}
		}
		Some(&self.last_guess)
	}

	fn respond_to_guess(&mut self, bulls: i32, cows: i32) {
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
