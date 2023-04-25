use super::colored_string::{Color, ColoredString};
use super::words_chooser::CharResult;
use std::io::Write;

pub enum Command {
	StopGame, // Stop the current game, start the new one
	Quit,     // Stop the current game and quit
}

pub enum Input<T> {
	Value(T),
	Cmd(Command),
}

pub type InputResult<T> = std::io::Result<Input<T>>;

pub struct InputGetter {
	word_len: usize,
}

fn is_accepted(colored_str: &ColoredString) -> std::io::Result<bool> {
	println!("Your answer is {0}?", colored_str);
	dialoguer::Confirm::new().interact()
}

impl InputGetter {
	pub fn new(word_len: usize) -> InputGetter {
		InputGetter { word_len }
	}

	pub fn get_word<T: AsRef<str>>(&self, msg: T) -> InputResult<String> {
		loop {
			print!("{0}", msg.as_ref());
			std::io::stdout().flush()?;
			let r = Self::get_str_or_command();

			if let Ok(Input::Value(s)) = &r {
				if s.chars().count() != self.word_len {
					println!(
						"Invalid length of word. In this game you have to use words contain {} symbols",
						self.word_len
					);

					continue;
				}
			}

			return r;
		}
	}

	pub fn get_response_vector<TA: AsRef<str>, TM: AsRef<str>>(
		&self,
		attempt_word: Option<TA>,
		msg: TM,
	) -> InputResult<Vec<CharResult>> {
		if let Some(x) = &attempt_word {
			if x.as_ref().chars().count() != self.word_len {
				panic!("Length of attempt_word should be equal to self.word_len!");
			}
		}
		loop {
			print!("{0}", msg.as_ref());
			std::io::stdout().flush()?;

			let r = Self::get_str_or_command()?;

			match r {
				Input::Value(s) => {
					if s.chars().count() != self.word_len {
						println!(
							"Invalid length of word. In this game you have to use words contain {} symbols", 
							self.word_len
						);
						continue;
					}

					if s.chars().any(|x| x != '0' && x != '1' && x != '2') {
						println!("This string contains forbidden symbol(s). Use can use just 0, 1 and 2 as answer");
						continue;
					}

					let vec_res: Vec<_> = s
						.chars()
						.map(|x| match x {
							'0' => CharResult::NotPresented,
							'1' => CharResult::PartialMatch,
							'2' => CharResult::FullMatch,
							_ => unreachable!("Unexpected symbol in the string: {}", s),
						})
						.collect();

					if let Some(attempt_word) = &attempt_word {
						let mut colored_string =
							ColoredString::new(attempt_word.as_ref().to_owned());
						for (i, c) in vec_res.iter().enumerate() {
							match c {
								CharResult::FullMatch => colored_string.set_color(i, Color::Green),
								CharResult::PartialMatch => {
									colored_string.set_color(i, Color::Yellow)
								}
								CharResult::NotPresented => {
									colored_string.set_color(i, Color::Gray)
								}
							}
						}
						if is_accepted(&colored_string)? {
							return Ok(Input::Value(vec_res));
						} else {
							continue;
						}
					}
				}
				Input::Cmd(cmd) => {
					return Ok(Input::Cmd(cmd));
				}
			}
		}
	}

	fn get_str_or_command() -> InputResult<String> {
		let mut s = String::new();

		std::io::stdin().read_line(&mut s)?;
		let s = s.trim();

		if s == "-quit" {
			Ok(Input::Cmd(Command::Quit))
		} else if s == "-stop" {
			Ok(Input::Cmd(Command::StopGame))
		} else {
			Ok(Input::Value(s.trim().to_owned().to_lowercase()))
		}
	}
}
