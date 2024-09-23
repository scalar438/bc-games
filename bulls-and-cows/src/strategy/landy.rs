use super::TargetFunc;
use std::sync::Arc;

#[derive(Clone)]
pub struct LandyFunc {
	inv_values: Arc<Vec<f64>>,
}

impl TargetFunc for LandyFunc {
	type EvaluationResult = f64;

	fn new(_: i32) -> Self {
		let l = 10000;
		Self {
			inv_values: Arc::new(
				std::iter::once(0.0)
					.chain((1..=l).map(|x| calc_inv(x as f64)))
					.collect(),
			),
		}
	}

	fn evaluate_distribution(&self, distribution: &[i32], _: i32) -> Self::EvaluationResult {
		distribution
			.iter()
			.map(|x| self.inv_values[*x as usize] * (*x as f64))
			.sum()
	}

	fn get_initial_value(&mut self) -> Self::EvaluationResult {
		1e100
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
