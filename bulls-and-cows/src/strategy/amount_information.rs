use super::TargetFunc;

#[derive(Clone)]
pub struct AmountInfFunc {}

impl TargetFunc for AmountInfFunc {
	type EvaluationResult = f64;

	fn new(_: i32) -> Self {
		Self {}
	}

	fn evaluate_distribution(
		&self,
		distribution: &[i32],
		current_candidates: i32,
	) -> Self::EvaluationResult {
		let s = current_candidates as f64;

		distribution
			.iter()
			.filter_map(|x| {
				if *x != 0 {
					let p = (*x as f64) / s;
					Some(p * f64::ln(p))
				} else {
					None
				}
			})
			.sum()
	}

	fn get_initial_value(&mut self) -> Self::EvaluationResult {
		1.0
	}
}
