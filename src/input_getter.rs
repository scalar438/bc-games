use super::words_chooser::CharResult;

enum Command {
	StopGame, // Stop the current game, starn new
	Quit,     // Stop the current game and quit
}

enum Input<T> {
	Value(T),
	Cmd(Command),
}

pub type InputResult<T> = std::io::Result<Input<T>>;

pub struct InputGetter {
	word_len: usize,
}

impl InputGetter {
	pub fn new(word_len: usize) -> InputGetter {
		InputGetter { word_len }
	}

	pub fn get_word(&self) -> InputResult<String> {
		loop {
			let r = Self::get_str_or_command();

			if let Ok(Input::Value(s)) = &r {
				if s.len() != self.word_len {
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

	pub fn get_response_vector(&self) -> InputResult<Vec<CharResult>> {
		loop {
			let r = Self::get_str_or_command()?;

			match r {
				Input::Value(s) => {
					if s.len() != self.word_len {
						println!(
						"Invalid length of word. In this game you have to use words contain {} symbols",
						self.word_len
					);
						continue;
					}

					if s.chars().any(|x| x != '0' || x != '1' || x != '2') {
						print!("This string contains forbidden symbol. Use can use just 0, 1 and 2 as answer");
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

					return Ok(Input::Value(vec_res));
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

		if s == "-quit" {
			Ok(Input::Cmd(Command::Quit))
		} else if s == "-stop" {
			Ok(Input::Cmd(Command::StopGame))
		} else {
			Ok(Input::Value(s))
		}
	}
}
