mod game_utils;
mod strategy;

use std::{
	env::args,
	mem,
	num::NonZero,
	sync::{mpsc::channel, Arc, Mutex},
};

use game_utils::Number;
use strategy::{create_strategy, StrategyType};

#[derive(Debug, Default)]
struct EvaluationResult {
	total: i32,
	worst_guess_count: i32,
	avg: f64,
	worst_number: Number,
	time: std::time::Duration,
}

fn evaluate_strategy_one_thread(
	strategy: &mut dyn strategy::Strategy,
	vals: Arc<Mutex<Vec<Number>>>,
	game: game_utils::GameParams,
) -> Result<EvaluationResult, String> {
	const CHUNK_SIZE: usize = 20;

	let mut total = 0;
	let mut worst_guess_count = 0;
	let mut worst_number = Number::default();

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
						if guess == &x {
							break;
						}
						counter += 1;
						if counter > 30 {
							return Err(format!(
								"Probably it is an infinite loop. Problem number: {x}"
							));
						}
						let (b, c) = game.calc_bc(guess, &x);
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
	game: &game_utils::GameParams,
) -> Result<EvaluationResult, String> {
	let start_time = std::time::Instant::now();

	let vals: Vec<_> = game_utils::get_numbers_iter(game).collect();
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
		let g = game.clone();
		join_handles.push(std::thread::spawn(move || {
			let res = evaluate_strategy_one_thread(strategy.as_mut(), vals, g);
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

// Run the strategy to guess the given hidden number, print the all steps
fn solve_for_one_number(
	a: &mut dyn strategy::Strategy,
	hidden_number: Number,
	g: &game_utils::GameParams,
) {
	a.init();
	let mut counter = 1;
	loop {
		if let Some(guess) = a.make_guess() {
			let (bulls, cows) = g.calc_bc(guess, &hidden_number);
			println!(
				"Guess #{:?}: {:}, answer is {:} bulls, {:} cows",
				counter, guess, bulls, cows
			);
			if bulls == g.number_len {
				break;
			}
			a.respond_to_guess(bulls, cows);
		} else {
			println!("Answers are inconsistent. Something wrong with the strategy");
			break;
		}
		counter += 1;
	}
}

enum GameMode {
	Solve,
	Print,
	Analyze,
	Help,
}

enum ParseStrategyTypeError {
	NotPresented,
	UnknownStrategy(String),
}

fn parse_strategy_type() -> Result<StrategyType, ParseStrategyTypeError> {
	let mut need_parse_strategy = false;
	for s in args() {
		if need_parse_strategy {
			for (n, st) in [
				("naive", StrategyType::Naive),
				("amountinformation", StrategyType::AmountInformation),
				("landy", StrategyType::Landy),
				("minavg", StrategyType::MinAvg),
				("minmax", StrategyType::MinMax),
			] {
				if s.to_lowercase() == n {
					return Ok(st);
				}
			}
			return Err(ParseStrategyTypeError::UnknownStrategy(s));
		}
		if s == "-st" || s == "--strategy_type" {
			need_parse_strategy = true;
			continue;
		}
	}

	Err(ParseStrategyTypeError::NotPresented)
}

fn parse_game_mode() -> GameMode {
	if let Some(arg) = args().nth(1) {
		if arg == "solve" {
			return GameMode::Solve;
		} else if arg == "print" {
			return GameMode::Print;
		} else if arg == "analyze" {
			return GameMode::Analyze;
		}
	}
	return GameMode::Help;
}

fn main() {
	let game_params;
	match game_utils::GameParams::new_from_args() {
		Ok(g) => game_params = g,
		Err(e) => {
			game_params = e.game_params;
			println!("{:}", e.error_string);
			let with_rep_s = if game_params.with_reps {
				"with repetitions"
			} else {
				"without repetitions"
			};
			println!(
				"Actual game params: base = {:}, number_len = {:}, {with_rep_s}",
				game_params.base, game_params.number_len
			);
		}
	}

	match parse_game_mode() {
		GameMode::Analyze => {
			let strategies = match parse_strategy_type() {
				Ok(st) => vec![st],

				Err(ParseStrategyTypeError::NotPresented) => vec![
					StrategyType::Naive,
					StrategyType::AmountInformation,
					StrategyType::MinMax,
					StrategyType::Landy,
					StrategyType::MinAvg,
				],

				Err(ParseStrategyTypeError::UnknownStrategy(s)) => {
					println!("Unknown strategy type: {s}");
					return;
				}
			};
			for st in strategies {
				let mut s = create_strategy(st, &game_params);

				match evaluate_strategy(s.as_mut(), &game_params) {
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
		}

		GameMode::Print => match std::env::args().position(|x| x == "-n") {
			Some(p) => {
				if let Some(p_n) = std::env::args().nth(p + 1) {
					if let Some(num) = game_params.to_number_checked(&p_n) {
						let st;
						match parse_strategy_type() {
							Ok(s) => st = s,
							Err(_) => {
								st = StrategyType::Naive;
								println!("Can't parse a strategy type. Use {:?}", st);
							}
						};

						solve_for_one_number(
							&mut *create_strategy(st, &game_params),
							num,
							&game_params,
						);
					} else {
						println!("The number {p_n} doesn't fit to the game parameters");
					}
				} else {
					println!("There is no required -n argument");
				}
			}
			None => {
				println!("There is no required -n argument");
			}
		},

		GameMode::Solve => {
			let st;
			match parse_strategy_type() {
				Ok(s) => st = s,
				Err(_) => {
					st = StrategyType::Naive;
					println!("Can't parse a strategy type. Use {:?}", st);
				}
			};

			let mut s = create_strategy(st, &game_params);

			one_game(s.as_mut());
		}

		GameMode::Help => {
			println!("This is a help message :)");
		}
	}
}
