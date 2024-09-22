use std::{
	mem,
	num::NonZero,
	sync::{mpsc::channel, Arc, Mutex},
};

use game_utils::get_numbers_iter;
use strategy::{create_strategy, StrategyType};

mod common;
mod game_utils;
mod strategy;

#[derive(Debug, Default)]
struct EvaluationResult {
	total: i32,
	worst_guess_count: i32,
	avg: f64,
	worst_number: String,
	time: std::time::Duration,
}

fn evaluate_strategy_one_thread(
	strategy: &mut dyn strategy::Strategy,
	vals: Arc<Mutex<Vec<String>>>,
) -> Result<EvaluationResult, String> {
	const CHUNK_SIZE: usize = 20;

	let mut total = 0;
	let mut worst_guess_count = 0;
	let mut worst_number = String::new();

	loop {
		let cur_values;
		{
			let mut vals = vals.lock().unwrap();
			let n = vals.len();
			if n < CHUNK_SIZE {
				if n == 0 {
					break;
				}
				cur_values = mem::replace(vals.as_mut(), Vec::new());
			} else {
				cur_values = vals.drain(n - CHUNK_SIZE..n).collect();
			}
		}

		for x in cur_values {
			strategy.init();

			let mut counter = 1;
			loop {
				total += 1;
				let guess = strategy.make_guess();

				match guess {
					Some(guess) => {
						if guess == x {
							break;
						}
						counter += 1;
						if counter > 30 {
							return Err(format!(
								"Probably it is an infinite loop. Problem number: {x}"
							));
						}
						let (b, c) = common::calc_bc(guess, &x);
						strategy.respond_to_guess(b, c);
					}

					None => {
						return Err(format!("The strategy returned None. Problem number: {x}"));
					}
				}
			}
			if counter > worst_guess_count {
				worst_guess_count = counter;
				worst_number = x;
			}
		}
	}

	Ok(EvaluationResult {
		total,
		worst_guess_count,
		worst_number,
		avg: 0.0,
		time: Default::default(),
	})
}

fn evaluate_strategy(
	strategy: &mut dyn strategy::Strategy,
	game_params: &game_utils::GameParams,
) -> Result<EvaluationResult, String> {
	let start_time = std::time::Instant::now();

	let vals: Vec<_> = get_numbers_iter(game_params)
		.map(|x| x.to_string())
		.collect();
	let numbers_count = vals.len() as f64;

	let vals = Mutex::new(vals);
	let vals = Arc::new(vals);

	let (sx, rx) = channel();

	let mut join_handles = Vec::new();
	for _ in 0..std::thread::available_parallelism()
		.unwrap_or(NonZero::new(1).unwrap())
		.get()
	{
		let sx = sx.clone();
		let vals = vals.clone();
		let mut strategy = strategy.clone_strategy();

		join_handles.push(std::thread::spawn(move || {
			let res = evaluate_strategy_one_thread(strategy.as_mut(), vals);
			sx.send(res).unwrap();
		}));
	}
	drop(sx);
	let mut res = EvaluationResult::default();
	for res_partial in rx {
		match res_partial {
			Ok(res_partial) => {
				res.total += res_partial.total;
				if res.worst_guess_count < res_partial.worst_guess_count {
					res.worst_guess_count = res_partial.worst_guess_count;
					res.worst_number = res_partial.worst_number;
				}
			}
			Err(err) => {
				vals.lock().unwrap().clear();
				join_handles.into_iter().for_each(|x| x.join().unwrap());
				return Err(err);
			}
		}
	}
	join_handles.into_iter().for_each(|x| x.join().unwrap());

	res.avg = res.total as f64 / numbers_count;
	res.time = std::time::Instant::now() - start_time;
	Ok(res)
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
	const N: u8 = 4;

	let g = game_utils::GameParams::new(N as u8);

	if std::env::args().position(|x| x == "--analyze").is_some() {
		for st in [
			StrategyType::Naive,
			StrategyType::AmountInformation,
			StrategyType::MinMax,
			StrategyType::Landy,
			StrategyType::MinAvg,
		] {
			let mut s = create_strategy(st, &g);

			match evaluate_strategy(s.as_mut(), &g) {
				Ok(res) => {
					println!("Strategy type: {:?}, check successfull. Results", st);
					println!(
						"Total number of guesses {:}, average {:}",
						res.total, res.avg
					);
					println!(
						"Worst number {:} guessed with {:} attempts",
						res.worst_number, res.worst_guess_count
					);
					println!("Total time: {:?}\n", res.time);
				}
				Err(s) => println!(
					"Strategy type: {:?} isn't able to solve the puzzle. Error message: {:}",
					st, s
				),
			}
		}
	} else {
		let mut s = create_strategy(StrategyType::MinMax, &g);

		one_game(s.as_mut());
	}
}
