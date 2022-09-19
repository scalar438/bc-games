use std::env;
pub mod comb_utils;
mod db_reader;
mod words_chooser;

fn get_word_len() -> Option<usize> {
	let mut word_len_arg = false;
	for arg in env::args() {
		if arg == "-wl" {
			word_len_arg = true;
		} else {
			if word_len_arg {
				let tmp_res = arg.trim().parse::<usize>();
				if let Ok(res) = tmp_res {
					if res >= 4 && res <= 8 {
						return Some(res);
					}
				}
				word_len_arg = false;
			}
		}
	}
	return None;
}

// Return true if you want to continue
fn one_game(
	db: &mut db_reader::WordsDb,
	strategy: &mut words_chooser::WordsChooser,
	word_len: usize,
) -> bool {
	let mut game_over = false;
	loop {
		// Type or get a word as a new attempt
		match strategy.make_guess() {
			Some(x) => {
				println!("My attempt: {}", x);
			}
			None => {
				if !game_over {
					println!("My game is over. Finish the game instead of me");
					game_over = true;
				}

				let user_attempt = loop {
					print!("Your attempt: ");
					let mut s = String::new();
					std::io::stdin().read_line(&mut s).unwrap();
					let s = s.trim();
					if s == "-quit" {
						return false;
					}
					if s == "-stop" {
						return true;
					}
					if s.len() != word_len {
						println!("Invalid length of word. In this game you have to use words contain {} symbols", word_len);
					} else {
						break s.to_owned();
					}
				};
				strategy.add_word_to_list(&user_attempt);
				db.add_word(&user_attempt);
			}
		}

		// Make the answer, if it was a bot attempt
		if !game_over {
			let answer_vec: Vec<_> = loop {
				print!("Result of that attempt: ");
				let mut s = String::new();
				std::io::stdin().read_line(&mut s).unwrap();
				let s = s.trim();
				if s == "-quit" {
					return false;
				}
				if s == "-stop" {
					return true;
				}
				if s.len() != word_len {
					println!("Invalid length of word. In this game you have to use words contain {} symbols", word_len);
					continue;
				}
				if s.chars().any(|x| x != '0' || x != '1' || x != '2') {
					print!("The string contains forbidden symbol. Use can use just 0, 1 and 2 as answer");
					continue;
				}
				break s
					.chars()
					.map(|x| match x {
						'0' => words_chooser::CharResult::NotPresented,
						'1' => words_chooser::CharResult::PartialMatch,
						'2' => words_chooser::CharResult::FullMatch,
						_ => unreachable!("Unexpected symbol in the string: {}", s),
					})
					.collect();
			};
			strategy.respond_to_guess(&answer_vec);
		}
	}
}

fn main() {
	let q = vec![1, 2, 3, 4];

	for i in &q[0..2] {}

	/*let word_len;
	match get_word_len() {
		Some(w) => word_len = w,
		None => {
			println!("The length of words is not set. Call the program with -wl <len> arguments");
			return;
		}
	}
	let mut db = db_reader::WordsDb::new(std::path::Path::new("./words_db.txt")).unwrap();
	let mut strategy = words_chooser::WordsChooser::new(word_len, &mut db.words_iter());
	loop {
		strategy.init();
		if !one_game(&mut db, &mut strategy, word_len) {
			break;
		}
	}*/
}
