use super::common;
use super::Strategy;
use std::sync::Arc;

struct StaticData {
	n: i32,
	inv_values: Vec<f64>,
	all_values: Vec<String>,
}

#[derive(Clone)]
pub struct LandyStrategy {
	candidates: Vec<String>,
	last_guess: String,
	static_data: Arc<StaticData>,
	is_first: bool,
}

impl LandyStrategy {
	// The target function of this strategy. Pick the number with lowest value of it.
	fn evaluate_attempt(&self, attempt: &str) -> f64 {
		let mut v = [0; 25];
		for ans in self.candidates.iter() {
			let bc = common::calc_bc(attempt, ans);
			v[(bc.0 * self.static_data.n + bc.1) as usize] += 1;
		}
		v.iter()
			.filter_map(|x| {
				if *x != 0 {
					Some(self.static_data.inv_values[*x] * (*x as f64))
				} else {
					None
				}
			})
			.sum()
	}

	pub fn new(n: i32) -> Self {
		let mut all_values = common::gen_values(n);
		all_values.sort();
		let l = all_values.len();
		let s = StaticData {
			n,
			inv_values: std::iter::once(0.0)
				.chain((1..=l).map(|x| calc_inv(x as f64)))
				.collect(),
			all_values,
		};
		LandyStrategy {
			candidates: Vec::new(),
			static_data: Arc::new(s),
			last_guess: String::new(),
			is_first: true,
		}
	}
}

// Inversion of the function x^x. The answer is calculated by binary search
fn calc_inv(n: f64) -> f64 {
	let mut x = 1.0;
	let mut y = 1.0;
	for i in 1.. {
		let f = i as f64;
		let z = f.powf(f);
		if z > n {
			y = f;
			break;
		}
	}
	loop {
		let z = (x + y) / 2.0;
		if z <= x || z >= y {
			break;
		}
		if z.powf(z) <= n {
			x = z
		} else {
			y = z
		};
	}
	x
}

impl Strategy for LandyStrategy {
	fn init(&mut self) {
		self.candidates = self.static_data.all_values.clone();
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
					let mut hs = std::collections::HashSet::new();
					let mut min_value = (self.candidates.len() * self.candidates.len()) as f64;
					for attempt in self.candidates.iter() {
						let value = self.evaluate_attempt(attempt);
						if min_value > value {
							min_value = value;
							self.last_guess = attempt.clone();
						}
						hs.insert(attempt);
					}
					for attempt in self.static_data.all_values.iter() {
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
