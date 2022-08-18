#[derive(Debug, Clone, Copy)]
pub enum CharResult {
	NotPresented,
	NotHere,
	Here,
}

pub struct WordsChooser {}

impl WordsChooser {
	pub fn new<T: AsRef<str>>(
		word_len: usize,
		all_words: &dyn Iterator<Item = &T>,
	) -> WordsChooser {
		WordsChooser {}
	}

	pub fn init(&mut self) {
		unimplemented!()
	}
	pub fn make_guess(&self) -> Option<&str> {
		unimplemented!()
	}
	pub fn respond_to_guess(&mut self, respond: &[CharResult]) {
		unimplemented!()
	}
	pub fn add_word_to_list<T: AsRef<str>>(&mut self, word: T) {
		unimplemented!()
	}
}
