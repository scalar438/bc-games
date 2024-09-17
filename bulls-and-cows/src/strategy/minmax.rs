use super::TargetFunc;

#[derive(Clone)]
pub struct MinMaxFunc {}

impl TargetFunc for MinMaxFunc {
	type EvaluationResult = Vec<i32>;

	fn new(_: i32) -> Self {
		Self {}
	}

	fn evaluate_distribution(&self, distribution: &[i32], _: i32) -> Self::EvaluationResult {
		let mut res: Vec<_> = distribution.iter().map(|x| *x).collect();
		res.sort();
		res.reverse();
		res
	}

	fn get_initial_value(&mut self) -> Self::EvaluationResult {
		vec![i32::max_value()]
	}
}
