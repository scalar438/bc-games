use super::TargetFunc;

#[derive(Clone)]
pub struct MinAvgFunc {}

impl TargetFunc for MinAvgFunc {
	type EvaluationResult = u64;

	fn new(_: i32) -> Self {
		Self {}
	}

	fn evaluate_distribution(&self, distribution: &[i32], _: i32) -> Self::EvaluationResult {
		distribution
			.iter()
			.filter_map(|x| {
				if *x != 0 {
					let x = *x as u64;
					Some(x * x)
				} else {
					None
				}
			})
			.sum()
	}

	fn get_initial_value(&mut self) -> Self::EvaluationResult {
		u64::max_value()
	}
}
