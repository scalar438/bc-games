use std::env;
mod db_reader;
mod input_getter;
mod colored_string;
mod words_chooser;
use crate::words_chooser::CharResult;
use input_getter::{Command, Input, InputGetter};

enum BotRunResult {
	BotLost,
	BotWon,
	ExitByCommand(Command),
}

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

fn bot_game(
	strategy: &mut words_chooser::WordsChooser,
	input_getter: &input_getter::InputGetter,
) -> std::io::Result<BotRunResult> {
	loop {
		let cur_word;
		match strategy.make_guess() {
			Some(word) => {
				println!("Bot's attempt: {}", word);
				cur_word = word.to_owned();
			}
			None => {
				return Ok(BotRunResult::BotLost);
			}
		}

		let ans = input_getter.get_response_vector(Some(&cur_word), "Type answer to attempt: ")?;

		match ans {
			Input::Value(v) => {
				if v.iter().all(|x| x == &CharResult::FullMatch) {
					return Ok(BotRunResult::BotWon);
				}
				strategy.respond_to_guess(&v);
			}
			Input::Cmd(c) => return Ok(BotRunResult::ExitByCommand(c)),
		}
	}
}

// Return true if you want to continue
fn one_game(
	db: &mut db_reader::WordsDb,
	strategy: &mut words_chooser::WordsChooser,
	word_len: usize,
) -> std::io::Result<bool> {
	let input_getter = InputGetter::new(word_len);
	let bot_result = bot_game(strategy, &input_getter)?;

	match bot_result {
		BotRunResult::ExitByCommand(Command::Quit) => return Ok(false),
		BotRunResult::ExitByCommand(Command::StopGame) => return Ok(true),
		BotRunResult::BotWon => {
			println!("Bot won. Run the next game");
			return Ok(true);
		}
		BotRunResult::BotLost => {
			println!("Bot lost. Finish the game for bot. Next time it will know more words");
		}
	}

	loop {
		match input_getter.get_word("Your attempt: ")? {
			Input::Cmd(c) => match c {
				Command::Quit => return Ok(false),
				Command::StopGame => return Ok(true),
			},

			Input::Value(s) => {
				db.add_word(&s);
			}
		}
	}
}

fn main() {
	let word_len;
	match get_word_len() {
		Some(w) => word_len = w,
		None => {
			println!("The length of words is not set. Call the program with -wl <len> arguments. Len should be between 4 and 8");
			return;
		}
	}
	let mut db = db_reader::WordsDb::new(std::path::Path::new("./words_db.txt"), word_len).unwrap();
	loop {
		let mut strategy = words_chooser::WordsChooser::new(&mut db.words_iter());

		if !one_game(&mut db, &mut strategy, word_len).unwrap() {
			break;
		}
		if let Err(e) = db.flush() {
			println!("Unextected error: {0}", e);
			return;
		}
	}
}
