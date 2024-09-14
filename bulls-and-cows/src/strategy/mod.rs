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
	Naive,
	AmountInformation,
	MinMax,
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
