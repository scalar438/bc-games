use crate::game_utils::get_numbers_iter;

use super::common;
use super::game_utils;

mod amount_information;
mod landy;
mod min_avg;
mod minmax;
mod naive;

pub trait Strategy: Send {
	// Init the strategy. After this call the object is ready to start a new game
	fn init(&mut self);

	// Make a guess. None means responses were inconsistent
	fn make_guess(&mut self) -> Option<&str>;

	fn respond_to_guess(&mut self, bulls: u8, cows: u8);

	fn clone_strategy(&self) -> Box<dyn Strategy>;
}

trait TargetFunc: Clone + Send {
	type EvaluationResult;

	fn new(n: i32) -> Self;

	fn evaluate_distribution(
		&self,
		distribution: &[i32],
		current_candidates: i32,
	) -> Self::EvaluationResult;

	fn get_initial_value(&mut self) -> Self::EvaluationResult;
}

// The basic code that can be used in almost every strategy
// It builds a list of all possible candidates, evaluates each candidate,
// picks the one that minimizes the target function F
// and removes candidates that don't satisfy the condition
#[derive(Clone)]
struct BasicStrategy<F: TargetFunc>
where
	F::EvaluationResult: PartialOrd,
{
	all_values: Vec<String>,
	candidates: Vec<String>,
	is_first: bool,
	last_guess: String,
	n: u8,
	func: F,
}

impl<F: TargetFunc> BasicStrategy<F>
where
	F::EvaluationResult: PartialOrd,
{
	fn new(g: &game_utils::GameParams) -> BasicStrategy<F> {
		let all_values = get_numbers_iter(&g).map(|x| x.to_string()).collect();
		let n = g.number_len();
		BasicStrategy {
			all_values,
			candidates: Vec::new(),
			is_first: false,
			last_guess: String::new(),
			n: n + 1,
			func: F::new(n as i32),
		}
	}

	fn evaluate_attempt(&self, attempt: &str) -> F::EvaluationResult {
		let mut v = [0; 25];
		let attempt = game_utils::Number::from(attempt);
		for ans in self.candidates.iter() {
			let bc = game_utils::calc_bc_with_size(&attempt, &game_utils::Number::from(ans), 10);
			v[(bc.0 * self.n + bc.1) as usize] += 1;
		}
		let v: Vec<_> = v
			.iter()
			.filter_map(|x| if *x != 0 { Some(*x) } else { None })
			.collect();
		self.func
			.evaluate_distribution(&v[..], self.candidates.len() as i32)
	}
}

impl<F: TargetFunc + 'static> Strategy for BasicStrategy<F>
where
	F::EvaluationResult: PartialOrd + core::fmt::Debug,
{
	fn init(&mut self) {
		self.is_first = true;
		self.candidates = self.all_values.clone();
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
					let mut min_value = self.func.get_initial_value();
					let mut res = &self.all_values[0];

					let mut hs = std::collections::HashSet::new();
					for attempt in self.candidates.iter() {
						let new_value = self.evaluate_attempt(attempt);
						if min_value > new_value {
							min_value = new_value;
							res = attempt;
						}
						hs.insert(attempt);
					}
					for attempt in self.all_values.iter() {
						if hs.contains(attempt) {
							continue;
						}
						let new_value = self.evaluate_attempt(attempt);
						if min_value > new_value {
							min_value = new_value;
							res = attempt;
						}
					}

					self.last_guess = res.clone();
				}
			}
		}
		Some(&self.last_guess)
	}

	fn respond_to_guess(&mut self, bulls: u8, cows: u8) {
		self.candidates.retain(|x| {
			let bc = common::calc_bc(&self.last_guess, x);
			bc.0 == bulls as i32 && bc.1 == cows as i32
		});
	}

	fn clone_strategy(&self) -> Box<dyn Strategy> {
		Box::new(self.clone())
	}
}

#[derive(Debug, Clone, Copy)]
pub enum StrategyType {
	// The fastest and simplest, but not the most efficient algorithm
	// Just picks the first number from the list of candidates, without any strategy
	Naive,

	// Strategy that tries to maximize the average amount of information gotten by the attempt
	AmountInformation,

	// Strategy that tries to minimize the worst case. It isn't the best on average
	MinMax,

	// Strategy that uses Landy's formula for picking an attempt
	Landy,

	// Strategy that tries to minimize the average candidates count on the next step
	MinAvg,
}

pub fn create_strategy(t: StrategyType, g: &game_utils::GameParams) -> Box<dyn Strategy> {
	match t {
		StrategyType::Naive => Box::new(naive::NaiveStrategy::new(*g)),
		StrategyType::AmountInformation => {
			Box::new(BasicStrategy::<amount_information::AmountInfFunc>::new(g))
		}
		StrategyType::MinMax => Box::new(BasicStrategy::<minmax::MinMaxFunc>::new(g)),
		StrategyType::Landy => Box::new(BasicStrategy::<landy::LandyFunc>::new(g)),
		StrategyType::MinAvg => Box::new(BasicStrategy::<min_avg::MinAvgFunc>::new(g)),
	}
}
