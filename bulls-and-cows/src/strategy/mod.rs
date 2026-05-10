use super::game_utils;
use super::game_utils::Number;

mod amount_information;
mod landy;
mod min_avg;
mod minmax;
mod naive;

pub trait Strategy: Send {
	// Initialize the strategy. After this call, the object is ready to start a new game
	fn init(&mut self);

	// Make a guess. None means the responses were inconsistent
	fn make_guess(&mut self) -> Option<&Number>;

	fn respond_to_guess(&mut self, bulls: u8, cows: u8);

	fn clone_strategy(&self) -> Box<dyn Strategy>;
}

trait TargetFunc: Clone + Send {
	type EvaluationResult;

	fn new(g: &super::game_utils::GameParams) -> Self;

	fn evaluate_distribution(
		&self,
		distribution: &[i32],
		current_candidates: i32,
	) -> Self::EvaluationResult;

	// A value of type EvaluationResult that is greater than any value returned by evaluate_distribution
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
	all_values: Vec<Number>,
	candidates: Vec<Number>,
	is_first: bool,
	last_guess: Number,
	n: u8,
	target_func: F,
	with_repetitions: bool,

	distribution_buf: std::cell::RefCell<Vec<i32>>,
}

impl<F: TargetFunc> BasicStrategy<F>
where
	F::EvaluationResult: PartialOrd,
{
	fn new(g: &game_utils::GameParams) -> BasicStrategy<F> {
		let all_values: Vec<_> = game_utils::get_numbers_iter(&g).collect();
		let n = g.number_len as u16;
		BasicStrategy {
			all_values,
			candidates: Vec::new(),
			is_first: false,
			last_guess: Number::default(),
			n: (n + 1) as u8,
			with_repetitions: g.with_reps,
			target_func: F::new(&g),
			distribution_buf: std::cell::RefCell::new(vec![0; ((n + 1) * (n + 1)) as usize]),
		}
	}

	fn evaluate_attempt(&self, attempt: &Number) -> F::EvaluationResult {
		let mut distribution = self.distribution_buf.borrow_mut();
		distribution.iter_mut().for_each(|x| *x = 0);

		for ans in self.candidates.iter() {
			let bc = game_utils::calc_bc_with_base(&attempt, &ans, 10);
			distribution[(bc.0 * self.n + bc.1) as usize] += 1;
		}
		let v: Vec<_> = distribution
			.iter()
			.filter_map(|x| if *x != 0 { Some(*x) } else { None })
			.collect();
		self.target_func
			.evaluate_distribution(&v[..], self.candidates.len() as i32)
	}
}

impl<F: TargetFunc + 'static> Strategy for BasicStrategy<F>
where
	F::EvaluationResult: PartialOrd + core::fmt::Debug,
{
	fn init(&mut self) {
		// If repetitions are allowed, we can't pick any number as first attempt (is_first must be false)
		self.is_first = !self.with_repetitions;
		self.candidates = self.all_values.clone();
	}

	fn make_guess(&mut self) -> Option<&Number> {
		if self.is_first {
			self.is_first = false;
			self.last_guess = self.candidates[0].clone()
		} else {
			match self.candidates.len() {
				0 => return None,
				1 => self.last_guess = self.candidates[0].clone(),
				_ => {
					let mut min_value = self.target_func.get_initial_value();
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
			let bc = game_utils::calc_bc_with_base(&self.last_guess, &x, 10);
			bc.0 == bulls && bc.1 == cows
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

	// Strategy that tries to maximize the average amount of information obtained by the attempt
	AmountInformation,

	// Strategy that tries to minimize the worst case. It isn't the best on average
	MinMax,

	// Strategy that uses Landy's formula (see the implementation) for picking an attempt
	Landy,

	// Strategy that tries to minimize the average number of candidates left on the next step
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
