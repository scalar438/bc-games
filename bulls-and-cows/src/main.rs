use strategy::{create_strategy, StrategyType};

use std::time;

mod common;
mod strategy;

#[derive(Debug)]
struct EvaluationResult {
	total: i32,
	worst_attempt: i32,
	avg: f64,
	worst_number: String,
	time: std::time::Duration,
}

fn evaluate_strategy(a: &mut dyn strategy::Strategy, n: i32) -> Option<EvaluationResult> {
	let start_time = std::time::Instant::now();

	let mut total = 0;
	let mut worst_attempt = 0;
	let vals = common::gen_values(n);
	let numbers_count = vals.len() as f64;
	let mut worst_number = String::new();
	for x in vals {
		a.init();

		let mut counter = 1;
		loop {
			total += 1;
			let guess = a.make_guess();
			match guess {
				Some(guess) => {
					if guess == x {
						break;
					}
					counter += 1;
					if counter > 30 {
						println!(
							"It seems strategy felt into an infinite loop. Problem number: {:?}",
							x
						);
						return None;
					}
					let bc = common::calc_bc(&guess, &x);
					a.respond_to_guess(bc.0, bc.1);
				}
				None => {
					println!(
						"Strategy isn't able to handle all numbers! Problem number: {:?}",
						x
					);
					return None;
				}
			}
		}
		if worst_attempt < counter {
			worst_attempt = counter;
			worst_number = x;
		}
	}

	Some(EvaluationResult {
		total,
		worst_attempt,
		avg: (total as f64) / numbers_count,
		worst_number,
		time: std::time::Instant::now() - start_time,
	})
}

fn one_game(a: &mut dyn strategy::Strategy) {
	a.init();
	let mut counter = 1;
	loop {
		if let Some(guess) = a.make_guess() {
			println!("Guess #{:?}: {:}", counter, guess);
		} else {
			println!("Answers are inconsistent");
			break;
		}
		counter += 1;
		let mut s = String::new();
		std::io::stdin().read_line(&mut s).unwrap();
		let v: Vec<_> = s.split(' ').map(|x| x.trim().parse().unwrap()).collect();
		if v.len() == 2 {
			a.respond_to_guess(v[0], v[1]);
		}
	}
}

fn main() {
	const N: i32 = 3;

	if std::env::args().position(|x| x == "--analyze").is_some() {
		for st in [
			StrategyType::Naive,
			StrategyType::AmountInformation,
			StrategyType::MinMax,
			StrategyType::Landy,
		] {
			let mut s = create_strategy(st, N);

			println!(
				"Strategy type: {:?}, evaluation result: {:?}",
				st,
				evaluate_strategy(s.as_mut(), N)
			);
		}
	} else {
		let mut s = create_strategy(StrategyType::Naive, N);

		one_game(s.as_mut());
	}
}
