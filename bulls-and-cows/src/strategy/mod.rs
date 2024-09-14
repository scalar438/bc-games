use super::common;

mod amount_information;
mod landy;
mod minmax;
mod naive;

pub trait Strategy {
	// Init the strategy. After this call the object is ready to start a new game
	fn init(&mut self);

	// Make a guess. None means responses were inconsistent
	fn make_guess(&mut self) -> Option<&str>;

	fn respond_to_guess(&mut self, bulls: i32, cows: i32);

	fn _clone_dyn(&self) -> Box<dyn Strategy>;
}

#[derive(Debug, Clone, Copy)]
pub enum StrategyType {
	// The fastest and simplest, but not the most efficient algorithm
	// Just picks the first number from list of candidates, without any strategy
	Naive,

	// Strategy that tries to maximize the average amount of information gotten by the attempt
	AmountInformation,

	// Strategy that tries to minimize the worst case. It isn't the best on average
	MinMax,

	// Strategy that uses Landy's formula for picking the attempt
	Landy,
}

pub fn create_strategy(t: StrategyType, n: i32) -> Box<dyn Strategy> {
	match t {
		StrategyType::Naive => Box::new(naive::NaiveStrategy::new(n)),
		StrategyType::AmountInformation => Box::new(amount_information::AmountInfStrategy::new(n)),
		StrategyType::MinMax => Box::new(minmax::MinMaxStrategy::new(n)),
		StrategyType::Landy => Box::new(landy::LandyStrategy::new(n)),
	}
}
